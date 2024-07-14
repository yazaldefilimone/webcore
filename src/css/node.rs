#[derive(Debug, Clone)]
pub struct StyleSheet {
  pub rules: Vec<Rule>,
}
#[derive(Debug, Clone)]
pub struct Rule {
  pub selectors: Vec<Selector>,
  pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
  pub name: String,
  pub value: DeclarationValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationValue {
  Keyword(String),
  Length(f32, UnitValue),
  Color(ColorValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitValue {
  Px,
  Em,
  Rem,
  Percent,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorValue {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Copy for ColorValue {}

#[derive(Debug, Clone)]
pub enum Selector {
  // .selector
  Simple(SimpleSelector),
}
#[derive(Debug, Clone)]
pub struct SimpleSelector {
  pub tag_name: Option<String>,
  pub id: Option<String>,
  pub class: Vec<String>,
}

impl SimpleSelector {
  pub fn new() -> Self {
    SimpleSelector { tag_name: None, id: None, class: Vec::new() }
  }
}

pub type Specificity = (usize, usize, usize);

impl Selector {
  pub fn new() -> Self {
    Selector::Simple(SimpleSelector::new())
  }
  pub fn specificity(&self) -> Specificity {
    // http://www.w3.org/TR/selectors/#specificity
    let Selector::Simple(SimpleSelector { tag_name, id, class }) = self;
    return (tag_name.iter().count(), id.iter().count(), class.len());
  }
}

impl DeclarationValue {
  pub fn to_px(&self) -> f32 {
    match self {
      DeclarationValue::Length(px, _) => *px,
      // TODO: support other unit
      _ => 0.0,
    }
  }
}
