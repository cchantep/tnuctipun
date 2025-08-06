package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

type DocChecker struct {
	config     *Config
	results    *Results
	tempDir    string
	snippetMap map[int]string // maps snippet index to source file path
}

func NewDocChecker(config *Config) *DocChecker {
	return &DocChecker{
		config: config,
		results: &Results{
			Summary: Summary{
				ErrorsByCategory: make(map[string]int),
			},
			Files: make(map[string]FileResult),
		},
		snippetMap: make(map[int]string),
	}
}

func (dc *DocChecker) Run() (*Results, error) {
	// Create temporary directory
	tempDir, err := os.MkdirTemp("", "doc-checker-*")

	if err != nil {
		return nil, fmt.Errorf("failed to create temp directory: %w", err)
	}

	dc.tempDir = tempDir

	if !dc.config.KeepTempDir {
		defer os.RemoveAll(tempDir)
	}

	// Discover files to process
	files, err := dc.discoverFiles()

	if err != nil {
		return nil, fmt.Errorf("failed to discover files: %w", err)
	}

	if len(files) == 0 {
		dc.logInfo("No Markdown files found")

		if dc.config.KeepTempDir {
			// Print in green color at the end
			fmt.Printf("\033[1;32m[doc-checker]\033[0m Temporary directory kept: \033[1;36m%s\033[0m\n", tempDir)
		}

		return dc.results, nil
	}

	dc.logInfo(fmt.Sprintf("Found %d Markdown files", len(files)))

	// Process each file
	for _, file := range files {
		if err := dc.processFile(file); err != nil {
			if dc.config.ExitOnError {
				return nil, fmt.Errorf("processing file %s: %w", file, err)
			}

			dc.logError(fmt.Sprintf("Error processing %s: %v", file, err))
		}
	}

	// Compile all snippets
	if err := dc.compileSnippets(); err != nil {
		return nil, fmt.Errorf("failed to compile snippets: %w", err)
	}

	if dc.config.KeepTempDir {
		// Print in green color at the end
		fmt.Printf("\033[1;32m[doc-checker]\033[0m Temporary directory kept: \033[1;36m%s\033[0m\n", tempDir)
	}

	return dc.results, nil
}

func (dc *DocChecker) discoverFiles() ([]string, error) {
	if len(dc.config.Files) > 0 {
		// Use specified files
		var files []string

		for _, path := range dc.config.Files {
			stat, err := os.Stat(path)
			if err != nil {
				return nil, fmt.Errorf("path not found: %s", path)
			}

			if stat.IsDir() {
				// If it's a directory, find all .md files recursively
				dirFiles, err := dc.findMarkdownFilesInDir(path)

				if err != nil {
					return nil, fmt.Errorf("failed to find markdown files in directory %s: %w", path, err)
				}

				files = append(files, dirFiles...)
			} else {
				// If it's a file, add it directly
				files = append(files, path)
			}
		}

		return files, nil
	}

	// Discover files using git
	cmd := exec.Command("git", "ls-files", "*.md")
	cmd.Dir = dc.config.ProjectRoot
	output, err := cmd.Output()

	if err != nil {
		return nil, fmt.Errorf("failed to list git files (are you in a git repository?): %w", err)
	}

	var files []string
	scanner := bufio.NewScanner(bytes.NewReader(output))

	for scanner.Scan() {
		file := strings.TrimSpace(scanner.Text())

		if file != "" && !strings.HasPrefix(file, "target/") {
			files = append(files, filepath.Join(dc.config.ProjectRoot, file))
		}
	}

	return files, scanner.Err()
}

func (dc *DocChecker) findMarkdownFilesInDir(dirPath string) ([]string, error) {
	var files []string

	err := filepath.Walk(dirPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		// Skip directories and only process .md files
		if !info.IsDir() && strings.HasSuffix(strings.ToLower(info.Name()), ".md") {
			// Skip files in target/ directory
			if !strings.Contains(path, "/target/") && !strings.Contains(path, "\\target\\") {
				files = append(files, path)
			}
		}

		return nil
	})

	if err != nil {
		return nil, err
	}

	return files, nil
}

