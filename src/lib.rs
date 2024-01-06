pub mod parser;
use crate::parser::Parser;

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
