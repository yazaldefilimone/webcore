#![allow(dead_code)]
pub fn is_tag_char(character: char) -> bool {
  // TODO: Include U+00A0 and higher.
  match character {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
    _ => false,
  }
}

pub fn is_color_pattern(pattern: &str) -> bool {
  if pattern.starts_with("rgb") {
    return true;
  }
  if pattern.starts_with("rgba") {
    return true;
  }
  if pattern.contains("hsl") {
    return true;
  }
  return pattern.starts_with("#");
}
