#!/usr/bin/env python3
"""
Extract Rust code snippets from Markdown files.

This script parses Markdown files and extracts code blocks that are marked as Rust code.
"""

import sys
import re
import os

def extract_rust_snippets(file_path, temp_dir, safe_name):
    """Extract Rust code snippets from a markdown file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Split content by lines for easier processing
        lines = content.split('\n')
        
        snippets = []
        in_rust_block = False
        current_snippet = []
        
        for line in lines:
            if line.strip().startswith('```rust'):
                # Start of a Rust code block
                in_rust_block = True
                current_snippet = []
            elif line.strip() == '```' and in_rust_block:
                # End of code block
                in_rust_block = False
                if current_snippet:
                    snippet_code = '\n'.join(current_snippet).strip()
                    if snippet_code and not is_placeholder_snippet(snippet_code):
                        snippets.append(snippet_code)
                current_snippet = []
            elif in_rust_block:
                # Inside a Rust code block
                current_snippet.append(line)
        
        # Write snippets to files
        snippet_count = 0
        for i, snippet in enumerate(snippets, 1):
            # Check if snippet contains multiple struct definitions
            struct_matches = list(re.finditer(r'^#\[derive.*?\]\s*\nstruct\s+\w+\s*\{.*?\}', snippet, re.MULTILINE | re.DOTALL))
            
            if len(struct_matches) > 1:
                # Split into multiple snippets
                for j, match in enumerate(struct_matches):
                    snippet_count += 1
                    snippet_file = os.path.join(temp_dir, f"{safe_name}_snippet_{snippet_count}.rs")
                    
                    # Extract individual struct definition
                    struct_code = match.group(0)
                    
                    with open(snippet_file, 'w', encoding='utf-8') as sf:
                        sf.write(struct_code)
            else:
                # Single snippet
                snippet_count += 1
                snippet_file = os.path.join(temp_dir, f"{safe_name}_snippet_{snippet_count}.rs")
                with open(snippet_file, 'w', encoding='utf-8') as sf:
                    sf.write(snippet)
        
        # Write count to a file
        count_file = os.path.join(temp_dir, f"count_{safe_name}.txt")

        with open(count_file, 'w') as cf:
            cf.write(str(snippet_count))
        
        print(f"Extracted {snippet_count} snippets from {file_path}")
        return 0
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}", file=sys.stderr)
        return 1

def is_placeholder_snippet(code):
    """Check if a code snippet is just a placeholder and shouldn't be compiled."""
    # Skip snippets that are clearly placeholders
    placeholder_patterns = [
        r'^\s*//\s*Your\s+code\s+here\s*$',  # // Your code here
        r'^\s*//\s*\.\.\.\s*$',              # // ...
        r'^\s*#\s*\.\.\.\s*$',               # # ...
        r'^\s*\.\.\.\s*$',                   # ...
    ]
    
    code_stripped = code.strip()
    
    # Check if it's just a placeholder
    for pattern in placeholder_patterns:
        if re.match(pattern, code_stripped, re.IGNORECASE | re.MULTILINE):
            return True
    
    # Check if it's too short to be meaningful
    if len(code_stripped) < 10:
        return True
    
    return False

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: extract_rust_snippets.py <file_path> <temp_dir> <safe_name>", file=sys.stderr)
        sys.exit(1)
    
    file_path = sys.argv[1]
    temp_dir = sys.argv[2]
    safe_name = sys.argv[3]
    
    sys.exit(extract_rust_snippets(file_path, temp_dir, safe_name))
