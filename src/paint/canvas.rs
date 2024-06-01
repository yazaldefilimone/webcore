pub struct Canvas {
  pub width: f32,
  pub height: f32,
  pub context: String,
}

impl Canvas {
  pub fn new(width: f32, height: f32, context: String) -> Self {
    Canvas { width, height, context }
  }
}
