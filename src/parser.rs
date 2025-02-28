use regex::Regex;

pub struct Parser {
    html: String,
}

impl Parser {
    pub fn new(html: String) -> Self {
        Parser { html }
    }

    /// Parses HTML content and extracts all tags of the specified type
    /// 
    /// # Arguments
    /// 
    /// * `tag` - The HTML tag name to search for (e.g., "a", "p", "div")
    /// 
    /// # Returns
    /// 
    /// A vector of strings containing all matching HTML tags
    /// 
    /// # Example
    /// 
    /// ```
    /// # // This example is for internal documentation only and not run as a test
    /// # // To use this in your code, you would need to import Parser from the parser module
    /// # use tagparser::parser::Parser;
    /// # 
    /// let html = "<a href='https://example.com'>Link</a><p>Paragraph</p>";
    /// let mut parser = Parser::new(html.to_string());
    /// 
    /// // Get all links
    /// let links = parser.parse_tags("a".to_string());
    /// assert_eq!(links, vec!["<a href='https://example.com'>Link</a>"]);
    /// 
    /// // Get all paragraphs
    /// let paragraphs = parser.parse_tags("p".to_string());
    /// assert_eq!(paragraphs, vec!["<p>Paragraph</p>"]);
    /// ```
    pub fn parse_tags(&mut self, tag: String) -> Vec<String> {
        Regex::new(&format!(r"<{}.*?>.*?</{}.*?>", tag, tag))
            .unwrap()
            .find_iter(&self.html)
            .map(|x| x.as_str().to_string())
            .collect()
    }

    /// Filters HTML tags by attribute name and optionally by attribute value
    /// 
    /// # Arguments
    /// 
    /// * `tag` - The HTML tag name to search for (e.g., "a", "div", "img")
    /// * `attr_name` - The attribute name to filter by (e.g., "href", "class", "id")
    /// * `attr_value` - Optional attribute value to filter by
    ///   - If `None`, returns all tags with the specified attribute regardless of value
    ///   - If `Some(value)`, returns only tags where the attribute exactly matches the value
    /// 
    /// # Returns
    /// 
    /// A vector of strings containing the matching HTML tags
    /// 
    /// # Examples
    /// 
    /// ```
    /// # // This example is for internal documentation only and not run as a test
    /// # // To use this in your code, you would need to import Parser from the parser module
    /// # use tagparser::parser::Parser;
    /// # 
    /// let html = r#"
    ///     <a href="https://github.com">GitHub</a>
    ///     <a href="https://rust-lang.org" class="official">Rust</a>
    ///     <a class="social" href="https://twitter.com">Twitter</a>
    /// "#;
    /// 
    /// let mut parser = Parser::new(html.to_string());
    /// 
    /// // Example 1: Find all links with href attribute (any value)
    /// let links_with_href = parser.parse_tags_with_attr("a".to_string(), "href", None);
    /// // Returns all three links
    /// 
    /// // Example 2: Find links with class="social"
    /// let social_links = parser.parse_tags_with_attr("a".to_string(), "class", Some("social"));
    /// // Returns only: <a class="social" href="https://twitter.com">Twitter</a>
    /// 
    /// // Example 3: Find links to a specific URL
    /// let github_links = parser.parse_tags_with_attr("a".to_string(), "href", Some("https://github.com"));
    /// // Returns only: <a href="https://github.com">GitHub</a>
    /// ```
    /// 
    /// # Command Line Usage
    /// 
    /// When using the CLI tool, you can filter by attributes like this:
    /// 
    /// ```bash
    /// # Find all links with href attribute
    /// tagparser "<html>...</html>" "a" "href"
    /// 
    /// # Find all links with href pointing to github.com
    /// tagparser "<html>...</html>" "a" "href" "https://github.com"
    /// ```
    pub fn parse_tags_with_attr(&mut self, tag: String, attr_name: &str, attr_value: Option<&str>) -> Vec<String> {
        let all_tags = self.parse_tags(tag);
        
        all_tags.into_iter().filter(|tag_str| {
            // Check if the tag contains the attribute
            let attr_pattern = match attr_value {
                Some(value) => format!(r#"{}=["']{}["']"#, attr_name, value),
                None => format!(r#"{}=["'][^"']*["']"#, attr_name),
            };
            
            Regex::new(&attr_pattern).unwrap().is_match(tag_str)
        }).collect()
    }

    /// Extracts the content (text) from inside HTML tags of the specified type
    /// 
    /// This method returns only the text content between the opening and closing tags,
    /// without the tags themselves or any HTML attributes.
    /// 
    /// # Arguments
    /// 
    /// * `tag` - The HTML tag name to search for (e.g., "a", "p", "div")
    /// 
    /// # Returns
    /// 
    /// A vector of strings containing the text content of all matching tags
    /// 
    /// # Examples
    /// 
    /// ```
    /// # // This example is for internal documentation only and not run as a test
    /// # // To use this in your code, you would need to import Parser from the parser module
    /// # use tagparser::parser::Parser;
    /// # 
    /// let html = r#"
    ///     <a href="https://github.com">GitHub</a>
    ///     <p>This is a <strong>paragraph</strong> with some text.</p>
    ///     <div class="container">Some content</div>
    /// "#;
    /// 
    /// let mut parser = Parser::new(html.to_string());
    /// 
    /// // Extract content from links
    /// let link_texts = parser.extract_tag_content("a".to_string());
    /// assert_eq!(link_texts, vec!["GitHub"]);
    /// 
    /// // Extract content from paragraphs (includes nested HTML)
    /// let paragraph_texts = parser.extract_tag_content("p".to_string());
    /// assert_eq!(paragraph_texts, vec!["This is a <strong>paragraph</strong> with some text."]);
    /// 
    /// // Extract content from divs
    /// let div_texts = parser.extract_tag_content("div".to_string());
    /// assert_eq!(div_texts, vec!["Some content"]);
    /// ```
    pub fn extract_tag_content(&mut self, tag: String) -> Vec<String> {
        // Create a regex pattern that captures the content between tags
        let pattern = format!(r"<{}.*?>(.*?)</{}.*?>", tag, tag);
        
        // Find all matches and extract the captured group (content)
        Regex::new(&pattern)
            .unwrap()
            .captures_iter(&self.html)
            .map(|cap| cap[1].to_string())
            .collect()
    }
}
