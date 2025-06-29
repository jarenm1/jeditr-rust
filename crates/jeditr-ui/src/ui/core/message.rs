use winit::keyboard::KeyCode;

use super::state::ElementId;

pub enum UiMessage {
  Click(ElementId),
  Hover { id: ElementId, hovered: bool },
  KeyPress { id: ElementId, key: KeyCode },
  TextInput { id: ElementId, text: String },
  
  Resize { id: ElementId, width: u32, height: u32 },
}