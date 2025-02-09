use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::widget::widget_renderer::WidgetRenderer;

use super::layout::{self, Layout};
use super::wgpu_gui::{GuiElementContainer, LayoutElements, WgpuGui};
use super::{mouse_event::MouseEvent, size::Size};

struct GuiEventResult<TMessage> {
    pub mouse_event_consumed: bool,
    pub events_generated: [Option<TMessage>; 2],
}

pub trait GuiElement<TMessage> {    
    fn size(&mut self) -> Size;

    fn mouse_event(&mut self, mouse_event: &MouseEvent, model: &mut dyn FnMut(TMessage)) -> bool;

    fn update(&mut self, widget_renderer: &mut dyn WidgetRenderer);

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size);
}


pub trait GuiElementVisitor<TMessage> {
    fn visit(&mut self, layout: &mut Layout, elements: &mut [&mut dyn GuiElement<TMessage>]);
}

pub struct GuiElementVisitorClosure<'a, TMessage>  {
    func: &'a mut dyn FnMut(&mut Layout, &mut [&mut dyn GuiElement<TMessage>]),
}

impl<'a, TMessage> GuiElementVisitor<TMessage>  for GuiElementVisitorClosure<'a, TMessage> {
    fn visit(&mut self, layout: &mut Layout, elements: &mut [&mut dyn GuiElement<TMessage>]) {
        (self.func)(layout, elements);
    }
}

pub trait GuiElementSubView
{
    type TMessage;
    type TSubMessage;

    fn visit_elements(&mut self, visitor: &mut dyn GuiElementVisitor<Self::TSubMessage>);

    fn get_event_conversion_function(&self) -> fn(Self::TSubMessage) -> Self::TMessage;

}

impl<T> GuiElement<T::TMessage> for T where T: GuiElementSubView {
    fn mouse_event(&mut self, mouse_event: &MouseEvent, model: &mut dyn FnMut(T::TMessage)) -> bool
    {
        let conversion_function = self.get_event_conversion_function();
        let mut model_converted = |sub_message: T::TSubMessage| {
            model(conversion_function(sub_message));
        };

        let mut visitor = GuiElementVisitorClosure {
            func: &mut |layout: &mut Layout, elements: &mut [&mut dyn GuiElement<_>]| 
            {
                layout.mouse_event(mouse_event, &mut model_converted, elements);
            }
        };

        self.visit_elements(&mut visitor);

        true
    }

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size)
    {
        let mut visitor = GuiElementVisitorClosure {
            func: &mut |layout: &mut Layout, elements: &mut [&mut dyn GuiElement<_>]| 
            {
                layout.resize(widget_renderer, abs_x, abs_y, size, elements);
            }
        };

        self.visit_elements(&mut visitor);

    }

    fn update(&mut self, widget_renderer: &mut dyn WidgetRenderer) {
        let mut visitor = GuiElementVisitorClosure {
            func: &mut |layout: &mut Layout, elements: &mut [&mut dyn GuiElement<_>]| 
            {
                layout.update(widget_renderer, elements);
            }
        };

        self.visit_elements(&mut visitor);
    }
    
    fn size(&mut self) -> Size {
        let mut res = Size::new();

        let mut visitor = GuiElementVisitorClosure {
            func: &mut |layout: &mut Layout, elements: &mut [&mut dyn GuiElement<_>]| 
            {
                res = layout.size();
            }
        };

        self.visit_elements(&mut visitor);

        res
    }
    

}