func (dc *DocChecker) processFile(filePath string) error {
	dc.results.Summary.FilesProcessed++
	dc.logInfo(fmt.Sprintf("Processing: %s", filePath))

	// Initialize file result
	fileResult := FileResult{
		Errors: []string{},
	}

	// Extract Rust code blocks with IDs
	content, err := os.ReadFile(filePath)

	if err != nil {
		fileResult.Errors = append(fileResult.Errors, fmt.Sprintf("Failed to read file: %v", err))
		dc.results.Files[filePath] = fileResult

		return err
	}

	snippets, err := dc.extractRustSnippets(string(content))
	if err != nil {
		fileResult.Errors = append(fileResult.Errors, fmt.Sprintf("Failed to extract snippets: %v", err))
		dc.results.Files[filePath] = fileResult
		return err
	}

	fileResult.SnippetsFound = len(snippets)
	dc.results.Summary.TotalSnippets += len(snippets)

	if len(snippets) == 0 {
		dc.logInfo("  No Rust snippets found")
		dc.results.Files[filePath] = fileResult
		return nil
	}

	dc.logInfo(fmt.Sprintf("  Found %d Rust snippet(s)", len(snippets)))

	// Process each snippet individually
	for idx, snippet := range snippets {
		// Skip ignored snippets
		if snippet.Ignore {
			dc.logInfo(fmt.Sprintf("  Skipping ignored snippet %d", idx+1))
			continue
		}

		code := snippet.Content

		// Determine start line of snippet in markdown file, or use index as fallback
		startLine := dc.findSnippetStartLine(filePath, code, idx)

		// Normalize markdown filename (remove .md, replace / and .)
		base := filepath.Base(filePath)
		norm := strings.TrimSuffix(base, ".md")
		norm = strings.ReplaceAll(norm, ".", "_")
		norm = strings.ReplaceAll(norm, "-", "_")

		snippetFile := filepath.Join(dc.tempDir, fmt.Sprintf("%s-%d.rs", norm, startLine))

		// Create a snippet with just the code (no additional imports)
		var enhancedSnippet strings.Builder

		// Check if the code already has imports
		hasImports := strings.Contains(code, "use tnuctipun") || strings.Contains(code, "use serde")

		if !hasImports {
			// Add imports only if they don't exist
			enhancedSnippet.WriteString("use tnuctipun::{FieldWitnesses, MongoComparable, updates};\n")
			enhancedSnippet.WriteString("use serde::{Deserialize, Serialize};\n\n")
		}

		// Add the original code as-is
		enhancedSnippet.WriteString(code)

		if err := os.WriteFile(snippetFile, []byte(enhancedSnippet.String()), 0644); err != nil {
			return fmt.Errorf("failed to write snippet file: %w", err)
		}

		if dc.config.Verbose && dc.config.OutputFormat == "human" {
			dc.showSnippetPreview(code, idx+1)
		}
	}

	// Store the final file result
	dc.results.Files[filePath] = fileResult

	return nil
}

// Find the start line of the snippet in the markdown file
func (dc *DocChecker) findSnippetStartLine(filePath, snippet string, snippetIndex int) int {
	content, err := os.ReadFile(filePath)
	if err != nil {
		return snippetIndex + 1 // Use snippet index as fallback
	}

	lines := strings.Split(string(content), "\n")
	snippetLines := strings.Split(snippet, "\n")
	snippetsFound := 0

	// Look for the code block that contains this snippet
	for i := 0; i < len(lines); i++ {
		line := strings.TrimSpace(lines[i])

		// Found a rust code block start
		if strings.HasPrefix(line, "```rust") || strings.HasPrefix(line, "```rs") {
			// Check if the content after this marker matches our snippet
			codeStart := i + 1

			if codeStart < len(lines) {
				// Extract the code block content
				var codeLines []string

				for j := codeStart; j < len(lines); j++ {
					if strings.HasPrefix(strings.TrimSpace(lines[j]), "```") {
						break
					}

					codeLines = append(codeLines, lines[j])
				}

				// Compare the extracted code with our snippet
				if len(codeLines) >= len(snippetLines) {
					match := true

					for k := 0; k < len(snippetLines) && match; k++ {
						if k < len(codeLines) && strings.TrimSpace(codeLines[k]) != strings.TrimSpace(snippetLines[k]) {
							match = false
						}
					}

					if match && snippetsFound == snippetIndex {
						return i + 1 // Return the line number of the ```rust marker (1-based)
					}

					if match {
						snippetsFound++
					}
				}
			}
		}
	}

	// Fallback: use snippet index as line number
	return snippetIndex + 1
}

