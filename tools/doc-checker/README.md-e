# Doc Checker

A Go CLI tool to extract and validate Rust code snippets from Markdown files. This tool replaces the complex bash script with a more maintainable, testable, and efficient solution.

## Features

- **Fast and reliable**: Written in Go for better performance and error handling
- **JSON output**: Machine-readable output format for CI/CD integration
- **Individual file processing**: Check specific files instead of all markdown files
- **Flexible options**: Verbose, quiet, quick modes for different use cases
- **Smart snippet processing**: Automatically handles struct name conflicts between snippets
- **Exit codes**: Proper exit codes for different failure scenarios
- **Unit tested**: Comprehensive test coverage for reliability

## Installation

### From source

```bash
cd tools/doc-checker
make build
```

### System-wide installation

```bash
cd tools/doc-checker
make install
```

This installs the binary to `/usr/local/bin/doc-checker`.

## Usage

### Basic usage

```bash
# Check all .md files under git control
doc-checker

# Check specific files
doc-checker -f README.md
doc-checker -f "docs/*.md"
doc-checker README.md docs/guide.md

# JSON output for CI/CD
doc-checker -o json

# Quiet mode
doc-checker -q

# Quick mode (exit on first error)
doc-checker --quick

# Verbose output
doc-checker -v
```

### Command line options

```
-f, --files FILES       Comma-separated list of files to check
-o, --output FORMAT     Output format: 'human' (default) or 'json'
-q, --quiet             Quiet mode: minimal output
-v, --verbose           Verbose mode (default)
--quick                 Quick mode: exit on first compilation error
--exit-on-error         Exit immediately on first error
--color                 Force colored output
--no-color              Disable colored output
--version               Show version
-h, --help              Show help message
```

### Exit codes

- `0` - All snippets compiled successfully
- `1` - Some snippets failed to compile
- `2` - Script configuration/setup error
- `3` - File not found or access error

## Colored Output

The tool automatically detects if your terminal supports colors and enables them by default. You can control color output with:

- `--color` - Force colored output even when not detected
- `--no-color` - Disable colored output
- `NO_COLOR=1` environment variable - Disable colors
- `FORCE_COLOR=1` environment variable - Force colors

Color coding:
- 🔵 **[INFO]** - General information (blue)
- 🟢 **[SUCCESS]** - Successful operations (green)  
- 🟡 **[WARNING]** - Warnings and recoverable issues (yellow)
- 🔴 **[ERROR]** - Errors and failures (red)

## JSON Output Format

When using `-o json`, the tool outputs structured data:

```json
{
  "summary": {
    "total_snippets": 5,
    "valid_snippets": 4,
    "failed_snippets": 1,
    "files_processed": 2
  },
  "files": {
    "README.md": {
      "snippets_found": 3,
      "snippets_valid": 3,
      "snippets_failed": 0,
      "errors": []
    },
    "docs/guide.md": {
      "snippets_found": 2,
      "snippets_valid": 1,
      "snippets_failed": 1,
      "errors": ["compilation error: undefined struct User"]
    }
  }
}
```

## Development

### Running tests

```bash
make test
```

### Code quality checks

```bash
make check  # Runs fmt, vet, and test
```

### Development workflow

```bash
# Build and test on README.md
make dev-test

# Test JSON output
make run-json

# Test specific file
make run-readme
```

## Advantages over the bash script

1. **Maintainability**: Go code is easier to read, debug, and extend
2. **Testing**: Unit tests ensure reliability and catch regressions
3. **Performance**: Faster execution, especially for large numbers of files
4. **Error handling**: Better error messages and recovery
5. **Cross-platform**: Works consistently across different operating systems
6. **Type safety**: Compile-time checks prevent many runtime errors
7. **JSON output**: Proper JSON formatting without shell escaping issues
8. **Memory efficiency**: Better handling of large files and many snippets

## Agent/CI Integration

This tool is designed to be easily integrated into automated workflows:

```bash
# Simple validation
if doc-checker -q; then
    echo "All documentation snippets are valid"
else
    echo "Some snippets failed validation"
    exit 1
fi

# JSON processing for detailed analysis
doc-checker -o json | jq '.summary.failed_snippets'

# Check specific files in a pull request
doc-checker -f "$(git diff --name-only main...HEAD | grep '\.md$' | tr '\n' ',')"
```

## Migration from bash script

The Go version provides the same functionality as the bash script but with improved:

- Reliability (no bash-specific quirks)
- Performance (native binary vs interpreted script)  
- Maintainability (structured code with tests)
- Output formatting (proper JSON without escaping issues)
- Error handling (specific exit codes and error messages)

The command-line interface is designed to be compatible with most common usage patterns of the original script.
