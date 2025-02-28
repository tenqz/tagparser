use std::process::Command;
use std::str;

#[test]
fn test_cli_basic_usage() {
    let output = Command::new("cargo")
        .args(&["run", "--", "<a href='https://example.com'>Link</a>", "a"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("<a href='https://example.com'>Link</a>"));
}

#[test]
fn test_cli_with_attribute() {
    let html = "<a href='https://example.com'>Link 1</a><a class='button'>Link 2</a>";
    
    let output = Command::new("cargo")
        .args(&["run", "--", html, "a", "class"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("<a class='button'>Link 2</a>"));
    assert!(!stdout.contains("<a href='https://example.com'>Link 1</a>"));
}

#[test]
fn test_cli_with_attribute_value() {
    let html = "<a class='button'>Button</a><a class='link'>Link</a>";
    
    let output = Command::new("cargo")
        .args(&["run", "--", html, "a", "class", "button"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("<a class='button'>Button</a>"));
    assert!(!stdout.contains("<a class='link'>Link</a>"));
}

#[test]
fn test_cli_no_matches() {
    let html = "<p>Paragraph</p>";
    
    let output = Command::new("cargo")
        .args(&["run", "--", html, "a"])
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
    assert!(stdout.contains("Usage: tagparser <html> <tag> [attr_name] [attr_value]"));
}

#[test]
fn test_cli_extract_content() {
    let html = "<a href='https://example.com'>Example</a><a href='#'>Home</a>";
    
    let output = Command::new("cargo")
        .args(&["run", "--", html, "a", "--content"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("\"Example\""));
    assert!(stdout.contains("\"Home\""));
    assert!(!stdout.contains("href"));
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
    let html = "<p>Paragraph</p>";
    
    let output = Command::new("cargo")
        .args(&["run", "--", html, "div", "--content"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert_eq!("[]", stdout.trim());
} 