type Snippet struct {
	Content string
	Ignore  bool // If true, this snippet should be ignored during compilation
}

func (dc *DocChecker) extractRustSnippets(content string) ([]Snippet, error) {
	var snippets []Snippet

	lines := strings.Split(content, "\n")
	inCodeBlock := false
	isRustBlock := false
	shouldIgnore := false
	currentSnippet := []string{}

	for _, line := range lines {
		if strings.HasPrefix(line, "```") {
			if !inCodeBlock {
				// Starting a code block
				inCodeBlock = true
				codeBlockHeader := strings.TrimPrefix(line, "```")
				codeBlockHeader = strings.TrimSpace(codeBlockHeader)

				// Parse language and attributes: "rust", "rust:ignore", "rs", "rs:ignore"
				isRustBlock = false
				shouldIgnore = false

				if codeBlockHeader == "rust" || codeBlockHeader == "rs" {
					isRustBlock = true
				} else if codeBlockHeader == "rust:ignore" || codeBlockHeader == "rs:ignore" {
					isRustBlock = true
					shouldIgnore = true
				}

				currentSnippet = []string{}
			} else {
				// Ending a code block
				inCodeBlock = false

				if isRustBlock && len(currentSnippet) > 0 {
					// Filter out empty lines and markdown content
					filteredSnippet := dc.filterSnippetContent(currentSnippet)

					if len(filteredSnippet) > 0 {
						snippets = append(snippets, Snippet{
							Content: strings.Join(filteredSnippet, "\n"),
							Ignore:  shouldIgnore,
						})
					}
				}

				currentSnippet = []string{}
				isRustBlock = false
				shouldIgnore = false
			}
		} else if inCodeBlock && isRustBlock {
			currentSnippet = append(currentSnippet, line)
		}
	}

	// Handle case where file ends without closing code block
	if inCodeBlock && isRustBlock && len(currentSnippet) > 0 {
		filteredSnippet := dc.filterSnippetContent(currentSnippet)

		if len(filteredSnippet) > 0 {
			snippets = append(snippets, Snippet{
				Content: strings.Join(filteredSnippet, "\n"),
				Ignore:  shouldIgnore,
			})
		}
	}

	return snippets, nil
}

func (dc *DocChecker) filterSnippetContent(lines []string) []string {
	var filtered []string

	for _, line := range lines {
		trimmed := strings.TrimSpace(line)

		// Skip markdown headers that somehow got included, but preserve Rust attributes
		if strings.HasPrefix(trimmed, "#") && !strings.HasPrefix(trimmed, "#[") {
			continue
		}

		// Include all other lines - if it's in a rust fence, it should be rust code
		filtered = append(filtered, line)
	}

	return filtered
}

func (dc *DocChecker) showSnippetPreview(snippet string, snippetNum int) {
	lines := strings.Split(snippet, "\n")

	fmt.Printf("    Snippet %d preview:\n", snippetNum)

	previewLines := 3

	if len(lines) < previewLines {
		previewLines = len(lines)
	}

	for i := 0; i < previewLines; i++ {
		fmt.Printf("      %s\n", lines[i])
	}

	if len(lines) > 3 {
		fmt.Println("      ...")
	}

	fmt.Println()
}

