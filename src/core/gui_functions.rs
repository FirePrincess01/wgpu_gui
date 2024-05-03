use super::{mouse_event::MouseEvent, size::Size};

use super::gui_message::GuiMessage;

pub trait GuiMessageFunctions {
    type TMessage;
    
}



pub trait GuiFunctions<TMessage> {    
    fn mouse_event(&mut self, mouse_event: &MouseEvent, models: &mut [&mut dyn GuiMessage<TMessage>]) -> bool;

    fn update_device(&mut self);

    fn resize(&mut self, size: Size);

    fn draw(&mut self);
}
