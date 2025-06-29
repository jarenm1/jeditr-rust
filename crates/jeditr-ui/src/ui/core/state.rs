use std::collections::HashMap;
use uuid::Uuid;

use super::Style;

use crate::ui::widgets::button::ButtonState;
use super::style::{Color, Padding, Border, BorderRadius};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementId(Uuid);

impl ElementId {
  pub fn new() -> Self {
    ElementId(Uuid::new_v4())
  }
}

#[derive(Debug, Clone)]
pub struct ElementState {
  pub id: ElementId,
  pub children: Vec<ElementId>,
  pub visible: bool,
  pub style: Style,
}

impl ElementState {
  pub fn new() -> Self {
    Self {
      id: ElementId::new(),
      children: Vec::new(),
      visible: true,
      style: Style::default(),
    }
  }
  
  pub fn padding(mut self, padding: Padding) -> Self {
    self.style.padding = padding;
    self
  }
  
  // TODO: Implement other chaining styles

}

#[derive(Debug, Clone)]
pub enum ElementData {
  Button(ButtonState),
}

#[derive(Debug, Clone)]
pub struct UiState {
  pub root: Option<ElementId>,
  pub elements: HashMap<ElementId, ElementData>,
}

impl UiState {
  pub fn new() -> Self {
    UiState {
      root: None,
      elements: HashMap::new(),
    }
  }
  pub fn with_root(&mut self, id: ElementId) -> &mut Self {
    self.root = Some(id);
    self
  }
  pub fn add_element(&mut self, id: ElementId, data: ElementData) -> &mut Self {
    self.elements.insert(id, data);
    self
  }
}
