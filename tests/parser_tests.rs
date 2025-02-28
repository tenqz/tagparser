use tagparser::{parse_tags, parse_tags_with_attr, extract_tag_content, extract_attribute_values};

#[test]
fn test_parse_tags() {
    let html = "<p>Test</p><a href='https://github.com/tenqz/'>Test Link 1</a><p>Another Text</p><a href='https://github.com/tenqz/'>Test Link 2</a><p>Another Text</p><a class='test' href='https://github.com/tenqz/'>Test Link 3</a><p>Another Text</p>".to_string();
    let tag_a = "a".to_string();
    let tag_p = "p".to_string();
    let tags_a = parse_tags(html.clone(), tag_a);
    let tags_p = parse_tags(html, tag_p);
    assert_eq!(
        vec![
            "<a href='https://github.com/tenqz/'>Test Link 1</a>".to_string(),
            "<a href='https://github.com/tenqz/'>Test Link 2</a>".to_string(),
            "<a class='test' href='https://github.com/tenqz/'>Test Link 3</a>".to_string()
        ],
        tags_a
    );
    assert_eq!(
        vec![
            "<p>Test</p>".to_string(),
            "<p>Another Text</p>".to_string(),
            "<p>Another Text</p>".to_string(),
            "<p>Another Text</p>".to_string()
        ],
        tags_p
    )
}

#[test]
fn test_parse_tags_with_attr() {
    let html = "<p>Test</p><a href='https://github.com/tenqz/'>Test Link 1</a><p>Another Text</p><a href='https://example.com/'>Test Link 2</a><p>Another Text</p><a class='test' href='https://github.com/tenqz/'>Test Link 3</a><p>Another Text</p>".to_string();
    
    // Test filtering by attribute name only
    let tags_with_href = parse_tags_with_attr(html.clone(), "a".to_string(), "href", None);
    assert_eq!(
        vec![
            "<a href='https://github.com/tenqz/'>Test Link 1</a>".to_string(),
            "<a href='https://example.com/'>Test Link 2</a>".to_string(),
            "<a class='test' href='https://github.com/tenqz/'>Test Link 3</a>".to_string()
        ],
        tags_with_href
    );
    
    // Test filtering by attribute name and value
    let tags_with_class_test = parse_tags_with_attr(html.clone(), "a".to_string(), "class", Some("test"));
    assert_eq!(
        vec![
            "<a class='test' href='https://github.com/tenqz/'>Test Link 3</a>".to_string()
        ],
        tags_with_class_test
    );
    
    // Test filtering by attribute with specific href value
    let tags_with_specific_href = parse_tags_with_attr(html.clone(), "a".to_string(), "href", Some("https://example.com/"));
    assert_eq!(
        vec![
            "<a href='https://example.com/'>Test Link 2</a>".to_string()
        ],
        tags_with_specific_href
    );
    
    // Test with non-existent attribute
    let tags_with_nonexistent_attr = parse_tags_with_attr(html.clone(), "a".to_string(), "nonexistent", None);
    assert_eq!(
        Vec::<String>::new(),
        tags_with_nonexistent_attr
    );
    
    // Test with non-existent tag
    let tags_with_nonexistent_tag = parse_tags_with_attr(html.clone(), "nonexistent".to_string(), "href", None);
    assert_eq!(
        Vec::<String>::new(),
        tags_with_nonexistent_tag
    );
}

#[test]
fn test_empty_html() {
    let html = "".to_string();
    
    // Test with empty HTML
    let tags_a = parse_tags(html.clone(), "a".to_string());
    assert_eq!(Vec::<String>::new(), tags_a);
    
    let tags_with_attr = parse_tags_with_attr(html, "a".to_string(), "href", None);
    assert_eq!(Vec::<String>::new(), tags_with_attr);
}

#[test]
fn test_malformed_html() {
    let html = "<a href='https://example.com'>Unclosed tag".to_string();
    
    // Test with malformed HTML (unclosed tag)
    let tags_a = parse_tags(html.clone(), "a".to_string());
    assert_eq!(Vec::<String>::new(), tags_a);
    
    let tags_with_attr = parse_tags_with_attr(html, "a".to_string(), "href", None);
    assert_eq!(Vec::<String>::new(), tags_with_attr);
}

#[test]
fn test_nested_tags() {
    let html = "<div><a href='https://example.com'>Link</a></div>".to_string();
    
    // Test with nested tags
    let tags_div = parse_tags(html.clone(), "div".to_string());
    assert_eq!(
        vec!["<div><a href='https://example.com'>Link</a></div>".to_string()],
        tags_div
    );
    
    let tags_a = parse_tags(html.clone(), "a".to_string());
    assert_eq!(
        vec!["<a href='https://example.com'>Link</a>".to_string()],
        tags_a
    );
}

