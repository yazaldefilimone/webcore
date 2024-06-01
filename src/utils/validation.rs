pub fn is_valid_identifier_char(c: char) -> bool {
  match c {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true, // TODO: Include U+00A0 and higher.
    _ => false,
  }
}
