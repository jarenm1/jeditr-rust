use crate::ui::core::{
    ElementState,
    style::{Color, Padding, Border, BorderRadius},
};

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
    pub fn background_color(mut self, color: Color) -> Self {
        self.base.style.background = Some(color);
        self
    }
    pub fn border(mut self, border: Border) -> Self {
        self.base.style.border = Some(border);
        self
    }
    pub fn with_border_radius(mut self, border_radius: BorderRadius) -> Self {
        if let Some(border) = &mut self.base.style.border {
            border.radius = border_radius;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::core::style::{Color, Padding, Border, BorderRadius};

    #[test]
    fn test_button_new() {
        let button = ButtonState::new("Click me".to_string());
        assert_eq!(button.label, "Click me");
        assert_eq!(button.is_pressed, false);
        assert_eq!(button.is_hovered, false);
    }

    #[test]
    fn test_button_padding() {
        let padding = Padding::uniform(10.0);
        let button = ButtonState::new("Click me".to_string())
            .padding(padding.clone());
        
        assert_eq!(button.base.style.padding.top, 10.0);
        assert_eq!(button.base.style.padding.right, 10.0);
        assert_eq!(button.base.style.padding.bottom, 10.0);
        assert_eq!(button.base.style.padding.left, 10.0);
    }

    #[test]
    fn test_button_background_color() {
        let color = Color::rgb(1.0, 0.0, 0.0);
        let button = ButtonState::new("Click me".to_string())
            .background_color(color);
        
        let bg = button.base.style.background.unwrap();
        assert_eq!(bg.r, 1.0);
        assert_eq!(bg.g, 0.0);
        assert_eq!(bg.b, 0.0);
        assert_eq!(bg.a, 1.0);
    }

    #[test]
    fn test_button_border() {
        let border = Border::new(
            Color::BLACK,
            2.0,
            BorderRadius::uniform(5.0)
        );
        let button = ButtonState::new("Click me".to_string())
            .border(border);
        
        let border = button.base.style.border.unwrap();
        assert_eq!(border.width, 2.0);
        assert_eq!(border.radius.top_left, 5.0);
    }

    #[test]
    fn test_button_with_border_radius() {
        let border = Border::new(
            Color::BLACK,
            1.0,
            BorderRadius::uniform(2.0)
        );
        let border_radius = BorderRadius::uniform(8.0);
        let button = ButtonState::new("Click me".to_string())
            .border(border)
            .with_border_radius(border_radius);
        
        let border = button.base.style.border.unwrap();
        assert_eq!(border.radius.top_left, 8.0);
        assert_eq!(border.radius.top_right, 8.0);
        assert_eq!(border.radius.bottom_left, 8.0);
        assert_eq!(border.radius.bottom_right, 8.0);
    }

    #[test]
    fn test_button_method_chaining() {
        let button = ButtonState::new("Click me".to_string())
            .padding(Padding::uniform(5.0))
            .background_color(Color::WHITE)
            .border(Border::new(
                Color::BLACK,
                1.0,
                BorderRadius::uniform(3.0)
            ))
            .with_border_radius(BorderRadius::uniform(4.0));
        
        assert_eq!(button.label, "Click me");
        assert_eq!(button.base.style.padding.top, 5.0);
        
        let bg = button.base.style.background.unwrap();
        assert_eq!(bg.r, 1.0);
        assert_eq!(bg.g, 1.0);
        assert_eq!(bg.b, 1.0);
        
        let border = button.base.style.border.unwrap();
        assert_eq!(border.width, 1.0);
        assert_eq!(border.radius.top_left, 4.0);
    }
}
