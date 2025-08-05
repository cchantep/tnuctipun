package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
)

const version = "1.0.0"

type Config struct {
	Files           []string
	OutputFormat    string
	Verbose         bool
	Quiet           bool
	QuickMode       bool
	ExitOnError     bool
	ShowVersion     bool
	ShowHelp        bool
	ForceColor      bool
	NoColor         bool
	ProjectRoot     string
	TempDir         string
	KeepTempDir     bool // New option to keep temp dir after execution
	ShowSuggestions bool // Show suggestions for fixing common errors
}

type Results struct {
	Summary Summary               `json:"summary"`
	Files   map[string]FileResult `json:"files"`
}

type Summary struct {
	TotalSnippets    int            `json:"total_snippets"`
	ValidSnippets    int            `json:"valid_snippets"`
	FailedSnippets   int            `json:"failed_snippets"`
	FilesProcessed   int            `json:"files_processed"`
	ErrorsByCategory map[string]int `json:"errors_by_category"`
}

type FileResult struct {
	SnippetsFound  int      `json:"snippets_found"`
	SnippetsValid  int      `json:"snippets_valid"`
	SnippetsFailed int      `json:"snippets_failed"`
	Errors         []string `json:"errors"`
}

func main() {
	config, err := parseFlags()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(2)
	}

	if config.ShowHelp {
		showHelp()
		os.Exit(0)
	}

	if config.ShowVersion {
		fmt.Printf("doc-checker version %s\n", version)
		os.Exit(0)
	}

	// Setup logging
	if config.Quiet {
		log.SetOutput(os.Stderr)
	} else if !config.Verbose {
		log.SetFlags(0)
	}

	checker := NewDocChecker(config)
	results, err := checker.Run()

	if err != nil {
		if config.OutputFormat == "json" {
			errorResult := Results{
				Summary: Summary{},
				Files:   make(map[string]FileResult),
			}
			json.NewEncoder(os.Stdout).Encode(errorResult)
		} else {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		}

		os.Exit(2)
	}

	// Output results
	if config.OutputFormat == "json" {
		encoder := json.NewEncoder(os.Stdout)
		encoder.SetIndent("", "  ")

		if err := encoder.Encode(results); err != nil {
			fmt.Fprintf(os.Stderr, "Error encoding JSON: %v\n", err)
			os.Exit(2)
		}
	} else {
		printHumanResults(results, config.Verbose, config.ShowSuggestions)
	}

	// Exit with appropriate code
	if results.Summary.FailedSnippets > 0 {
		os.Exit(1)
	}
}

func parseFlags() (*Config, error) {
	config := &Config{
		OutputFormat: "human",
		Verbose:      true,
	}

	var filesStr string

	flag.StringVar(&filesStr, "f", "", "Comma-separated list of files to check")
	flag.StringVar(&filesStr, "files", "", "Comma-separated list of files to check")
	flag.StringVar(&config.OutputFormat, "o", "human", "Output format: human or json")
	flag.StringVar(&config.OutputFormat, "output", "human", "Output format: human or json")
	flag.BoolVar(&config.Quiet, "q", false, "Quiet mode")
	flag.BoolVar(&config.Quiet, "quiet", false, "Quiet mode")
	flag.BoolVar(&config.Verbose, "v", true, "Verbose mode")
	flag.BoolVar(&config.Verbose, "verbose", true, "Verbose mode")
	flag.BoolVar(&config.QuickMode, "quick", false, "Quick mode: exit on first compilation error")
	flag.BoolVar(&config.ExitOnError, "exit-on-error", false, "Exit immediately on first error")
	flag.BoolVar(&config.ForceColor, "color", false, "Force colored output")
	flag.BoolVar(&config.NoColor, "no-color", false, "Disable colored output")
	flag.BoolVar(&config.ShowVersion, "version", false, "Show version")
	flag.BoolVar(&config.ShowHelp, "h", false, "Show help")
	flag.BoolVar(&config.ShowHelp, "help", false, "Show help")
	flag.BoolVar(&config.KeepTempDir, "keep-temp", false, "Keep temporary directory after execution")
	flag.BoolVar(&config.ShowSuggestions, "suggestions", false, "Show suggestions for fixing common documentation errors")

	flag.Parse()

	if config.Quiet {
		config.Verbose = false
	}

	// Handle color settings
	if config.ForceColor {
		os.Setenv("FORCE_COLOR", "1")
	}
	if config.NoColor {
		os.Setenv("NO_COLOR", "1")
	}

	if config.OutputFormat != "human" && config.OutputFormat != "json" {
		return nil, fmt.Errorf("invalid output format '%s'. Must be 'human' or 'json'", config.OutputFormat)
	}

	// Parse files
	if filesStr != "" {
		config.Files = strings.Split(filesStr, ",")

		for i, file := range config.Files {
			config.Files[i] = strings.TrimSpace(file)
		}
	}

	// Add remaining arguments as files
	config.Files = append(config.Files, flag.Args()...)

	// Get project root - look for Cargo.toml in parent directories
	wd, err := os.Getwd()
	if err != nil {
		return nil, fmt.Errorf("failed to get working directory: %w", err)
	}

	// Find project root by looking for Cargo.toml
	projectRoot := findProjectRoot(wd)
	if projectRoot == "" {
		return nil, fmt.Errorf("could not find project root (no Cargo.toml found in parent directories)")
	}

	config.ProjectRoot = projectRoot

	return config, nil
}

