use crate::dom;
use crate::utils::validation::is_tag_char;

pub struct HTMLParser {
  input: String,
  cursor: usize,
}

impl HTMLParser {
  pub fn new(input: String) -> Self {
    HTMLParser { input, cursor: 0 }
  }

  pub fn parse(&mut self) -> dom::Node {
    let mut nodes = self.parse_nodes();
    if nodes.len() == 1 {
      return nodes.swap_remove(0);
    }
    dom::create_element("html".to_string(), dom::AtributeMapType::new(), nodes)
  }

  fn parse_nodes(&mut self) -> Vec<dom::Node> {
    self.skip_trivial();
    let mut nodes = Vec::new();
    while !self.is_end() && !self.starts_with("</") {
      nodes.push(self.parse_node());
      self.skip_trivial();
    }
    nodes
  }

  fn parse_node(&mut self) -> dom::Node {
    match self.peek_one() {
      '<' => self.parse_element(),
      _ => self.parse_text(),
    }
  }

  fn parse_element(&mut self) -> dom::Node {
    self.consume_expect("<");
    let tag_name = self.parse_tag_name();
    let attributes = self.parse_attributes();
    self.consume_expect(">");
    let children = self.parse_nodes();

    self.consume_expect("<");
    self.consume_expect("/");
    self.consume_expect(&tag_name);
    self.consume_expect(">");
    dom::create_element(tag_name, attributes, children)
  }

  fn parse_text(&mut self) -> dom::Node {
    let text = self.consume_while(|character| character != '<');
    dom::create_text(text)
  }

  fn parse_tag_name(&mut self) -> String {
    self.consume_while(is_tag_char)
  }

  fn parse_attribute(&mut self) -> (String, String) {
    let name = self.parse_tag_name();
    self.consume_expect("=");
    let value = self.parse_attribute_value();
    (name, value)
  }

  fn parse_attribute_value(&mut self) -> String {
    let open_quote = self.consume();
    assert!(open_quote == '"' || open_quote == '\'');
    let value = self.consume_while(|c| c != open_quote);
    self.consume_expect(open_quote.to_string().as_str());
    value
  }

  fn parse_attributes(&mut self) -> dom::AtributeMapType {
    let mut attributes = dom::AtributeMapType::new();
    self.skip_whitespace();
    while self.peek_one() != '>' && self.peek_one() != '/' {
      let (name, value) = self.parse_attribute();
      attributes.insert(name, value);
      self.skip_whitespace();
    }
    attributes
  }

  fn skip_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }

  fn skip_trivial(&mut self) {
    self.skip_whitespace();
  }

  fn peek_one(&self) -> char {
    self.input[self.cursor..].chars().next().unwrap()
  }

  fn peek_many(&self, count: usize) -> String {
    self.input[self.cursor..].chars().take(count).collect()
  }

  fn starts_with(&self, s: &str) -> bool {
    self.input[self.cursor..].starts_with(s)
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.input.len()
  }

  fn advance_one(&mut self) {
    self.cursor += 1;
  }

  fn advance_many(&mut self, count: usize) {
    self.cursor += count;
  }

  fn consume(&mut self) -> char {
    let mut iter = self.input[self.cursor..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_cursor, _) = iter.next().unwrap_or((1, ' '));
    self.cursor += next_cursor;
    cur_char
  }

  fn consume_expect(&mut self, text: &str) {
    if &self.peek_many(text.len()) == text {
      self.advance_many(text.len());
    } else {
      panic!("Expected '{}' but got '{}'", text, &self.peek_many(text.len()));
    }
  }

  fn consume_while(&mut self, mut test: impl FnMut(char) -> bool) -> String {
    let start_cursor = self.cursor;
    while !self.is_end() && test(self.peek_one()) {
      self.advance_one();
    }
    self.input[start_cursor..self.cursor].to_string()
  }
}
