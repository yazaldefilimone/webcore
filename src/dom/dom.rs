#![allow(dead_code)]
use std::collections::{BTreeMap, HashMap, HashSet};

use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}
#[derive(Debug, Clone, serde::Serialize)]
pub enum NodeType {
  Text(String),
  Element(Element),
}

pub type AtributeMapType = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Element {
  pub tag_name: String,
  pub atributes: AtributeMapType,
}

// This is a better implementation of Serialize for HashMap? (we use it in snapshot tests)
impl Serialize for Element {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let ordered_attributes: BTreeMap<_, _> = self.atributes.iter().collect();
    let mut state = serializer.serialize_struct("Element", 2)?;
    state.serialize_field("tag_name", &self.tag_name)?;
    state.serialize_field("atributes", &ordered_attributes)?;
    state.end()
  }
}

// Doctype
//
#[derive(Debug, Clone, serde::Serialize)]
pub struct Doctype {
  pub name: String,
  pub public_id: Option<String>,
  pub system_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HtmlRoot {
  pub doctype: Option<Doctype>,
  pub children: Vec<Node>,
}

pub fn create_text(text: String) -> Node {
  Node { children: Vec::new(), node_type: NodeType::Text(text) }
}
pub fn create_element(tag_name: String, atributes: AtributeMapType, children: Vec<Node>) -> Node {
  Node { children, node_type: NodeType::Element(Element { tag_name, atributes }) }
}

impl Element {
  pub fn id(&self) -> String {
    self.atributes.get("id").unwrap().to_string()
  }

  pub fn classes(&self) -> HashSet<&str> {
    let mut classes = HashSet::new();
    for class in self.atributes.get("class").unwrap().split(" ") {
      classes.insert(class);
    }
    classes
  }

  pub fn has_class(&self, class_name: &str) -> bool {
    self.classes().contains(class_name)
  }
}

impl Node {
  pub fn tag_name(&self) -> String {
    match &self.node_type {
      NodeType::Text(_) => "".to_string(),
      NodeType::Element(element) => element.tag_name.clone(),
    }
  }
  pub fn text(&self) -> Option<String> {
    match &self.node_type {
      NodeType::Text(text) => Some(text.clone()),
      _ => None,
    }
  }

  pub fn get_children(&self) -> Vec<Node> {
    self.children.clone()
  }
}
