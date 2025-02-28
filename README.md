# Tagparser

A lightweight Rust library for parsing HTML tags with powerful filtering capabilities.

## Features

- Extract any HTML tags from HTML content
- Filter tags by attribute name (e.g., find all links with `href` attribute)
- Filter tags by attribute value (e.g., find all links to a specific URL)
- Simple and intuitive API
- Command-line interface for quick parsing

## Extract any html tags from html page

### Installation

You can install Ahref using cargo:

```
cargo add tagparser
```

### Usage

Here's an example of how to use Tagparser lib:

```rust
use tagparser::Parser;

fn main() {
    let html = "<a href='https://github.com/tenqz'>Test link</a><p>test p tag</p>".to_string();
    let mut parser = Parser::new(html);
    println!("{:?}", parser.parse_tags("a".to_string()));
    println!("{:?}", parser.parse_tags("p".to_string()));
}

```

As a result, all "a" and "p" tags will be displayed.
```text
["<a href='https://github.com/tenqz'>Test link</a>"]
["<p>test p tag</p>"]
```

### Filtering by Attributes

You can also filter tags by their attributes:

```rust
use tagparser::parse_tags_with_attr;

fn main() {
    let html = "<a href='https://github.com/tenqz'>Link 1</a><a class='button' href='https://example.com'>Link 2</a>".to_string();
    
    // Find all 'a' tags with 'href' attribute
    let tags_with_href = parse_tags_with_attr(html.clone(), "a".to_string(), "href", None);
    println!("All links: {:?}", tags_with_href);
    
    // Find all 'a' tags with 'class' attribute with value 'button'
    let button_links = parse_tags_with_attr(html.clone(), "a".to_string(), "class", Some("button"));
    println!("Button links: {:?}", button_links);
    
    // Find all 'a' tags with specific href value
    let github_links = parse_tags_with_attr(html.clone(), "a".to_string(), "href", Some("https://github.com/tenqz"));
    println!("GitHub links: {:?}", github_links);
}
```

Output:
```text
All links: ["<a href='https://github.com/tenqz'>Link 1</a>", "<a class='button' href='https://example.com'>Link 2</a>"]
Button links: ["<a class='button' href='https://example.com'>Link 2</a>"]
GitHub links: ["<a href='https://github.com/tenqz'>Link 1</a>"]
```

### Command Line Usage

You can also use Tagparser as a command-line tool:

```bash
# Basic usage - extract all tags of a specific type
tagparser "<html>...</html>" "a"

# Filter by attribute - extract all tags with a specific attribute
tagparser "<html>...</html>" "a" "href"

# Filter by attribute value - extract tags with a specific attribute value
tagparser "<html>...</html>" "a" "href" "https://github.com"
```

## Development

### Running Tests

The project includes a comprehensive test suite. To run the tests:

```bash
cargo test
```

The tests are organized into:

1. **Unit Tests** - Testing individual functions and methods
2. **Integration Tests** - Testing the CLI interface
3. **Documentation Tests** - Ensuring examples in documentation work correctly

### Project Structure

```
tagparser/
├── src/
│   ├── parser.rs    # Core parsing functionality
│   ├── lib.rs       # Library API
│   └── main.rs      # CLI implementation
├── tests/
│   ├── parser_tests.rs  # Tests for parsing functionality
│   └── cli_tests.rs     # Tests for CLI interface
└── README.md
```