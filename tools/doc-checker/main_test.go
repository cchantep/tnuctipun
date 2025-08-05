package main

import (
	"io/ioutil"
	"os"
	"path/filepath"
	"testing"
)

func TestExtractRustSnippets(t *testing.T) {
	checker := &DocChecker{}

	testCases := []struct {
		name     string
		content  string
		expected int
	}{
		{
			name: "simple rust block",
			content: `# Test
` + "```rust\n" + `fn main() {
    println!("Hello");
}
` + "```",
			expected: 1,
		},
		{
			name: "multiple rust blocks",
			content: `# Test
` + "```rust\n" + `struct User { name: String }
` + "```\n\n" + "```rust\n" + `impl User { fn new() -> Self {} }
` + "```",
			expected: 2,
		},
		{
			name: "mixed languages",
			content: `# Test
` + "```javascript\n" + `console.log("hello");
` + "```\n" + "```rust\n" + `println!("hello");
` + "```",
			expected: 1,
		},
		{
			name:     "no rust blocks",
			content:  `# Test\nSome text with no code blocks.`,
			expected: 0,
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			// Create temporary file
			tmpfile, err := ioutil.TempFile("", "test*.md")
			if err != nil {
				t.Fatal(err)
			}
			defer os.Remove(tmpfile.Name())

			if _, err := tmpfile.WriteString(tc.content); err != nil {
				t.Fatal(err)
			}
			tmpfile.Close()

			snippets, err := checker.extractRustSnippetsWithIDs(tc.content)
			if err != nil {
				t.Fatalf("extractRustSnippetsWithIDs failed: %v", err)
			}

			if len(snippets) != tc.expected {
				t.Errorf("expected %d snippets, got %d", tc.expected, len(snippets))
			}
		})
	}
}

func TestDiscoverFiles(t *testing.T) {
	// Create a temporary directory structure
	tmpDir, err := ioutil.TempDir("", "test-discover")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	// Create some test files
	testFiles := []string{
		"README.md",
		"docs/guide.md",
		"src/lib.rs", // Should be ignored
	}

	for _, file := range testFiles {
		fullPath := filepath.Join(tmpDir, file)
		if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
			t.Fatal(err)
		}
		if err := ioutil.WriteFile(fullPath, []byte("test content"), 0644); err != nil {
			t.Fatal(err)
		}
	}

	config := &Config{
		ProjectRoot: tmpDir,
		Files:       []string{filepath.Join(tmpDir, "README.md")},
	}

	checker := NewDocChecker(config)
	files, err := checker.discoverFiles()
	if err != nil {
		t.Fatal(err)
	}

	if len(files) != 1 {
		t.Errorf("expected 1 file, got %d", len(files))
	}

	if files[0] != filepath.Join(tmpDir, "README.md") {
		t.Errorf("expected %s, got %s", filepath.Join(tmpDir, "README.md"), files[0])
	}
}

func containsString(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr ||
		(len(s) > len(substr) && contains(s, substr)))
}

func contains(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}
