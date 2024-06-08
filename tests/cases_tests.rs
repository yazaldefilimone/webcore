use insta::assert_snapshot;
use itertools::Itertools;
use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};
use web_core::dom::Node;
use web_core::html::HTMLParser;

const TESTS_PATH: &str = "tests/cases/";

type RunFn = dyn Fn(&str) -> Node;

fn run_test(name: &str, path: &Path, run: &[&RunFn]) {
  println!("Running test: {}...", name);
  println!("path: {}", path.display());
  let code = std::fs::read_to_string(path).expect("Failed to read test file");
  let file_name = path.to_str().and_then(|path| path.rsplit_once(TESTS_PATH)).unwrap().1;
  let file_path = format!("{}{}", &TESTS_PATH[1..], file_name);
  let file_path = Path::new(&file_path);

  let mut results: HashMap<&Path, Vec<Node>> = HashMap::new();

  for run_fn in run {
    let result = run_fn(&code);
    results.entry(file_path).or_default().push(result);
  }

  let results = results
    .into_values()
    .map(|nodes| nodes.into_iter().map(|node| format!("{:#?}", node)).collect_vec())
    .flatten()
    .collect_vec();

  let mut settings = insta::Settings::clone_current();
  settings.set_prepend_module_to_snapshot(false);
  settings.set_omit_expression(true);
  settings.set_input_file(path);
  settings.bind(|| {
    for result in results {
      assert_snapshot!(file_name, result);
    }
  });
}

fn run_tests(path: &Path, run: &[&RunFn]) {
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
    let name = path.to_str().unwrap().split(TESTS_PATH).last().unwrap();
    run_test(name, &path, run);
  }
}

fn parse_html(code: &str) -> Node {
  let mut parser = HTMLParser::new(code.to_string());
  parser.parse()
}

fn run_multiple_dirs(path: &Path, run: &[&RunFn]) {
  let root = path.to_str().unwrap();
  let walker = walkdir::WalkDir::new(path).into_iter();
  for entry in walker.filter_map(|e| e.ok()) {
    let path = entry.path();
    if path.is_file() {
      let name = path.to_str().unwrap().split(root).last().unwrap();
      eprintln!("Running test: {}...", name);
      run_test(name, &path, run);
    }
  }
}

#[test]
fn test_html_parser() {
  let path = Path::new(TESTS_PATH);
  run_multiple_dirs(path, &[&parse_html]);
}
