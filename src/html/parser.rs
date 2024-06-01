use crate::dom;

pub struct Parser {
  input: String,
  cursor: usize,
}

impl Parser {
  pub fn new(input: String) -> Self {
    Parser { input, cursor: 0 }
  }

  pub fn parse(&mut self) -> Vec<dom::Node> {
    let mut nodes = Vec::new();
    while self.cursor < self.input.len() {
      let node = self.parse_node();
      nodes.push(node);
    }
    nodes
  }

  fn parse_node(&mut self) -> dom::Node {
    let start = self.cursor;
    self.skip_whitespace();
    let node = match self.input.chars().nth(self.cursor) {
      Some('<') => self.parse_element(),
      Some('>') => self.parse_text(),
      _ => self.parse_text(),
    };
    self.cursor = start;
    node
  }

  fn parse_element(&mut self) -> dom::Node {
    self.cursor += 1;
    let tag_name = self.parse_tag_name();
    let atributes = self.parse_atributes();
    dom::create_element_node(tag_name, atributes)
  }

  fn parse_text(&mut self) -> dom::Node {
    let start = self.cursor;
    self.skip_whitespace();
    let end = self.cursor;
    let text = self.input[start..end].to_string();
    dom::create_text_node(text)
  }

  fn parse_tag_name(&mut self) -> String {
    let start = self.cursor;
    self.skip_whitespace();
    let end = self.cursor;
    let tag_name = self.input[start..end].to_string();
    self.cursor = end;
    tag_name
  }

  fn parse_atributes(&mut self) -> dom::AtributeMapType {
    let mut atributes = dom::AtributeMapType::new();
    while self.cursor < self.input.len() {
      self.skip_whitespace();
      if self.input.chars().nth(self.cursor) == Some('>') {
        break;
      }
      let start = self.cursor;
      self.skip_whitespace();
      let end = self.cursor;
      let name = self.input[start..end].to_string();
      self.skip_whitespace();
      if self.input.chars().nth(self.cursor) != Some('=') {
        break;
      }
      self.cursor += 1;
      self.skip_whitespace();
      let start = self.cursor;
      self.skip_whitespace();
      let end = self.cursor;
      let value = self.input[start..end].to_string();
      atributes.insert(name, value);
    }
    atributes
  }

  fn skip_whitespace(&mut self) {
    while self.cursor < self.input.len() && self.input.chars().nth(self.cursor) == Some(' ') {
      self.cursor += 1;
    }
  }

  fn skip_newline(&mut self) {
    while self.cursor < self.input.len() && self.input.chars().nth(self.cursor) == Some('\n') {
      self.cursor += 1;
    }
  }
}
