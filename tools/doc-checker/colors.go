package main

import (
	"fmt"
	"os"
	"runtime"
)

// ANSI color codes
const (
	ColorReset  = "\033[0m"
	ColorRed    = "\033[31m"
	ColorGreen  = "\033[32m"
	ColorYellow = "\033[33m"
	ColorBlue   = "\033[34m"
	ColorPurple = "\033[35m"
	ColorCyan   = "\033[36m"
	ColorWhite  = "\033[37m"
)

// Color formatting functions
func colorize(color, text string) string {
	if !supportsColor() {
		return text
	}

	return color + text + ColorReset
}

func colorInfo(text string) string {
	return colorize(ColorBlue, text)
}

func colorSuccess(text string) string {
	return colorize(ColorGreen, text)
}

func colorWarning(text string) string {
	return colorize(ColorYellow, text)
}

func colorError(text string) string {
	return colorize(ColorRed, text)
}

// Check if the terminal supports color
func supportsColor() bool {
	// Disable colors if output is not a terminal
	if !isTerminal() {
		return false
	}

	// Disable colors on Windows unless explicitly enabled
	if runtime.GOOS == "windows" {
		return os.Getenv("FORCE_COLOR") != "" || os.Getenv("CLICOLOR_FORCE") != ""
	}

	// Check common environment variables
	term := os.Getenv("TERM")
	if term == "" || term == "dumb" {
		return false
	}

	// Check if colors are explicitly disabled
	if os.Getenv("NO_COLOR") != "" || os.Getenv("CLICOLOR") == "0" {
		return false
	}

	return true
}

func isTerminal() bool {
	// Check if stdout is a terminal
	if fileInfo, _ := os.Stdout.Stat(); fileInfo != nil {
		return (fileInfo.Mode() & os.ModeCharDevice) != 0
	}

	return false
}

// Formatted log functions
func logInfo(msg string) {
	fmt.Printf("%s %s\n", colorInfo("[INFO]"), msg)
}

func logSuccess(msg string) {
	fmt.Printf("%s %s\n", colorSuccess("[SUCCESS]"), msg)
}

func logWarning(msg string) {
	fmt.Printf("%s %s\n", colorWarning("[WARNING]"), msg)
}

func logError(msg string) {
	fmt.Printf("%s %s\n", colorError("[ERROR]"), msg)
}
