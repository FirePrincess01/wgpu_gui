use crate::core::{gui_functions::GuiElement, mouse_event::MouseEvent, size::Size};





pub struct Text {
    _space: u32,
    size: u32,
    value: i32,
}

impl Text {
    pub fn from_space(space: u32) -> Self {
        Self {
            _space: space,
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

impl<TMessage> GuiElement<TMessage> for Text {
    fn mouse_event(&mut self, 
        _mouse_event: &MouseEvent, 
        _models: &mut dyn FnMut(TMessage)) -> bool {
        todo!()
    }

    fn update_device(&mut self) {
        todo!()
    }

    fn resize(&mut self, _abs_x: u32, _abs_y: u32, _size: Size) {
        todo!()
    }

    fn draw(&mut self) {
        todo!()
    }
    
    fn size(&mut self) -> Size {
        todo!()
    }

}