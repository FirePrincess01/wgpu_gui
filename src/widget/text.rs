use crate::core::{gui_functions::GuiFunctions, gui_message::GuiMessage, mouse_event::MouseEvent, size::Size};





pub struct Text {
    space: u32,
    size: u32,
    value: i32,
}

impl Text {
    pub fn from_space(space: u32) -> Self {
        Self {
            space: space,
            size: 0,
            value: 0,
        }
    }

    pub fn value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
}

impl<TMessage> GuiFunctions<TMessage> for Text {
    fn mouse_event(&mut self, 
        mouse_event: &MouseEvent, 
        models: &mut [&mut dyn GuiMessage<TMessage>]) -> bool {
        todo!()
    }

    fn update_device(&mut self) {
        todo!()
    }

    fn resize(&mut self, size: Size) {
        todo!()
    }

    fn draw(&mut self) {
        todo!()
    }
}