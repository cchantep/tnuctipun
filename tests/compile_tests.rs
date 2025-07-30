#[test]
fn compile_tests() {
    // Skip compile-fail tests on beta/nightly as error messages may differ
    // We check the rustc version to determine if we're on stable
    let output = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to run rustc --version");

    let version_string = String::from_utf8(output.stdout).expect("Invalid UTF-8 from rustc");

    // Only run on stable Rust or when not in CI (for local development)
    if version_string.contains("-beta") || version_string.contains("-nightly") {
        println!(
            "Skipping compile-fail tests on non-stable Rust: {}",
            version_string.trim()
        );
        return;
    }

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
