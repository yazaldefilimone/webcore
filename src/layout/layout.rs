use crate::css::{DeclarationValue, StyledNode};
// =============================================
// https://www.w3.org/TR/CSS2/visuren.html#box-gen
//

#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
  width: f32,
  height: f32,
  y: f32,
  x: f32,
}
pub enum BoxModalType<'a> {
  BlockNode(&'a StyledNode<'a>),
  InlineNode(&'a StyledNode<'a>),
  AnonymousBlock,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct EdgeSizes {
  pub left: f32,
  pub right: f32,
  pub top: f32,
  pub bottom: f32,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Dimensions {
  /// Position of the content area relative to the document origin:
  pub content: Rect,
  // Surrounding edges:
  pub padding: EdgeSizes,
  pub border: EdgeSizes,
  pub margin: EdgeSizes,
}

pub struct LayoutBox<'a> {
  dimensions: Dimensions,
  box_modal_type: BoxModalType<'a>,
  children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
  fn new(box_modal_type: BoxModalType<'a>) -> Self {
    LayoutBox { dimensions: Dimensions::default(), box_modal_type, children: Vec::new() }
  }
  // Where a new inline child should go.
  fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
    match self.box_modal_type {
      BoxModalType::InlineNode(_) | BoxModalType::AnonymousBlock => self,
      BoxModalType::BlockNode(_) => {
        // If we've just generated an anonymous block box, keep using it.
        // Otherwise, create a new one.
        match &self.children.last() {
          Some(LayoutBox { box_modal_type: AnonymousBlock, .. }) => {}
          _ => self.children.push(LayoutBox::new(BoxModalType::AnonymousBlock)),
        }
        self.children.last_mut().unwrap()
      }
    }
  }
}

pub enum Display {
  Inline,
  Block,
  None,
}

impl<'a> StyledNode<'a> {
  /// Return the specified value of a property if it exists, otherwise `None`.
  pub fn value(&mut self, name: &str) -> Option<DeclarationValue> {
    self.specified_values.get(name).map(|value| value.clone())
  }

  /// Return the specified value of property `name`, or property `fallback_name` if that doesn't
  /// exist, or value `default` if neither does.
  pub fn lookup(&mut self, name: &str, fallback_name: &str, default: &DeclarationValue) -> DeclarationValue {
    let declaration = self.value(name);
    declaration.unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
  }

  /// The value of the `display` property (defaults to inline).
  pub fn display(&mut self) -> Display {
    match self.value("display") {
      Some(DeclarationValue::Keyword(keyword)) => match &*keyword {
        "block" => Display::Block,
        "none" => Display::None,
        _ => Display::Inline,
      },
      _ => Display::Inline,
    }
  }
}

// Build the tree of LayoutBoxes, but don't perform any layout calculations yet.
// fn create_layout_tree<'a>(style_node: &'a mut StyledNode<'a>) -> LayoutBox<'a> {
//   // Create the root box.
//   let box_modal = match style_node {
//     Block => BoxModalType::BlockNode(&style_node.clone()),
//     Inline => BoxModalType::InlineNode(style_node),
//     _ => panic!("Root node has display: none."),
//   };

//   let mut root = LayoutBox::new(box_modal);
//   return root;
//   // Create the descendant boxes.
// }