func (dc *DocChecker) compileSnippets() error {
	// Find all snippet files - updated pattern to match new naming convention
	snippetFiles, err := filepath.Glob(filepath.Join(dc.tempDir, "*-*.rs"))

	if err != nil {
		return fmt.Errorf("failed to find snippet files: %w", err)
	}

	if len(snippetFiles) == 0 {
		return nil
	}

	dc.logInfo(fmt.Sprintf("Compiling %d snippets...", len(snippetFiles)))

	// Create Cargo project

	projectDir := filepath.Join(dc.tempDir, "test_project")
	if err := dc.createCargoProject(projectDir, snippetFiles); err != nil {
		return fmt.Errorf("failed to create cargo project: %w", err)
	}

	// Try workspace compilation first
	if dc.compileWorkspace(projectDir) {
		dc.logSuccess("All snippets compiled successfully")

		dc.results.Summary.ValidSnippets = len(snippetFiles)

		dc.updateAllFilesSuccess()

		return nil
	}

	if dc.config.QuickMode {
		dc.results.Summary.FailedSnippets = len(snippetFiles)

		dc.logWarning("Quick mode: Some snippets failed compilation")

		return nil
	}

	// Fall back to individual compilation
	dc.logWarning("Some snippets failed, checking individually...")

	return dc.compileIndividually(projectDir, snippetFiles)
}

func (dc *DocChecker) createCargoProject(projectDir string, snippetFiles []string) error {
	if err := os.MkdirAll(filepath.Join(projectDir, "src", "bin"), 0755); err != nil {
		return fmt.Errorf("failed to create project structure: %w", err)
	}

	// Create Cargo.toml content with binary declarations
	var binDeclarations strings.Builder

	for _, snippetFile := range snippetFiles {
		// Extract the base name without extension to use as binary name
		baseName := filepath.Base(snippetFile)
		binName := strings.TrimSuffix(baseName, ".rs")

		binDeclarations.WriteString(fmt.Sprintf(`
[[bin]]
name = "%s"
path = "src/bin/%s.rs"
`, binName, binName))
	}

	// Extract dependency versions from main project Cargo.toml
	dependencies, err := dc.extractDependencyVersions()

	if err != nil {
		return fmt.Errorf("failed to extract dependency versions: %w", err)
	}

	cargoToml := fmt.Sprintf(`[package]
name = "doc_snippet_test"
version = "0.1.0"
edition = "2021"

[dependencies]
tnuctipun = { path = "%s" }
%s%s`, dc.config.ProjectRoot, dependencies, binDeclarations.String())

	// Write Cargo.toml to both projectDir and tempDir if KeepTempDir is set
	cargoTomlPath := filepath.Join(projectDir, "Cargo.toml")

	if err := os.WriteFile(cargoTomlPath, []byte(cargoToml), 0644); err != nil {
		return fmt.Errorf("failed to write Cargo.toml: %w", err)
	}

	if dc.config.KeepTempDir {
		// Also keep a copy in the tempDir root for investigation
		_ = os.WriteFile(filepath.Join(dc.tempDir, "Cargo.toml"), []byte(cargoToml), 0644)
	}

	// Create binary files for each snippet
	for _, snippetFile := range snippetFiles {
		snippet, err := os.ReadFile(snippetFile)

		if err != nil {
			return fmt.Errorf("failed to read snippet file: %w", err)
		}

		// Use the same naming logic as in binary declarations
		baseName := filepath.Base(snippetFile)
		binName := strings.TrimSuffix(baseName, ".rs") // Remove .rs extension for consistency
		binContent := dc.wrapSnippet(string(snippet))
		binPath := filepath.Join(projectDir, "src", "bin", binName+".rs") // Add .rs extension to file path

		if err := os.WriteFile(binPath, []byte(binContent), 0644); err != nil {
			return fmt.Errorf("failed to write binary file: %w", err)
		}
	}

	return nil
}

