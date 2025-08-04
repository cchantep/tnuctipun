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

# Arrays for batch processing
BATCH_SNIPPETS=()
BATCH_FILES=()
BATCH_NUMBERS=()

# Function to generate unique replacements for struct names and field references
# This dynamically discovers struct names and field modules to avoid hardcoded patterns
generate_unique_replacements() {
    local content="$1"
    local snippet_id="$2"
    local result="$content"
    
    # Find all struct definitions and make them unique
    local structs
    structs=$(echo "$content" | grep -o 'struct [A-Za-z_][A-Za-z0-9_]*' | sed 's/struct //' | sort | uniq)
    
    while IFS= read -r struct_name; do
        if [ -n "$struct_name" ]; then
            local unique_name="${struct_name}${snippet_id}"

            # Replace struct definitions
            result=${result//struct ${struct_name}/struct ${unique_name}}

            # Replace type references
            result=${result//<${struct_name}>/<${unique_name}>}
            result=${result//::<${struct_name}>/::<${unique_name}>}
            
            # Generate field module name (convert CamelCase to snake_case and add suffix)
            local field_module=$(echo "$struct_name" | sed 's/\([A-Z]\)/_\L\1/g' | sed 's/^_//' | tr '[:upper:]' '[:lower:]')
            local unique_field_module="${field_module}${snippet_id}_fields"
            
            # Replace field module references
            result=${result//${field_module}_fields::/${unique_field_module}::}
        fi
    done <<< "$structs"
    
    # Find and replace any remaining field module patterns (for cases where struct isn't defined in snippet)
    local field_modules=$(echo "$content" | grep -o '[a-z_][a-z0-9_]*_fields::' | sed 's/_fields:://' | sort | uniq)
    
    while IFS= read -r module_name; do
        if [ -n "$module_name" ]; then
            local unique_module="${module_name}${snippet_id}_fields"
            result=${result//${module_name}_fields::/${unique_module}::}
        fi
    done <<< "$field_modules"
    
    echo "$result"
}

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
        
        # Process snippet content with dynamic replacements to avoid naming conflicts
        local snippet_content=$(cat "$snippet_file")
        
        # Generate unique replacements based on content analysis
        snippet_content=$(generate_unique_replacements "$snippet_content" "$i")
        
        # Add to batch compilation list
        BATCH_SNIPPETS+=("$snippet_content")
        BATCH_FILES+=("$file")
        BATCH_NUMBERS+=("$i")
    done
}

# Function to extract version from Cargo.toml for a given dependency
extract_version() {
    local dep_name="$1"
    local cargo_toml="./Cargo.toml"
    
    # Try to extract version using different patterns
    # Pattern 1: dep = "version"
    local version=$(grep "^${dep_name} = " "$cargo_toml" | sed 's/.*= *"\([^"]*\)".*/\1/' | head -1)
    
    # Pattern 2: dep = { version = "version", ... }
    if [ -z "$version" ]; then
        version=$(grep "^${dep_name} = " "$cargo_toml" | sed 's/.*version *= *"\([^"]*\)".*/\1/' | head -1)
    fi
    
    # Pattern 3: Multi-line table format
    if [ -z "$version" ]; then
        # Look for [dependencies.dep] or in a multi-line dep declaration
        version=$(awk "/^\[dependencies\.${dep_name}\]/,/^\[/ { if(/^version/) print }" "$cargo_toml" | sed 's/.*= *"\([^"]*\)".*/\1/' | head -1)
    fi
    
    # Fallback: Default versions if not found
    case "$dep_name" in
        "bson") echo "${version:-2.0}" ;;
        "serde") echo "${version:-1.0}" ;;
        "mongodb") echo "${version:-2.0}" ;;
        "tokio") echo "${version:-1.0}" ;;
        *) echo "${version:-1.0}" ;;
    esac
}

