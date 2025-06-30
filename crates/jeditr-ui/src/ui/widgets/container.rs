pub struct ContainerState {
    pub base: ElementState,
    pub is_hovered: bool,
}

impl ContainerState {
    pub fn new(children: Vec<Widget>, base: ElementState, is_hovered: bool) -> Self {
        Self {
            children,
            base,
            is_hovered,
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
