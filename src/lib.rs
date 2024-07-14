use crate::html::HTMLParser;
use css::CSSParser;

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod paint;
pub mod utils;

pub fn parse_html(input: String) -> dom::Node {
  let mut parser = HTMLParser::new(input);
  parser.parse()
}

pub fn parse_css(input: String) -> css::StyleSheet {
  let mut parser = CSSParser::new(input);
  parser.parse()
}
