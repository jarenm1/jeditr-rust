
use crate::ui::core::{style::{Color, Padding}, ElementState};

#[derive(Debug, Clone)]
pub struct ButtonState {
    pub base: ElementState,
    pub label: String,
    pub is_pressed: bool,
    pub is_hovered: bool,
}

impl ButtonState {
    pub fn new(label: String) -> Self {
        ButtonState {
            base: ElementState::new(),
            label,
            is_pressed: false,
            is_hovered: false,
        }
    }
    
    pub fn padding(mut self, padding: Padding) -> Self {
        self.base.style.padding = padding;
        self
    }
    
    pub fn padding_uniform(mut self, padding: f32) -> Self {
        self.base.style.padding = Padding::uniform(padding);
        self
    }
    
    pub fn background(mut self, background: Color) -> Self {
        self.base.style.background(color);
        self
    }
}
