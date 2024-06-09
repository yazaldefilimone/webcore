#[allow(dead_code)]
pub fn is_tag_char(character: char) -> bool {
  // TODO: Include U+00A0 and higher.
  match character {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
    _ => false,
  }
}
