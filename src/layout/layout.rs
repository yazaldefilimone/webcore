pub struct Layout {
  pub width: f32,
  pub height: f32,
}

impl Layout {
  pub fn new(width: f32, height: f32) -> Self {
    Layout { width, height }
  }
}