#[test]
fn test_multiple_attributes() {
    let html = "<a href='https://example.com' class='button' id='link1' data-test='value'>Link</a>".to_string();
    
    // Test with multiple attributes
    let tags_with_class = parse_tags_with_attr(html.clone(), "a".to_string(), "class", Some("button"));
    assert_eq!(
        vec!["<a href='https://example.com' class='button' id='link1' data-test='value'>Link</a>".to_string()],
        tags_with_class
    );
    
    let tags_with_id = parse_tags_with_attr(html.clone(), "a".to_string(), "id", Some("link1"));
    assert_eq!(
        vec!["<a href='https://example.com' class='button' id='link1' data-test='value'>Link</a>".to_string()],
        tags_with_id
    );
    
    let tags_with_data_attr = parse_tags_with_attr(html, "a".to_string(), "data-test", Some("value"));
    assert_eq!(
        vec!["<a href='https://example.com' class='button' id='link1' data-test='value'>Link</a>".to_string()],
        tags_with_data_attr
    );
}

#[test]
fn test_extract_tag_content() {
    let html = r#"
        <a href='https://github.com'>GitHub</a>
        <p>This is a <strong>paragraph</strong> with text.</p>
        <a href='https://rust-lang.org'>Rust Language</a>
        <div class="container">Some content</div>
    "#.to_string();
    
    // Test extracting content from links
    let link_texts = extract_tag_content(html.clone(), "a".to_string());
    assert_eq!(
        vec!["GitHub", "Rust Language"],
        link_texts
    );
    
    // Test extracting content from paragraphs (includes nested HTML)
    let paragraph_texts = extract_tag_content(html.clone(), "p".to_string());
    assert_eq!(
        vec!["This is a <strong>paragraph</strong> with text."],
        paragraph_texts
    );
    
    // Test extracting content from divs
    let div_texts = extract_tag_content(html.clone(), "div".to_string());
    assert_eq!(
        vec!["Some content"],
        div_texts
    );
}

#[test]
fn test_extract_tag_content_empty() {
    let html = "<p></p><a href='https://example.com'></a>".to_string();
    
    // Test with empty content
    let p_texts = extract_tag_content(html.clone(), "p".to_string());
    assert_eq!(vec![""], p_texts);
    
    let a_texts = extract_tag_content(html.clone(), "a".to_string());
    assert_eq!(vec![""], a_texts);
}

#[test]
fn test_extract_tag_content_nonexistent() {
    let html = "<p>Paragraph</p>".to_string();
    
    // Test with non-existent tag
    let div_texts = extract_tag_content(html.clone(), "div".to_string());
    assert_eq!(Vec::<String>::new(), div_texts);
}

#[test]
fn test_extract_tag_content_with_attributes() {
    let html = r#"
        <a href='https://example.com' class='link' id='link1'>Example</a>
        <a href='https://rust-lang.org' class='official'>Rust</a>
    "#.to_string();
    
    // Test with tags having multiple attributes
    let link_texts = extract_tag_content(html, "a".to_string());
    assert_eq!(
        vec!["Example", "Rust"],
        link_texts
    );
}

#[test]
fn test_extract_tag_content_nested_tags() {
    let html = r#"
        <div class="outer">
            <div class="inner">Nested content</div>
            Outer content
        </div>
    "#.to_string();
    
    // Test with nested tags of the same type
    let div_texts = extract_tag_content(html, "div".to_string());
    assert_eq!(
        vec!["Nested content"],
        div_texts
    );
}

#[test]
fn test_extract_attribute_values() {
    let html = r#"
        <a href='https://github.com'>GitHub</a>
        <a href='https://rust-lang.org' class='official'>Rust</a>
        <a class='social' href='https://twitter.com'>Twitter</a>
        <div src='image1.jpg' alt='Image 1'>Content</div>
        <div src='image2.jpg'>Content</div>
    "#.to_string();
    
    // Test extracting href values from links
    let hrefs = extract_attribute_values(html.clone(), "a".to_string(), "href");
    assert_eq!(
        vec!["https://github.com", "https://rust-lang.org", "https://twitter.com"],
        hrefs
    );
    
    // Test extracting class values from links
    let classes = extract_attribute_values(html.clone(), "a".to_string(), "class");
    assert_eq!(
        vec!["official", "social"],
        classes
    );
    
    // Test extracting src values from divs
    let srcs = extract_attribute_values(html.clone(), "div".to_string(), "src");
    assert_eq!(
        vec!["image1.jpg", "image2.jpg"],
        srcs
    );
    
    // Test extracting alt values from divs (one missing)
    let alts = extract_attribute_values(html.clone(), "div".to_string(), "alt");
    assert_eq!(
        vec!["Image 1"],
        alts
    );
}

#[test]
fn test_extract_attribute_values_nonexistent() {
    let html = "<p>Paragraph</p>".to_string();
    
    // Test with non-existent tag
    let values = extract_attribute_values(html.clone(), "div".to_string(), "class");
    assert_eq!(Vec::<String>::new(), values);
    
    // Test with non-existent attribute
    let values = extract_attribute_values(html.clone(), "p".to_string(), "nonexistent");
    assert_eq!(Vec::<String>::new(), values);
}

#[test]
fn test_extract_attribute_values_empty_html() {
    let html = "".to_string();
    
    // Test with empty HTML
    let values = extract_attribute_values(html, "a".to_string(), "href");
    assert_eq!(Vec::<String>::new(), values);
}

#[test]
fn test_extract_attribute_values_malformed() {
    let html = "<a href=https://example.com>No quotes</a>".to_string();
    
    // Test with malformed attribute (no quotes)
    let values = extract_attribute_values(html, "a".to_string(), "href");
    assert_eq!(Vec::<String>::new(), values);
} 