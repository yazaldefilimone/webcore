use crate::html::HTMLParser;

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
