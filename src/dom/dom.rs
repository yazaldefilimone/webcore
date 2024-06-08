use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}
#[derive(Debug, Clone)]
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

  pub fn classes(&self) -> HashSet<String> {
    let mut classes = HashSet::new();
    for class in self.atributes.get("class").unwrap().split(" ") {
      classes.insert(class.to_string());
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
