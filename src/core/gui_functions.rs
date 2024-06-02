use super::layout::Layout;
use super::wgpu_gui::{LayoutElements, WgpuGui};
use super::{mouse_event::MouseEvent, size::Size};




pub trait GuiElement<TMessage> {    
    fn size(&mut self) -> Size;

    fn mouse_event(&mut self, mouse_event: &MouseEvent, model: &mut dyn FnMut(TMessage)) -> bool;

    fn update_device(&mut self);

    fn resize(&mut self, abs_x: u32, abs_y: u32, size: Size);

    fn draw(&mut self);
}


pub trait GuiElementSubView
{
    type TMessage;
    type TSubMessage;

    fn get_elements(&mut self, ui: &mut WgpuGui<Self::TSubMessage>);

    fn get_event_conversion_function(&self) -> fn(Self::TSubMessage) -> Self::TMessage;
}

impl<T> GuiElement<T::TMessage> for T where T: GuiElementSubView {
    fn mouse_event(&mut self, mouse_event: &MouseEvent, model: &mut dyn FnMut(T::TMessage)) -> bool
    {
        let conversion_function = self.get_event_conversion_function();
        let mut model_converted = |sub_message: T::TSubMessage| {
            model(conversion_function(sub_message));
        };

        self.get_elements(&mut WgpuGui::new(&mut |layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            layout.mouse_event(mouse_event, &mut model_converted, elements);
        }));

        true
    }

    fn update_device(&mut self)
    {
        self.get_elements(&mut WgpuGui::new(&mut |_layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            elements(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<T::TSubMessage>| {
                element.update_device();
            }));
        }));
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32, size: Size)
    {
        self.get_elements(&mut WgpuGui::new(&mut |layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            layout.resize(abs_x, abs_y, size, elements);
        }));
    }

    fn draw(&mut self)
    {
        self.get_elements(&mut WgpuGui::new(&mut |_layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            elements(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<T::TSubMessage>| {
                element.draw();
            }));
        }));
    }
    
    fn size(&mut self) -> Size {
        let mut res = Size::new();
        self.get_elements(&mut WgpuGui::new(&mut |layout: &mut Layout, _elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            res = layout.size();
        }));

        res
    }
}
