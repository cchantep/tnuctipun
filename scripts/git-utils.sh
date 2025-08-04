#!/bin/bash

# Git utility functions for use in git hooks and scripts

# Function to check if files with a specific extension were changed in pushed commits
# Usage: check_files_changed "extension"
# Returns: 0 (true) if files with the extension were changed, 1 (false) otherwise
check_files_changed() {
    local extension="$1"
    local files_changed=false
    
    # Check if stdin has data (when run as git hook) or if running manually
    if [ -t 0 ]; then
        # Running manually - check against origin/main or just uncommitted changes
        if git rev-parse --verify origin/main >/dev/null 2>&1; then
            # Check changes between current branch and origin/main
            if git diff --name-only origin/main...HEAD | grep -q "\.$extension$"; then
                files_changed=true
            fi
        else
            # Fallback: check staged and unstaged changes
            if git diff --name-only HEAD | grep -q "\.$extension$" || git diff --cached --name-only | grep -q "\.$extension$"; then
                files_changed=true
            fi
        fi
    else
        # Read the push information (remote and URL are passed as stdin to pre-push hook)
        while read local_ref local_sha remote_ref remote_sha; do
            if [ "$local_sha" != "0000000000000000000000000000000000000000" ]; then
                if [ "$remote_sha" = "0000000000000000000000000000000000000000" ]; then
                    # New branch, check all commits
                    range="$local_sha"
                else
                    # Existing branch, check commits between remote and local
                    range="$remote_sha..$local_sha"
                fi
                
                # Check if any files with the specified extension were changed in the commits being pushed
                if git diff --name-only "$range" | grep -q "\.$extension$"; then
                    files_changed=true
                    break
                fi
            fi
        done
    fi    # Return true (0) if files changed, false (1) if not
    if [ "$files_changed" = true ]; then
        return 0
    else
        return 1
    fi
}