func findProjectRoot(startDir string) string {
	dir := startDir
	for {
		cargoPath := filepath.Join(dir, "Cargo.toml")
		if _, err := os.Stat(cargoPath); err == nil {
			return dir
		}

		parent := filepath.Dir(dir)
		if parent == dir {
			// Reached filesystem root
			break
		}
		dir = parent
	}
	return ""
}

func showHelp() {
	fmt.Printf(`doc-checker version %s

Extract and validate Rust code snippets from Markdown files.

USAGE:
	doc-checker [OPTIONS] [FILES...]

OPTIONS:
	-f, --files FILES       Comma-separated list of files to check
	-o, --output FORMAT     Output format: 'human' (default) or 'json'
	-q, --quiet             Quiet mode: minimal output
	-v, --verbose           Verbose mode (default)
	--quick                 Quick mode: exit on first compilation error
	--exit-on-error         Exit immediately on first error
	--color                 Force colored output
	--no-color              Disable colored output
	--version               Show version
	-h, --help              Show this help message

EXAMPLES:
	doc-checker                              # Check all .md files under git control
	doc-checker -f README.md                 # Check only README.md
	doc-checker -o json -q                   # JSON output, quiet mode
	doc-checker --quick README.md docs/*.md  # Quick check of specific docs
	doc-checker -o json --exit-on-error      # JSON output, fail fast

EXIT CODES:
	0   All snippets compiled successfully
	1   Some snippets failed to compile
	2   Script configuration/setup error
	3   File not found or access error

`, version)
}

func printHumanResults(results *Results, verbose bool, showSuggestions bool) {
	if verbose {
		fmt.Println()
		logInfo("=== SUMMARY ===")
		logInfo(fmt.Sprintf("Total Rust snippets found: %d", results.Summary.TotalSnippets))
		logSuccess(fmt.Sprintf("Valid snippets: %d", results.Summary.ValidSnippets))
	}

	if results.Summary.FailedSnippets > 0 {
		logError(fmt.Sprintf("Failed snippets: %d", results.Summary.FailedSnippets))

		// Show error categories if we have them
		if len(results.Summary.ErrorsByCategory) > 0 {
			fmt.Println()
			logWarning("Error breakdown by category:")
			for category, count := range results.Summary.ErrorsByCategory {
				var categoryDesc string
				switch category {
				case "MISSING_FIELD_WITNESS":
					categoryDesc = "Missing field witness modules (need struct definitions with FieldWitnesses derive)"
				case "UNKNOWN_FIELD":
					categoryDesc = "References to non-existent fields"
				case "SYNTAX_ERROR":
					categoryDesc = "Syntax errors (unclosed delimiters, malformed expressions)"
				case "MISSING_TRAIT":
					categoryDesc = "Missing trait implementations (e.g., Deserialize, Serialize)"
				default:
					categoryDesc = "General compilation errors"
				}
				fmt.Printf("  • %s: %d (%s)\n", category, count, categoryDesc)
			}

			// Show suggestions if requested
			if showSuggestions {
				fmt.Println()
				logInfo("💡 Suggestions to fix these errors:")

				if results.Summary.ErrorsByCategory["MISSING_FIELD_WITNESS"] > 0 {
					fmt.Println("  🔧 MISSING_FIELD_WITNESS: Each code snippet should either:")
					fmt.Println("     • Include the full struct definition with #[derive(FieldWitnesses)] in the same snippet")
					fmt.Println("     • Or be split into separate documentation sections showing struct definition first")
					fmt.Println("     • Example: Move struct definitions to the beginning of each code example")
					fmt.Println()
				}

				if results.Summary.ErrorsByCategory["UNKNOWN_FIELD"] > 0 {
					fmt.Println("  🔧 UNKNOWN_FIELD: Field name mismatches detected:")
					fmt.Println("     • Check if the field names in the examples match the struct definitions")
					fmt.Println("     • Ensure consistency between struct fields and update operations")
					fmt.Println("     • Run 'cargo expand' to see what field modules are generated")
					fmt.Println()
				}

				if results.Summary.ErrorsByCategory["SYNTAX_ERROR"] > 0 {
					fmt.Println("  🔧 SYNTAX_ERROR: Code formatting issues:")
					fmt.Println("     • Check for unclosed braces, parentheses, or brackets")
					fmt.Println("     • Ensure proper indentation and line endings")
					fmt.Println("     • Test code snippets in a Rust playground first")
					fmt.Println()
				}

				if results.Summary.ErrorsByCategory["MISSING_TRAIT"] > 0 {
					fmt.Println("  🔧 MISSING_TRAIT: Add required derive macros:")
					fmt.Println("     • Add #[derive(Deserialize, Serialize)] to structs used with MongoDB")
					fmt.Println("     • Include #[derive(Debug, Clone)] for better usability")
					fmt.Println("     • Consider adding #[derive(Default)] for struct initialization")
					fmt.Println()
				}
			}
		}

		fmt.Println()
		logError("Some documentation snippets failed to compile!")
		logError("Please update the failing snippets to match the current API.")

		fmt.Println("\nDetailed results:")

		for file, result := range results.Files {
			if result.SnippetsFailed > 0 {
				fmt.Printf("  %s: %d failed out of %d snippets\n",
					file, result.SnippetsFailed, result.SnippetsFound)
				for _, err := range result.Errors {
					// Print first few lines of each error
					lines := strings.Split(err, "\n")
					maxLines := 5
					if len(lines) > maxLines {
						lines = lines[:maxLines]
						lines = append(lines, "    ... (error truncated)")
					}
					for _, line := range lines {
						fmt.Printf("    %s\n", line)
					}
					fmt.Println()
				}
			}
		}
	} else {
		if verbose {
			logSuccess("All documentation snippets are valid! 🎉")
		}
	}
}
