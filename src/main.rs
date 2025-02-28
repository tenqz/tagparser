pub mod parser;
use crate::parser::Parser;
use std::env;

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
pub fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: tagparser <html> <tag> [attr_name] [attr_value]");
        return;
    }
    
    let html = &args[1];
    let tag = &args[2];
    
    let mut parser = Parser::new(html.to_string());
    
    if args.len() >= 4 {
        let attr_name = &args[3];
        let attr_value = if args.len() >= 5 { Some(args[4].as_str()) } else { None };
        
        println!("{:?}", parser.parse_tags_with_attr(tag.to_string(), attr_name, attr_value));
    } else {
        println!("{:?}", parser.parse_tags(tag.to_string()));
    }
}
