#![allow(dead_code)]

enum DiagnosticEnum {
  Warning,
  Error,
  Info,
}

pub struct Diagnostics {
  pub errors: Vec<String>,
  pub warnings: Vec<String>,
  pub infos: Vec<String>,
}

impl Diagnostics {
  pub fn new() -> Self {
    Diagnostics { errors: Vec::new(), warnings: Vec::new(), infos: Vec::new() }
  }

  pub fn add_error(&mut self, message: String) {
    self.errors.push(message);
  }

  pub fn add_warning(&mut self, message: String) {
    self.warnings.push(message);
  }

  pub fn add_info(&mut self, message: String) {
    self.infos.push(message);
  }
}
