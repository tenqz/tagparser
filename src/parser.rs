use regex::Regex;

pub struct Parser {
    html: String,
}

impl Parser {
    pub fn new(html: String) -> Self {
        Parser { html }
    }

    pub fn parse_tags(&mut self, tag: String) -> Vec<String> {
        Regex::new(&format!(r"<{}.*?>.*?</{}.*?>", tag, tag))
            .unwrap()
            .find_iter(&self.html)
            .map(|x| x.as_str().to_string())
            .collect()
    }
}
