use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}
#[derive(Debug)]
pub enum NodeType {
  Text(String),
  Element(Element),
}

pub type AtributeMapType = HashMap<String, String>;

#[derive(Debug)]
pub struct Element {
  pub tag_name: String,
  pub atributes: AtributeMapType,
}

pub fn create_text_node(text: String) -> Node {
  Node { children: Vec::new(), node_type: NodeType::Text(text) }
}
pub fn create_element_node(tag_name: String, atributes: AtributeMapType) -> Node {
  Node { children: Vec::new(), node_type: NodeType::Element(Element { tag_name, atributes }) }
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
}
