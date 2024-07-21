#![allow(dead_code, unused_imports)]

use glob::glob;
use insta::assert_ron_snapshot;
use std::fs;
use std::path::Path;
use webcore::css::{self, CSSParser};
use webcore::dom;
use webcore::html::HTMLParser;

fn read_test_files_with_pattern(pattern: &str) -> Vec<(String, String)> {
  let mut patterns = Vec::new();
  let glob_pattern = glob(pattern).expect("Failed to read glob pattern");
  for entry in glob_pattern {
    let path = entry.expect("Failed to read file");
    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
    let content = fs::read_to_string(path).expect("Failed to read file");
    patterns.push((file_name.to_owned(), content));
  }
  patterns
}
fn format_file_name_with_module(file_name: &str, module: &str, ext: &str) -> String {
  let file_name = format!("{}_{}", module, file_name).replace(ext, "");
  return file_name;
}
fn setings_snapshot() -> insta::Settings {
  let mut settings = insta::Settings::clone_current();
  settings.set_prepend_module_to_snapshot(false);
  settings.set_omit_expression(true);
  settings
}

fn create_syle_sheet_parser(source_code: &str) -> css::StyleSheet {
  let mut parser = CSSParser::new(source_code.to_string());
  return parser.parse_syle_sheet();
}
fn create_html_parser(source_code: &str) -> dom::HtmlRoot {
  let mut parser = HTMLParser::new(source_code.to_string());
  return parser.parse_root();
}

#[test]
fn test_css_parser_snapshot() {
  let test_files = read_test_files_with_pattern("tests/golden_tests/css/*.css");
  let settings = setings_snapshot();
  settings.bind(|| {
    for (file_name, source_code) in test_files.iter() {
      let stylesheet = create_syle_sheet_parser(source_code);
      let file_name = format_file_name_with_module(file_name, "css_parser", ".css");
      assert_ron_snapshot!(file_name.clone(), stylesheet);
    }
  });
}

#[test]
fn test_html_parser_snapshot() {
  let test_files = read_test_files_with_pattern("tests/golden_tests/html/*.html");
  let settings = setings_snapshot();
  settings.bind(|| {
    for (file_name, source_code) in test_files.iter() {
      let html_root = create_html_parser(source_code);
      let file_name = format_file_name_with_module(file_name, "html_parser", ".html");
      assert_ron_snapshot!(file_name.clone(), html_root);
    }
  });
}
