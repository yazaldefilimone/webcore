pub struct StyleSheet {
  pub rules: Vec<Rule>,
}

pub struct Rule {
  pub selectors: Vec<Selector>,
  pub declarations: Vec<Declaration>,
}

pub struct Declaration {
  pub name: String,
  pub value: DeclarationValue,
}

pub enum DeclarationValue {
  Keyword(String),
  Length(f32, UnitValue),
  Color(ColorValue),
}

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
}

impl Copy for ColorValue {}

pub enum Selector {
  // .selector
  Simple(SimpleSelector),
}
pub struct SimpleSelector {
  pub tag_name: Option<String>,
  pub id: Option<String>,
  pub class: Vec<String>,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
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
