pub mod parser;
use crate::parser::Parser;
use std::env;
use std::fs;

/// Tagparser CLI tool
/// 
/// This command-line tool allows you to extract HTML tags from HTML content
/// and filter them by attributes.
/// 
/// # Usage
/// 
/// ```bash
/// # Basic usage - extract all tags of a specific type
/// tagparser "<html>...</html>" "a"
/// 
/// # Filter by attribute - extract all tags with a specific attribute
/// tagparser "<html>...</html>" "a" "href"
/// 
/// # Filter by attribute value - extract tags with a specific attribute value
/// tagparser "<html>...</html>" "a" "href" "https://github.com"
/// 
/// # Extract content - extract only the text content inside tags
/// tagparser "<html>...</html>" "a" "--content"
/// 
/// # Extract attribute values - extract values of a specific attribute
/// tagparser "<html>...</html>" "a" "href" "--attr-values"
/// 
/// # Read HTML from file
/// tagparser --file "path/to/file.html" "a"
/// ```
/// 
/// # Examples
/// 
/// 1. Extract all links:
///    ```bash
///    tagparser "<a href='https://example.com'>Link</a>" "a"
///    ```
///    Output: `["<a href='https://example.com'>Link</a>"]`
/// 
/// 2. Extract all links with class attribute:
///    ```bash
///    tagparser "<a href='https://example.com'>Link</a><a class='button' href='#'>Button</a>" "a" "class"
///    ```
///    Output: `["<a class='button' href='#'>Button</a>"]`
/// 
/// 3. Extract all links with specific class value:
///    ```bash
///    tagparser "<a class='button'>Button 1</a><a class='link'>Link</a>" "a" "class" "button"
///    ```
///    Output: `["<a class='button'>Button 1</a>"]`
/// 
/// 4. Extract text content from links:
///    ```bash
///    tagparser "<a href='https://example.com'>Example</a><a href='#'>Home</a>" "a" "--content"
///    ```
///    Output: `["Example", "Home"]`
/// 
/// 5. Extract href values from links:
///    ```bash
///    tagparser "<a href='https://example.com'>Example</a><a href='https://github.com'>GitHub</a>" "a" "href" "--attr-values"
///    ```
///    Output: `["https://example.com", "https://github.com"]`
///
/// 6. Read HTML from file:
///    ```bash
///    tagparser --file "index.html" "a" "href" "--attr-values"
///    ```
///    Output: `["https://example.com", "https://github.com"]`
pub fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        print_usage();
        return;
    }
    
    let html_content;
    let mut tag_index = 2;
    
    // Check if we're reading from a file
    if args[1] == "--file" {
        if args.len() < 4 {
            print_usage();
            return;
        }
        
        let file_path = &args[2];
        match fs::read_to_string(file_path) {
            Ok(content) => {
                html_content = content;
                tag_index = 3;
            },
            Err(e) => {
                println!("Error reading file: {}", e);
                return;
            }
        }
    } else {
        html_content = args[1].clone();
    }
    
    let tag = &args[tag_index];
    
    let mut parser = Parser::new(html_content);
    
    if args.len() > tag_index + 1 {
        if args[tag_index + 1] == "--content" {
            // Extract content from tags
            println!("{:?}", parser.extract_tag_content(tag.to_string()));
        } else if args.len() > tag_index + 2 && args[tag_index + 2] == "--attr-values" {
            // Extract attribute values
            let attr_name = &args[tag_index + 1];
            println!("{:?}", parser.extract_attribute_values(tag.to_string(), attr_name));
        } else {
            // Filter by attribute
            let attr_name = &args[tag_index + 1];
            let attr_value = if args.len() > tag_index + 2 && args[tag_index + 2] != "--attr-values" { 
                Some(args[tag_index + 2].as_str()) 
            } else { 
                None 
            };
            
            println!("{:?}", parser.parse_tags_with_attr(tag.to_string(), attr_name, attr_value));
        }
    } else {
        println!("{:?}", parser.parse_tags(tag.to_string()));
    }
}

fn print_usage() {
    println!("Usage: tagparser <html> <tag> [attr_name] [attr_value]");
    println!("       tagparser <html> <tag> --content");
    println!("       tagparser <html> <tag> <attr_name> --attr-values");
    println!("       tagparser --file <path> <tag> [attr_name] [attr_value]");
    println!("       tagparser --file <path> <tag> --content");
    println!("       tagparser --file <path> <tag> <attr_name> --attr-values");
}
