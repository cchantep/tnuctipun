#!/bin/bash

# Script to extract and validate Rust code snippets from Markdown files
# This script dynamically discovers all .md files under git control

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository"
    exit 1
fi

# Check if Python is available
if ! command -v python3 > /dev/null 2>&1; then
    print_error "Python 3 is required but not found"
    print_error "Please install Python 3 or run this in an environment where Python 3 is available"
    exit 1
fi

# Create temporary directory for extracted snippets
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

print_info "Temporary directory: $TEMP_DIR"

# Find all .md files under git control
print_info "Discovering Markdown files under git control..."
MD_FILES=$(git ls-files '*.md' | grep -v '^target/' || true)

if [ -z "$MD_FILES" ]; then
    print_warning "No Markdown files found under git control"
    exit 0
fi

print_info "Found Markdown files:"
echo "$MD_FILES" | sed 's/^/  - /'

# Counter for snippets
TOTAL_SNIPPETS=0
VALID_SNIPPETS=0
FAILED_SNIPPETS=0

# Function to extract and test Rust code snippets from a file
check_md_file() {
    local file="$1"
    local file_snippets=0
    
    print_info "Processing: $file"
    
    # Create a safe filename for temporary files
    local safe_filename="${file//\//_}"
    safe_filename="${safe_filename//\./_}"
    
    # Extract Rust code blocks using the Python script
    local script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    if ! python3 "$script_dir/extract_rust_snippets.py" "$file" "$TEMP_DIR" "$safe_filename" > "$TEMP_DIR/extract_output.txt" 2>&1; then
        print_error "Failed to extract snippets from $file"
        cat "$TEMP_DIR/extract_output.txt"
        return
    fi
    
    # Get the count of snippets for this file
    local count_file="$TEMP_DIR/count_${safe_filename}.txt"
    
    if [ -f "$count_file" ]; then
        file_snippets=$(cat "$count_file")
        rm "$count_file"
    fi
    
    if [ "$file_snippets" -eq 0 ]; then
        print_info "  No Rust snippets found in $file"
        return
    fi
    
    print_info "  Found $file_snippets Rust snippet(s) in $file"
    TOTAL_SNIPPETS=$((TOTAL_SNIPPETS + file_snippets))
    
    # Test each snippet
    for i in $(seq 1 "$file_snippets"); do
        local snippet_file="$TEMP_DIR/${safe_filename}_snippet_${i}.rs"
        
        if [ ! -f "$snippet_file" ]; then
            print_warning "  Snippet file not found: $snippet_file"
            continue
        fi
        
        # Check if snippet is empty
        if [ ! -s "$snippet_file" ]; then
            print_warning "  Snippet $i in $file is empty, skipping"
            continue
        fi
        
        print_info "  Testing snippet $i from $file..."
        
        # Show the snippet content (first few lines)
        echo 
        echo "    Code preview:"
        echo

        head -3 "$snippet_file" | sed 's/^/      /'
        if [ "$(wc -l < "$snippet_file")" -gt 3 ]; then
            echo "      ..."
            echo
        fi
        
        # Create a test project for this snippet
        local test_dir="$TEMP_DIR/test_${safe_filename}_${i}"
        mkdir -p "$test_dir"
        
        # Create Cargo.toml
        cat > "$test_dir/Cargo.toml" << EOF
[package]
name = "doc_snippet_test"
version = "0.1.0"
edition = "2021"

[dependencies]
nessus = { path = "$(pwd)" }
bson = "2.0"
serde = { version = "1.0", features = ["derive"] }
mongodb = "2.0"
tokio = { version = "1.0", features = ["full"] }
EOF
        
        # Create src directory and copy snippet
        mkdir -p "$test_dir/src"
        
        # Wrap the snippet in a main function if it doesn't have one
        local wrapped_snippet="$test_dir/src/main.rs"
        
        # Replace struct names to avoid conflicts when testing multiple snippets
        local snippet_content
        snippet_content=$(sed "s/struct User/struct User${i}/g" "$snippet_file")
        snippet_content=$(echo "$snippet_content" | sed "s/user_fields::/user${i}_fields::/g")
        snippet_content=$(echo "$snippet_content" | sed "s/empty::<User>/empty::<User${i}>/g")
        snippet_content=$(echo "$snippet_content" | sed "s/projection::empty::<User>/projection::empty::<User${i}>/g")
        snippet_content=$(echo "$snippet_content" | sed "s/updates::empty::<User>/updates::empty::<User${i}>/g")
        
        if echo "$snippet_content" | grep -q "fn main"; then
            # Snippet already has a main function
            echo "$snippet_content" > "$wrapped_snippet"
        else
            # Wrap snippet in a main function
            cat > "$wrapped_snippet" << 'EOF'
use nessus::*;
use bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
EOF
            echo "$snippet_content" >> "$wrapped_snippet"
            cat >> "$wrapped_snippet" << 'EOF'
    
    Ok(())
}
EOF
        fi
        
        # Try to compile the snippet
        if (cd "$test_dir" && cargo check --quiet 2>/dev/null); then
            print_success "    ✓ Snippet $i from $file compiles successfully"
            VALID_SNIPPETS=$((VALID_SNIPPETS + 1))
        else
            print_error "    ✗ Snippet $i from $file failed to compile"
            print_error "    Compilation errors:"
            (cd "$test_dir" && cargo check 2>&1 | sed 's/^/      /' || true)
            FAILED_SNIPPETS=$((FAILED_SNIPPETS + 1))
        fi
    done
}

# Process each Markdown file
if [ -n "$MD_FILES" ]; then
    while IFS= read -r file; do
        if [ -f "$file" ] && [ -n "$file" ]; then
            check_md_file "$file"
        fi
    done << EOF
$MD_FILES
EOF
fi

# Print summary
echo
print_info "=== SUMMARY ==="
print_info "Total Rust snippets found: $TOTAL_SNIPPETS"
print_success "Valid snippets: $VALID_SNIPPETS"

if [ "$FAILED_SNIPPETS" -gt 0 ]; then
    print_error "Failed snippets: $FAILED_SNIPPETS"
    echo
    print_error "Some documentation snippets failed to compile!"
    print_error "Please update the failing snippets to match the current API."
    exit 1
else
    print_success "All documentation snippets are valid! 🎉"
fi
