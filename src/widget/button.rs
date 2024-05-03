use crate::core::{gui_element::GuiElement, gui_functions::GuiFunctions, size::Size};



pub struct Button<TMessage> {
    text: &'static str, 
    message_released: Option<TMessage>,
}

impl<TMessage> Button<TMessage>
{
    pub fn new(text: &'static str) -> Self 
    {
        Self {
            text,
            message_released: None,
        }
    }

    pub fn on_released(mut self, message: TMessage) -> Self {
        self.message_released = Some(message);
        self
    }
}

impl<TMessage> GuiFunctions<TMessage> for Button<TMessage> {
    fn mouse_event(&mut self, mouse_event: &crate::core::mouse_event::MouseEvent, models: &mut [&mut dyn crate::core::gui_message::GuiMessage<TMessage>]) -> bool {
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