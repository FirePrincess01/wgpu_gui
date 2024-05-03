

use super::{gui_element::GuiElement, gui_functions::{GuiFunctions, GuiMessageFunctions}, gui_message::GuiMessage, mouse_event::MouseEvent, size::Size};

pub enum Alignment {
    Center, 
    Left,
    Right
}

pub enum LayoutKind {
    Horizontal,
    Vertical,
}

pub struct Layout {
    size: Size,
    alignment: Alignment,
    layout: LayoutKind,
}

impl Layout 
{
    pub fn new() -> Self {
        Self {
            size: Size::new(),
            alignment: Alignment::Center,
            layout: LayoutKind::Vertical,
        }
    }

    pub fn horizontal_layout(mut self) -> Self {
        self.layout = LayoutKind::Vertical;
        self
    }

    pub fn align(mut self, alignment: Alignment) -> Layout {
        self.alignment = alignment;
        self
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn is_inside(&self, x: u32, y: u32) -> bool {
        false
    }

    pub fn mouse_event<TMessage>(&self, 
        elems: &mut [&mut dyn GuiFunctions<TMessage>], 
        mouse_event: &MouseEvent, 
        models: &mut [&mut dyn GuiMessage<TMessage>]
    ) -> bool {
        let mut res = false;
        for elem in elems {
            res = res || elem.mouse_event(mouse_event, models);
        }
        res
    }

    pub fn resize<TMessage>(&self, 
        elems: &mut [&mut dyn GuiFunctions<TMessage>], 
        size: Size) 
    {
        for elem in elems {
            elem.resize(size);
        }
    }
}