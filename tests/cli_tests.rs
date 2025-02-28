use std::process::Command;
use std::str;

#[test]
fn test_cli_basic_usage() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "h1"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("main-title"));
    assert!(stdout.contains("Rust Programming Language"));
}

#[test]
fn test_cli_with_attribute() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "a", "class"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("nav-link"));
}

#[test]
fn test_cli_with_attribute_value() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "a", "class", "nav-link"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("nav-link"));
    assert!(!stdout.contains("social-link"));
}

#[test]
fn test_cli_no_matches() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "nonexistent"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!("[]", stdout.trim());
}

#[test]
fn test_cli_usage_message() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Usage:"));
}

#[test]
fn test_cli_extract_content() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "h1", "--content"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Rust Programming Language"));
}

#[test]
fn test_cli_extract_content_empty() {
    let html = "<p></p>";
    
    let output = Command::new("cargo")
        .args(&["run", "--", html, "p", "--content"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!("[\"\"]", stdout.trim());
}

#[test]
fn test_cli_extract_content_no_matches() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "nonexistent", "--content"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!("[]", stdout.trim());
}

#[test]
fn test_cli_extract_attribute_values() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "a", "href", "--attr-values"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("https://www.rust-lang.org"));
    assert!(stdout.contains("https://doc.rust-lang.org/book/"));
}

#[test]
fn test_cli_extract_attribute_values_empty() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "a", "nonexistent", "--attr-values"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!("[]", stdout.trim());
}

#[test]
fn test_cli_extract_attribute_values_multiple_attributes() {
    let html_file = "tests/test_data/rust_page.html";
    
    let output = Command::new("cargo")
        .args(&["run", "--", "--file", html_file, "option", "value", "--attr-values"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("beginner"));
    assert!(stdout.contains("intermediate"));
    assert!(stdout.contains("advanced"));
} 