#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    pub fn uniform(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn horizontal_vertical(h: f32, v: f32) -> Self {
        Self {
            top: v,
            right: h,
            bottom: v,
            left: h,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl BorderRadius {
    pub fn uniform(value: f32) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_left: value,
            bottom_right: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Border {
    pub color: Color,
    pub width: f32,
    pub radius: BorderRadius,
}

impl Border {
    pub fn new(color: Color, width: f32, radius: BorderRadius) -> Self {
        Self {
            color,
            width,
            radius,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Style {
    pub padding: Padding,
    pub background: Option<Color>,
    pub border: Option<Border>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            padding: Padding::uniform(0.0),
            background: Some(Color::BLACK),
            border: Some(Border::new(Color::BLACK, 1.0, BorderRadius::uniform(2.0))),
        }
    }
}