# Function to generate Cargo.toml from template with extracted versions
generate_cargo_toml() {
    local target_dir="$1"
    local script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    local template_file="$script_dir/templates/Cargo.toml.template"
    local output_file="$target_dir/Cargo.toml"
    
    if [ ! -f "$template_file" ]; then
        print_error "Template file not found: $template_file"
        return 1
    fi
    
    # Extract versions from main Cargo.toml
    local bson_version=$(extract_version "bson")
    local serde_version=$(extract_version "serde")
    local mongodb_version=$(extract_version "mongodb")
    local tokio_version=$(extract_version "tokio")
    local project_path=$(pwd)
    
    print_info "Using dependency versions: bson=$bson_version, serde=$serde_version, mongodb=$mongodb_version, tokio=$tokio_version"
    
    # Generate Cargo.toml from template
    sed -e "s|{{PROJECT_PATH}}|$project_path|g" \
        -e "s|{{BSON_VERSION}}|$bson_version|g" \
        -e "s|{{SERDE_VERSION}}|$serde_version|g" \
        -e "s|{{MONGODB_VERSION}}|$mongodb_version|g" \
        -e "s|{{TOKIO_VERSION}}|$tokio_version|g" \
        "$template_file" > "$output_file"
    
    if [ $? -eq 0 ]; then
        print_info "Generated Cargo.toml from template"
        return 0
    else
        print_error "Failed to generate Cargo.toml from template"
        return 1
    fi
}

# Function to compile all snippets in batch
compile_batch_snippets() {
    if [ ${#BATCH_SNIPPETS[@]} -eq 0 ]; then
        return
    fi
    
    print_info "Compiling ${#BATCH_SNIPPETS[@]} snippets in batch..."
    
    # Create a single test project for all snippets
    local batch_dir="$TEMP_DIR/batch_test"
    mkdir -p "$batch_dir/src/bin"
    
    # Generate Cargo.toml from template with extracted versions
    if ! generate_cargo_toml "$batch_dir"; then
        print_error "Failed to generate Cargo.toml, falling back to hardcoded versions"
        # Fallback to old method
        cat > "$batch_dir/Cargo.toml" << EOF
[package]
name = "doc_snippet_batch_test"
version = "0.1.0"
edition = "2021"

[dependencies]
tnuctipun = { path = "$(pwd)" }
bson = "2.0"
serde = { version = "1.0", features = ["derive"] }
mongodb = "2.0"
tokio = { version = "1.0", features = ["full"] }
EOF
    fi
    
    # Create individual binary targets for each snippet
    for i in "${!BATCH_SNIPPETS[@]}"; do
        local snippet_content="${BATCH_SNIPPETS[$i]}"
        local bin_name="snippet_$i"
        local bin_file="$batch_dir/src/bin/${bin_name}.rs"
        
        if echo "$snippet_content" | grep -q "fn main"; then
            # Snippet already has a main function
            echo "$snippet_content" > "$bin_file"
        else
            # Wrap snippet in a main function
            cat > "$bin_file" << 'EOF'
use tnuctipun::*;
use bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
EOF
            echo "$snippet_content" >> "$bin_file"
            cat >> "$bin_file" << 'EOF'
    
    Ok(())
}
EOF
        fi
    done
    
    # Compile all snippets at once using workspace check
    print_info "Running cargo check --workspace for all snippets..."
    local workspace_check_output

    if workspace_check_output=$(cd "$batch_dir" && cargo check --workspace --quiet 2>&1); then
        print_success "All snippets compiled successfully"
        VALID_SNIPPETS=${#BATCH_SNIPPETS[@]}
    else
        print_warning "Some snippets failed, checking individually..."
        
        # If workspace check fails, fall back to individual checks to identify which ones failed
        local batch_results=()

        for i in "${!BATCH_SNIPPETS[@]}"; do
            local bin_name="snippet_$i"
            
            if (cd "$batch_dir" && cargo check --bin "$bin_name" --quiet 2>/dev/null); then
                batch_results[$i]="success"
                VALID_SNIPPETS=$((VALID_SNIPPETS + 1))
            else
                batch_results[$i]="failed"
                FAILED_SNIPPETS=$((FAILED_SNIPPETS + 1))
            fi
        done
        
        # Report individual results
        for i in "${!BATCH_SNIPPETS[@]}"; do
            local file="${BATCH_FILES[$i]}"
            local snippet_num="${BATCH_NUMBERS[$i]}"
            
            if [ "${batch_results[$i]}" = "success" ]; then
                print_success "    ✓ Snippet $snippet_num from $file compiles successfully"
            else
                print_error "    ✗ Snippet $snippet_num from $file failed to compile"
                print_error "    Compilation errors:"
                local bin_name="snippet_$i"
                (cd "$batch_dir" && cargo check --bin "$bin_name" 2>&1 | sed 's/^/      /' || true)
            fi
        done
    fi
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

# Compile all snippets in batch
compile_batch_snippets

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