// extractDependencyVersions reads the main Cargo.toml and extracts dependency versions
func (dc *DocChecker) extractDependencyVersions() (string, error) {
	cargoTomlPath := filepath.Join(dc.config.ProjectRoot, "Cargo.toml")
	content, err := os.ReadFile(cargoTomlPath)

	if err != nil {
		return "", fmt.Errorf("failed to read main Cargo.toml: %w", err)
	}

	lines := strings.Split(string(content), "\n")
	var dependencies strings.Builder

	// Dependencies we need for testing, with fallback versions
	neededDeps := map[string]string{
		"bson":        `"2.15.0"`, // fallback version
		"serde":       `"1.0"`,    // fallback version (will be overridden with features)
		"mongodb":     `"2.8.2"`,  // testing-specific, reasonable version
		"tokio":       `"1.47.1"`, // testing-specific, reasonable version
		"chrono":      `"0.4"`,    // fallback version
		"async-trait": `"0.1"`,    // testing-specific, reasonable version
		"uuid":        `"1.17.0"`, // testing-specific, extracted from Cargo.lock
	}

	// Parse main Cargo.toml to find actual versions
	inDependencies := false

	for _, line := range lines {
		trimmed := strings.TrimSpace(line)

		// Check if we're entering the [dependencies] section
		if trimmed == "[dependencies]" {
			inDependencies = true
			continue
		}

		// Check if we're leaving the dependencies section
		if inDependencies && strings.HasPrefix(trimmed, "[") && trimmed != "[dependencies]" {
			inDependencies = false
			continue
		}

		if inDependencies && strings.Contains(trimmed, "=") {
			parts := strings.SplitN(trimmed, "=", 2)

			if len(parts) == 2 {
				depName := strings.TrimSpace(parts[0])
				depValue := strings.TrimSpace(parts[1])

				// Update our needed dependencies with actual version from main project
				if _, exists := neededDeps[depName]; exists {
					// Extract just the version string if it's a complex dependency
					if strings.HasPrefix(depValue, "{") {
						// For complex dependencies like serde = { version = "1.0.219", features = ["derive"] }
						// extract just the version part
						versionMatch := strings.Contains(depValue, "version")

						if versionMatch {
							start := strings.Index(depValue, `version = "`) + len(`version = "`)

							if start > len(`version = "`)-1 {
								end := strings.Index(depValue[start:], `"`)

								if end > 0 {
									version := depValue[start : start+end]
									neededDeps[depName] = `"` + version + `"`
								}
							}
						}
					} else {
						// Simple version string, use as-is
						neededDeps[depName] = depValue
					}
				}
			}
		}
	}

	// Build the dependencies string
	for dep, version := range neededDeps {
		switch dep {
		case "serde":
			dependencies.WriteString(fmt.Sprintf("serde = { version = %s, features = [\"derive\"] }\n", version))
		case "tokio":
			dependencies.WriteString(fmt.Sprintf("tokio = { version = %s, features = [\"full\"] }\n", version))
		case "chrono":
			dependencies.WriteString(fmt.Sprintf("chrono = { version = %s, features = [\"serde\"] }\n", version))
		case "uuid":
			dependencies.WriteString(fmt.Sprintf("uuid = { version = %s, features = [\"v4\", \"serde\"] }\n", version))
		default:
			dependencies.WriteString(fmt.Sprintf("%s = %s\n", dep, version))
		}
	}

	return dependencies.String(), nil
}

func (dc *DocChecker) wrapSnippet(snippet string) string {
	if strings.Contains(snippet, "fn main") {
		return snippet
	}

	return fmt.Sprintf(`use tnuctipun::*;
use bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
%s
	Ok(())
}`, snippet)
}

func (dc *DocChecker) compileWorkspace(projectDir string) bool {
	cmd := exec.Command("cargo", "check", "--workspace")
	cmd.Dir = projectDir

	output, err := cmd.CombinedOutput()

	if err != nil {
		if dc.config.Verbose {
			fmt.Printf("Workspace compilation failed:\n%s\n", string(output))
		}

		return false
	}

	return true
}

func (dc *DocChecker) categorizeError(errorOutput string) string {
	if strings.Contains(errorOutput, "use of unresolved module") {
		return "MISSING_FIELD_WITNESS"
	}

	if strings.Contains(errorOutput, "no field") && strings.Contains(errorOutput, "on type") {
		return "UNKNOWN_FIELD"
	}

	if strings.Contains(errorOutput, "unclosed delimiter") {
		return "SYNTAX_ERROR"
	}

	if strings.Contains(errorOutput, "trait bounds were not satisfied") {
		return "MISSING_TRAIT"
	}

	if strings.Contains(errorOutput, "expected expression") {
		return "SYNTAX_ERROR"
	}

	return "COMPILATION_ERROR"
}

