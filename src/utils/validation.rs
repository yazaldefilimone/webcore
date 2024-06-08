#[allow(dead_code)]
pub fn is_tag_char(c: char) -> bool {
  match c {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
    _ => false,
  }
}
