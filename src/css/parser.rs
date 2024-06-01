use crate::{css, utils::validation::is_valid_identifier_char};

struct Parser {
  input: String,
  cursor: usize,
}

impl Parser {
  pub fn new(input: String) -> Self {
    Parser { input, cursor: 0 }
  }
}

pub fn parse(input: String) -> css::StyleSheet {
  let mut parser = Parser::new(input);
  return css::StyleSheet { rules: parser.parse_rules() };
}

impl Parser {
  fn parse_rules(&mut self) -> Vec<css::Rule> {
    let mut rules = Vec::new();
    // while !self.is_end() {
    //   rules.push(self.parse_rule());
    // }
    rules
  }

  /// Parse two hexadecimal digits.
  fn parse_hex_pair(&mut self) -> u8 {
    let s = &self.input[self.cursor..self.cursor + 2];
    self.cursor += 2;
    u8::from_str_radix(s, 16).unwrap()
  }
  fn parse_identifier(&mut self) -> String {
    return self.consume_while(is_valid_identifier_char);
  }

  fn skip_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }

  fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
    let mut result = String::new();
    while !self.is_end() && test(self.next_char()) {
      result.push(self.consume_char());
    }
    result
  }

  fn consume_char(&mut self) -> char {
    let mut iter = self.input[self.cursor..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_cursor, _) = iter.next().unwrap_or((1, ' '));
    self.cursor += next_cursor;
    cur_char
  }
  /// Read the current character without consuming it.
  fn next_char(&mut self) -> char {
    self.input[self.cursor..].chars().next().unwrap()
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.input.len()
  }
}