func (dc *DocChecker) compileIndividually(projectDir string, snippetFiles []string) error {
	for _, snippetFile := range snippetFiles {
		// Use the same name pattern as in createCargoProject
		baseName := filepath.Base(snippetFile)
		binName := strings.TrimSuffix(baseName, ".rs")

		cmd := exec.Command("cargo", "check", "--bin", binName, "--quiet")
		cmd.Dir = projectDir

		if cmd.Run() == nil {
			dc.results.Summary.ValidSnippets++

			// Find the original markdown file for this snippet
			originalFile := dc.getOriginalFileFromSnippet(baseName)

			if originalFile != "" {
				// Update the file result with success
				if result, exists := dc.results.Files[originalFile]; exists {
					result.SnippetsValid++

					dc.results.Files[originalFile] = result
				}
			}
		} else {
			dc.results.Summary.FailedSnippets++

			// Get detailed error for reporting
			errorCmd := exec.Command("cargo", "check", "--bin", binName)
			errorCmd.Dir = projectDir
			errorOutput, _ := errorCmd.CombinedOutput()

			// Categorize the error
			errorStr := string(errorOutput)
			errorCategory := dc.categorizeError(errorStr)
			dc.results.Summary.ErrorsByCategory[errorCategory]++

			if len(errorStr) > 500 {
				errorStr = errorStr[:500] + "... (truncated)"
			}

			// Find the original markdown file for this snippet
			originalFile := dc.getOriginalFileFromSnippet(baseName)

			if originalFile != "" {
				// Update the file result with the error
				if result, exists := dc.results.Files[originalFile]; exists {
					result.SnippetsFailed++
					result.Errors = append(result.Errors, fmt.Sprintf("Snippet %s (%s): %s", binName, errorCategory, errorStr))
					dc.results.Files[originalFile] = result
				}
			} else {
				// If mapping failed, still log it but continue with global tracking
				dc.logError(fmt.Sprintf("Could not map snippet %s to original file", baseName))
			}

			dc.logError(fmt.Sprintf("Compilation failed for %s (%s): %s", binName, errorCategory, errorStr))

			if dc.config.ExitOnError {
				return fmt.Errorf("compilation failed for %s", binName)
			}
		}
	}

	return nil
}

// getOriginalFileFromSnippet maps a snippet filename back to the original markdown file
func (dc *DocChecker) getOriginalFileFromSnippet(snippetBaseName string) string {
	// Remove .rs extension first
	snippetName := strings.TrimSuffix(snippetBaseName, ".rs")

	// Snippet files are named like "normalized_filename-123" where normalized_filename comes from markdown file
	// and 123 is the line number
	parts := strings.Split(snippetName, "-")

	if len(parts) < 2 {
		return ""
	}

	// Remove the line number part (last part) and reconstruct the normalized name
	normalizedName := strings.Join(parts[:len(parts)-1], "-")

	// Look for the file in our results by comparing normalized names
	for filePath := range dc.results.Files {
		base := filepath.Base(filePath)
		expectedNorm := strings.TrimSuffix(base, ".md")
		expectedNorm = strings.ReplaceAll(expectedNorm, ".", "_")
		expectedNorm = strings.ReplaceAll(expectedNorm, "-", "_")

		if normalizedName == expectedNorm {
			return filePath
		}
	}

	return ""
}

func (dc *DocChecker) updateAllFilesSuccess() {
	for filePath, result := range dc.results.Files {
		result.SnippetsValid = result.SnippetsFound
		dc.results.Files[filePath] = result
	}
}

func (dc *DocChecker) logInfo(msg string) {
	if dc.config.Verbose && dc.config.OutputFormat == "human" {
		logInfo(msg)
	}
}

func (dc *DocChecker) logSuccess(msg string) {
	if dc.config.Verbose && dc.config.OutputFormat == "human" {
		logSuccess(msg)
	}
}

func (dc *DocChecker) logWarning(msg string) {
	if dc.config.OutputFormat == "human" {
		logWarning(msg)
	}
}

func (dc *DocChecker) logError(msg string) {
	if dc.config.OutputFormat == "human" {
		logError(msg)
	}
}
