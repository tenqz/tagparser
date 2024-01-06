# Tagparser

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
