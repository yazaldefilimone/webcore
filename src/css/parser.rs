use crate::css;
use crate::utils::validation::is_tag_char;
// ==============================
// https://www.w3.org/TR/css-syntax-3/#tokenizing-and-parsing
//

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
    while !self.is_end() {
      rules.push(self.parse_rule());
    }
    rules
  }
  // Parse a rule set: `<selectors> { <declarations> }`.
  fn parse_rule(&mut self) -> css::Rule {
    css::Rule { selectors: self.parse_selectors(), declarations: self.parse_declarations() }
  }
  // simple selector, e.g.: `type#id.class1.class2.class3`
  fn parse_simple_selector(&mut self) -> css::SimpleSelector {
    let mut simple_selector = css::SimpleSelector::new();
    while !self.is_end() {
      match self.peek_one() {
        '.' => {
          self.consume_expect(".");
          simple_selector.class.push(self.parse_identifier());
        }
        '#' => {
          self.consume_expect("#");
          simple_selector.id = Some(self.parse_identifier());
        }
        character if is_tag_char(character) => {
          simple_selector.tag_name = Some(self.parse_identifier());
        }
        _ => break,
      }
    }
    simple_selector
  }

  fn parse_selectors(&mut self) -> Vec<css::Selector> {
    let mut selectors = Vec::new();
    while !self.is_end() {
      selectors.push(self.parse_selector());
      self.skip_whitespace();
      match self.peek_one() {
        ',' => {
          self.consume_expect(",");
          self.skip_whitespace();
        }
        '}' => break,
        character => {
          panic!("Unexpected character '{}' in selector list", character);
        }
      }
    }
    // return selectors with highest specificity first, for use in matching.
    selectors.sort_by(|left, right| right.specificity().cmp(&left.specificity()));
    selectors
  }

  /// Parse a list of declarations enclosed in `{ ... }`.
  fn parse_declarations(&mut self) -> Vec<css::Declaration> {
    self.consume_expect("{");
    let mut declarations = Vec::new();
    while !self.is_end() && !self.starts_with("}") {
      self.skip_whitespace();
      declarations.push(self.parse_declaration());
      self.skip_whitespace();
    }
    self.consume_expect("}");
    declarations
  }
  /// Parse one `<property>: <value>;` declaration.
  fn parse_declaration(&mut self) -> css::Declaration {
    let property_name = self.parse_identifier();
    self.consume_expect(":");
    self.skip_whitespace();
    let value = self.parse_value();
    self.consume_expect(";");
    css::Declaration { name: property_name, value }
  }

  fn parse_value(&mut self) -> css::DeclarationValue {
    let value = self.parse_identifier();
    if
    css::DeclarationValue::Keyword(value)
  }

  fn parse_value_length(&mut self) -> css::DeclarationValue {
    css::DeclarationValue::Length(self.parse_float(), self.parse_unit())
  }

  fn parse_unit(&mut self) -> css::UnitValue {
    let unit = self.parse_identifier();
    match &*unit.to_ascii_lowercase() {
      "px" => css::UnitValue::Px,
      "em" => css::UnitValue::Em,
      "rem" => css::UnitValue::Rem,
      _ => panic!("Unknown unit '{}'", unit),
    }
  }

  fn parse_float(&mut self) -> f32 {
    let float = self.consume_while(|character| match character {
      '0'..='9' | '.' => true,
      _ => false,
    });
    float.parse().unwrap()
  }


  // todo: support other color formats, e.g. hex, rgba, hsla...
  fn parse_color(&mut self) -> css::DeclarationValue {
    self.consume_expect("#");
    let r = self.parse_hex_pair();
    let g = self.parse_hex_pair();
    let b = self.parse_hex_pair();
    css::DeclarationValue::Color(css::ColorValue { r, g, b, a: 255 })
  }

  /// Parse two hexadecimal digits.
  fn parse_hex_pair(&mut self) -> u8 {
    let value = &self.input[self.cursor..self.cursor + 2];
    self.cursor += 2;
    u8::from_str_radix(value, 16).unwrap()
  }
  fn parse_length(&mut self) -> f32 {
    let value = self.parse_number();
    value.parse::<f32>().unwrap()
  }

  fn parse_number(&mut self) -> String {
    let mut number = String::new();
    while !self.is_end() {
      match self.peek_one() {
        '0'..='9' => number.push(self.consume()),
        _ => break,
      }
    }
    number
  }

  // todo: support more complex selectors.
  fn parse_selector(&mut self) -> css::Selector {
    self.parse_simple_selector();
    css::Selector::Simple(self.parse_simple_selector())
  }

  fn parse_identifier(&mut self) -> String {
    return self.consume_while(is_tag_char);
  }

  fn skip_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }

  fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
    let start_cursor = self.cursor;
    while !self.is_end() && test(self.peek_one()) {
      self.advance_one()
    }
    return self.input[start_cursor..self.cursor].to_string();
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.input.len()
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
}
