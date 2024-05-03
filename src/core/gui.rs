use super::{gui_element::GuiElement, gui_functions::{GuiFunctions, GuiMessageFunctions}, gui_message::GuiMessage, layout::Layout, mouse_event::MouseEvent, size::Size};

pub trait Gui {
    type TMessage;
    fn layout(&mut self, f: &mut dyn FnMut(&mut Layout, &mut [&mut (dyn GuiFunctions<Self::TMessage>)]));
}


impl <T:Gui> GuiFunctions<T::TMessage> for T {
    fn mouse_event(&mut self, mouse_event: &MouseEvent, models: &mut [&mut dyn GuiMessage<T::TMessage>]) -> bool {
        let mut res = false;
        self.layout(&mut |layout: &mut Layout, elems: &mut [&mut dyn GuiFunctions<T::TMessage>]| {
            res = layout.mouse_event(elems, mouse_event, models);
        });
        res
    }

    fn update_device(&mut self) {
        self.layout(&mut |layout: &mut Layout, elems: &mut [&mut dyn GuiFunctions<T::TMessage>]| {
            for elem in elems {
                elem.update_device();
            }
        });
    }

    fn resize(&mut self, size: Size) {
        self.layout(&mut |layout: &mut Layout, elems: &mut [&mut dyn GuiFunctions<T::TMessage>]| {
            layout.resize(elems, size)
        });
    }

    fn draw(&mut self) {
        self.layout(&mut |layout: &mut Layout, elems: &mut [&mut dyn GuiFunctions<T::TMessage>]| {
            for elem in elems {
                elem.draw();
            }
        });
    }
}