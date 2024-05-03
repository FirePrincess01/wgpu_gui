use super::size::Size;



pub trait GuiElement {
    fn size(&self) -> Size;
}

