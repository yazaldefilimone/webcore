use insta::{assert_ron_snapshot, Settings};
use std::path::Path;
use webcore::css::{CSSParser, StyleSheet};
use webcore::dom::HtmlRoot;
use webcore::html::HTMLParser;

const CSS_TESTS_PATH: &str = "tests/cases/css/";
const HTML_TESTS_PATH: &str = "tests/cases/html/";

enum TestResult {
  HtmlRoot(HtmlRoot),
  StyleSheet(StyleSheet),
}

type RunFn = dyn Fn(&str) -> TestResult;

fn run_test(file_name: &str, file_path: &Path, run_fns: &[&RunFn]) {
  println!("Running test: {}...", file_name);
  let code = std::fs::read_to_string(file_path).expect("Failed to read test file");

  let results: Vec<TestResult> = run_fns.iter().map(|run_fn| run_fn(&code)).collect();

  let mut settings = Settings::clone_current();
  settings.set_prepend_module_to_snapshot(false);
  settings.set_omit_expression(true);
  settings.set_input_file(file_path);

  let snapshot_name = file_path.to_str().unwrap().split("/").last().unwrap();

  settings.bind(|| {
    for result in results.iter() {
      match result {
        TestResult::HtmlRoot(html_root) => assert_ron_snapshot!(snapshot_name, html_root),
        TestResult::StyleSheet(stylesheet) => assert_ron_snapshot!(snapshot_name, stylesheet),
      }
    }
  });
}

fn run_tests_in_dir(path: &Path, run_fns: &[&RunFn]) {
  let paths: Vec<_> = std::fs::read_dir(path)
    .expect("Failed to read directory")
    .filter_map(Result::ok)
    .map(|entry| entry.path())
    .filter(|path| path.is_file())
    .collect();

  for path in paths {
    if let Some(name) = path.file_stem().and_then(|os_str| os_str.to_str()) {
      run_test(name, &path, run_fns);
    }
  }
}

fn parse_html(code: &str) -> TestResult {
  TestResult::HtmlRoot(HTMLParser::new(code.to_string()).parse_root())
}

fn parse_css(code: &str) -> TestResult {
  TestResult::StyleSheet(CSSParser::new(code.to_string()).parse())
}

#[test]
fn test_html_parser() {
  run_tests_in_dir(Path::new(HTML_TESTS_PATH), &[&parse_html]);
}

#[test]
fn test_css_parser() {
  run_tests_in_dir(Path::new(CSS_TESTS_PATH), &[&parse_css]);
}
