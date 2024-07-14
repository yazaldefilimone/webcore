use insta::assert_snapshot;
use itertools::Itertools;
use std::{collections::HashMap, path::Path};
use web_core::css::{CSSParser, StyleSheet};
use web_core::dom::Node;
use web_core::html::HTMLParser;

const CSS_TESTS_PATH: &str = "tests/cases/css/";
const HTML_TESTS_PATH: &str = "tests/cases/html/";

enum TestResult {
  Node(Node),
  StyleSheet(StyleSheet),
}

type RunFn = dyn Fn(&str) -> TestResult;

fn run_test(name: &str, path: &Path, run: &[&RunFn], module_path: &str) {
  println!("Running test: {}...", name);
  println!("path: {}", path.display());
  let code = std::fs::read_to_string(path).expect("Failed to read test file");
  let file_name = path.to_str().and_then(|path| path.rsplit_once(module_path)).unwrap().1;
  let file_path = format!("{}{}", &module_path[1..], file_name);
  let file_path = Path::new(&file_path);

  let mut results: HashMap<&Path, Vec<TestResult>> = HashMap::new();

  for run_fn in run {
    let result = run_fn(&code);
    results.entry(file_path).or_default().push(result);
  }

  let results = results
    .into_values()
    .map(|results| {
      results
        .into_iter()
        .map(|result| match result {
          TestResult::Node(node) => format!("{:#?}", node),
          TestResult::StyleSheet(stylesheet) => format!("{:#?}", stylesheet),
        })
        .collect_vec()
    })
    .flatten()
    .collect_vec();

  let mut settings = insta::Settings::clone_current();
  settings.set_prepend_module_to_snapshot(false);
  settings.set_omit_expression(true);
  settings.set_input_file(path);
  settings.bind(|| {
    for result in results {
      assert_snapshot!(result);
    }
  });
}

fn run_tests(path: &Path, run: &[&RunFn], module_path: &str) {
  let mut paths = Vec::new();
  for entry in std::fs::read_dir(path).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    if path.is_file() {
      paths.push(path);
    }
  }

  paths.sort();
  for path in paths {
    let name = path.to_str().unwrap().split(module_path).last().unwrap();
    run_test(name, &path, run, module_path);
  }
}

fn parse_html(code: &str) -> TestResult {
  let mut parser = HTMLParser::new(code.to_string());
  TestResult::Node(parser.parse())
}

fn parse_css(code: &str) -> TestResult {
  let mut parser = CSSParser::new(code.to_string());
  TestResult::StyleSheet(parser.parse())
}

fn run_multiple_dirs(path: &Path, run: &[&RunFn], module_path: &str) {
  let root = path.to_str().unwrap();
  let walker = walkdir::WalkDir::new(path).into_iter();
  for entry in walker.filter_map(|e| e.ok()) {
    let path = entry.path();
    if path.is_file() {
      let name = path.to_str().unwrap().split(root).last().unwrap();
      eprintln!("Running test: {}...", name);
      run_test(name, &path, run, module_path);
    }
  }
}

#[test]
fn test_html_parser() {
  let path = Path::new(HTML_TESTS_PATH);
  run_multiple_dirs(path, &[&parse_html], HTML_TESTS_PATH);
}

#[test]
fn test_css_parser() {
  let path = Path::new(CSS_TESTS_PATH);
  run_multiple_dirs(path, &[&parse_css], CSS_TESTS_PATH);
}
