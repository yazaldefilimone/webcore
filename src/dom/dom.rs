use std::collections::HashMap;

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
