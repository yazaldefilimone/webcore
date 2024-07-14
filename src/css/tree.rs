use crate::css::{DeclarationValue, Rule, Selector, SimpleSelector, Specificity, StyleSheet};
use crate::dom::{Element, Node, NodeType};
use std::collections::HashMap;
// ==================================
// https://wiki.mozilla.org/Gecko:Key_Gecko_Structures_And_Invariants
// map from CSS property names to values.
// todo: make eayse to  suport of compound selectors, e.g. `div.test p` or `div.test > p`.
//
type PropertyMap = HashMap<String, DeclarationValue>;

// A node with associated style data.
#[derive(Debug, Clone)]
pub struct StyledNode<'a> {
  pub node: &'a Node, // pointer to a DOM node
  pub specified_values: PropertyMap,
  pub children: Vec<StyledNode<'a>>,
}

// http://www.w3.org/TR/CSS2/selector.html#pattern-matching
fn matches(elem: &Element, selector: &Selector) -> bool {
  match *selector {
    Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
  }
}

fn matches_simple_selector(elem: &Element, selector: &SimpleSelector) -> bool {
  // Check type selector
  if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
    return false;
  }

  // Check ID selector
  if selector.id.iter().any(|id| elem.id() != *id) {
    return false;
  }

  // Check class selectors
  let elem_classes = elem.classes();
  if selector.class.iter().any(|class| !elem_classes.contains(&**class)) {
    return false;
  }
  // We didn't find any non-matching selector components.
  return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);

// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &Element, rule: &'a Rule) -> Option<MatchedRule<'a>> {
  // Find the first (highest-specificity) matching selector.
  let mut selector_vector = rule.selectors.iter();
  let selector = selector_vector.find(|selector| matches(elem, selector));
  selector.map(|selector| (selector.specificity(), rule))
}

// Find all CSS rules that match the given element.
fn matching_rules<'a>(elem: &Element, styleheet: &'a StyleSheet) -> Vec<MatchedRule<'a>> {
  let rule_vector = styleheet.rules.iter();
  rule_vector.filter_map(|rule| match_rule(elem, rule)).collect()
}

// Apply styles to a single element, returning the specified values.
fn specified_values(elem: &Element, styleheet: &StyleSheet) -> PropertyMap {
  let mut values_map = HashMap::new();
  let mut rules_matched = matching_rules(elem, styleheet);
  // Go through the rules from lowest to highest specificity.
  rules_matched.sort_by(|&(right, _), &(left, _)| left.cmp(&right));
  for (_, rule) in rules_matched {
    for declaration in &rule.declarations {
      values_map.insert(declaration.name.clone(), declaration.value.clone());
    }
  }
  return values_map;
}

// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a StyleSheet) -> StyledNode<'a> {
  let specified_values = match root.node_type {
    NodeType::Element(ref elem) => specified_values(elem, stylesheet),
    NodeType::Text(_) => HashMap::new(),
  };

  let children = root.children.iter().map(|child| style_tree(child, stylesheet));

  StyledNode { node: root, specified_values, children: children.collect() }
}
