pub mod parser;
pub use crate::parser::Parser;

/// Extract html tag from page
/// # Examples
/// ```
///     use tagparser::parse_tags;
///
///     let html = "<p>Test</p><a href='https://github.com/tenqz/'>Test Link 1</a><p>Another Text</p><a href='https://github.com/tenqz/'>Test Link 2</a><p>Another Text</p><a class='test' href='https://github.com/tenqz/'>Test Link 3</a><p>Another Text</p>".to_string();
///     let tag_a = "a".to_string();
///     let tag_p = "p".to_string();
///     let tags_a = parse_tags(html.clone(), tag_a);
///     let tags_p = parse_tags(html, tag_p);
///     assert_eq!(
///        vec![
///            "<a href='https://github.com/tenqz/'>Test Link 1</a>".to_string(),
///            "<a href='https://github.com/tenqz/'>Test Link 2</a>".to_string(),
///            "<a class='test' href='https://github.com/tenqz/'>Test Link 3</a>".to_string()
///        ],
///        tags_a
///    );
///     assert_eq!(
///        vec![
///             "<p>Test</p>".to_string(),
///             "<p>Another Text</p>".to_string(),
///             "<p>Another Text</p>".to_string(),
///             "<p>Another Text</p>".to_string()
///        ],
///        tags_p
///     )
///
/// ```
pub fn parse_tags(html: String, tag: String) -> Vec<String> {
    let mut parser = Parser::new(html);
    parser.parse_tags(tag)
}

/// Extract HTML tags with specific attribute from page
/// 
/// This function allows you to filter HTML tags not only by tag name but also by their attributes.
/// You can search for tags with a specific attribute (like "href" or "class") and optionally
/// filter by the exact value of that attribute.
/// 
/// # Arguments
/// 
/// * `html` - HTML content to parse
/// * `tag` - The HTML tag name to search for (e.g., "a", "div", "img")
/// * `attr_name` - The attribute name to filter by (e.g., "href", "class", "id")
/// * `attr_value` - Optional attribute value to filter by:
///   - Pass `None` to find all tags with the specified attribute regardless of value
///   - Pass `Some("value")` to find only tags where the attribute equals "value"
/// 
/// # Examples
/// 
/// Basic usage - finding all links with href attribute:
/// 
/// ```
///     use tagparser::parse_tags_with_attr;
///
///     let html = "<p>Test</p><a href='https://github.com/tenqz/'>Test Link 1</a><p>Another Text</p><a href='https://example.com/'>Test Link 2</a><p>Another Text</p><a class='test' href='https://github.com/tenqz/'>Test Link 3</a><p>Another Text</p>".to_string();
///     
///     // Find all 'a' tags with 'href' attribute
///     let tags_with_href = parse_tags_with_attr(html.clone(), "a".to_string(), "href", None);
///     assert_eq!(
///        vec![
///            "<a href='https://github.com/tenqz/'>Test Link 1</a>".to_string(),
///            "<a href='https://example.com/'>Test Link 2</a>".to_string(),
///            "<a class='test' href='https://github.com/tenqz/'>Test Link 3</a>".to_string()
///        ],
///        tags_with_href
///     );
///     
///     // Find all 'a' tags with 'class' attribute with value 'test'
///     let tags_with_class_test = parse_tags_with_attr(html.clone(), "a".to_string(), "class", Some("test"));
///     assert_eq!(
///        vec![
///            "<a class='test' href='https://github.com/tenqz/'>Test Link 3</a>".to_string()
///        ],
///        tags_with_class_test
///     );
/// ```
/// 
/// # Common Use Cases
/// 
/// 1. Extract all links to a specific domain:
///    ```
///    # use tagparser::parse_tags_with_attr;
///    # let html = "<a href='https://github.com'>Link</a>".to_string();
///    let github_links = parse_tags_with_attr(html, "a".to_string(), "href", Some("https://github.com"));
///    ```
/// 
/// 2. Find all images with a specific class:
///    ```
///    # use tagparser::parse_tags_with_attr;
///    # let html = "<img class='gallery' src='image.jpg'>".to_string();
///    let gallery_images = parse_tags_with_attr(html, "img".to_string(), "class", Some("gallery"));
///    ```
/// 
/// 3. Extract all input fields of a form:
///    ```
///    # use tagparser::parse_tags_with_attr;
///    # let html = "<input name='username'><input name='password'>".to_string();
///    let form_inputs = parse_tags_with_attr(html, "input".to_string(), "name", None);
///    ```
pub fn parse_tags_with_attr(html: String, tag: String, attr_name: &str, attr_value: Option<&str>) -> Vec<String> {
    let mut parser = Parser::new(html);
    parser.parse_tags_with_attr(tag, attr_name, attr_value)
}

/// Extract the text content from inside HTML tags
/// 
/// This function extracts only the text content between the opening and closing tags,
/// without the tags themselves or any HTML attributes.
/// 
/// # Arguments
/// 
/// * `html` - HTML content to parse
/// * `tag` - The HTML tag name to search for (e.g., "a", "p", "div")
/// 
/// # Returns
/// 
/// A vector of strings containing the text content of all matching tags
/// 
/// # Examples
/// 
/// Basic usage - extracting text from links and paragraphs:
/// 
/// ```
///     use tagparser::extract_tag_content;
///
///     let html = r#"
///         <a href='https://github.com'>GitHub</a>
///         <p>This is a <strong>paragraph</strong> with text.</p>
///         <a href='https://rust-lang.org'>Rust Language</a>
///     "#.to_string();
///     
///     // Extract text from all links
///     let link_texts = extract_tag_content(html.clone(), "a".to_string());
///     assert_eq!(
///         vec!["GitHub", "Rust Language"],
///         link_texts
///     );
///     
///     // Extract text from paragraphs (includes nested HTML)
///     let paragraph_texts = extract_tag_content(html.clone(), "p".to_string());
///     assert_eq!(
///         vec!["This is a <strong>paragraph</strong> with text."],
///         paragraph_texts
///     );
/// ```
/// 
/// # Common Use Cases
/// 
/// 1. Extract link text without HTML:
///    ```
///    # use tagparser::extract_tag_content;
///    # let html = "<a href='https://example.com'>Visit Example</a>".to_string();
///    let link_texts = extract_tag_content(html, "a".to_string());
///    // Returns: ["Visit Example"]
///    ```
/// 
/// 2. Extract headings from a page:
///    ```
///    # use tagparser::extract_tag_content;
///    # let html = "<h1>Main Title</h1><h2>Subtitle</h2>".to_string();
///    let headings = extract_tag_content(html, "h1".to_string());
///    // Returns: ["Main Title"]
///    ```
/// 
/// 3. Extract list items:
///    ```
///    # use tagparser::extract_tag_content;
///    # let html = "<ul><li>Item 1</li><li>Item 2</li></ul>".to_string();
///    let items = extract_tag_content(html, "li".to_string());
///    // Returns: ["Item 1", "Item 2"]
///    ```
pub fn extract_tag_content(html: String, tag: String) -> Vec<String> {
    let mut parser = Parser::new(html);
    parser.extract_tag_content(tag)
}
