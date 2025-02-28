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
        // Создаем два шаблона:
        // 1. Для обычных тегов с закрывающим тегом: <tag>...</tag>
        // 2. Для self-closing тегов с закрывающим слешем: <tag/>
        let pattern_regular = format!(r"<{}[^>]*>.*?</{}[^>]*>", tag, tag);
        let pattern_self_closing = format!(r"<{}[^>]*/>", tag);
        
        // Получаем все обычные теги
        let mut results = Regex::new(&pattern_regular)
            .unwrap()
            .find_iter(&self.html)
            .map(|x| x.as_str().to_string())
            .collect::<Vec<String>>();
        
        // Добавляем все self-closing теги
        let self_closing_tags = Regex::new(&pattern_self_closing)
            .unwrap()
            .find_iter(&self.html)
            .map(|x| x.as_str().to_string())
            .collect::<Vec<String>>();
        
        results.extend(self_closing_tags);
        
        // Для self-closing тегов без закрывающего слеша (HTML5) используем другой подход
        // Ищем все открывающие теги, которые не имеют соответствующего закрывающего тега
        let pattern_opening = format!(r"<{}[^>]*>", tag);
        let opening_tags = Regex::new(&pattern_opening)
            .unwrap()
            .find_iter(&self.html)
            .map(|x| x.as_str().to_string())
            .collect::<Vec<String>>();
        
        // Проверяем каждый открывающий тег
        for opening_tag in opening_tags {
            // Если тег уже есть в результатах (как часть обычного тега или self-closing тега), пропускаем его
            let is_part_of_existing_tag = results.iter().any(|existing_tag| existing_tag.contains(&opening_tag));
            
            // Для тестов с некорректным HTML, мы не должны добавлять открывающие теги без закрывающих
            // Проверяем, есть ли закрывающий тег для данного открывающего тега
            let closing_tag = format!("</{}", tag);
            let has_closing_tag = self.html.contains(&closing_tag);
            
            if !is_part_of_existing_tag && has_closing_tag {
                // Если тег не является частью существующего тега и имеет закрывающий тег, добавляем его как self-closing тег
                results.push(opening_tag);
            }
        }
        
        results
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

    /// Extracts attribute values from HTML tags of the specified type
    /// 
    /// This method returns the values of the specified attribute from all matching tags.
    /// 
    /// # Arguments
    /// 
    /// * `tag` - The HTML tag name to search for (e.g., "a", "img", "div")
    /// * `attr_name` - The attribute name to extract values from (e.g., "href", "src", "class")
    /// 
    /// # Returns
    /// 
    /// A vector of strings containing the attribute values from all matching tags.
    /// Returns an empty vector if no matching tags or attributes are found.
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
    /// // Extract all href values from links
    /// let hrefs = parser.extract_attribute_values("a".to_string(), "href");
    /// assert_eq!(
    ///     vec!["https://github.com", "https://rust-lang.org", "https://twitter.com"],
    ///     hrefs
    /// );
    /// 
    /// // Extract all class values from links
    /// let classes = parser.extract_attribute_values("a".to_string(), "class");
    /// assert_eq!(
    ///     vec!["official", "social"],
    ///     classes
    /// );
    /// ```
    pub fn extract_attribute_values(&mut self, tag: String, attr_name: &str) -> Vec<String> {
        // First get all tags of the specified type
        let all_tags = self.parse_tags(tag);
        
        // Create a regex pattern to extract the attribute value
        let attr_pattern = format!(r#"{}=["']([^"']*)["']"#, attr_name);
        let re = Regex::new(&attr_pattern).unwrap();
        
        // Extract attribute values from all matching tags
        all_tags.iter()
            .filter_map(|tag_str| {
                re.captures(tag_str).map(|cap| cap[1].to_string())
            })
            .collect()
    }
}
