
use crate::widget::widget_renderer::WidgetRenderer;

use super::layout::Layout;
use super::{mouse_event::MouseEvent, size::Size};

pub struct GuiEventResult<TMessage> {
    pub events: [Option<TMessage>; 2],
}

impl<TMessage> GuiEventResult<TMessage> {
    pub fn new() -> Self {
        let events = [None, None];

        Self { events }
    }

    pub fn add(&mut self, event: TMessage) 
        where TMessage: Copy 
    {
        for elem in &mut self.events {
            if elem.is_none() {
                *elem = Some(event);
                break;
            }
        }
    }
}


pub trait GuiElement<TMessage> {    
    fn size(&mut self) -> Size;

    fn mouse_event(&mut self, mouse_event: &MouseEvent, event_result: &mut GuiEventResult<TMessage>) -> bool;

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
    type TMessage: Copy;
    type TSubMessage: Copy;

    fn visit_elements(&mut self, visitor: &mut dyn GuiElementVisitor<Self::TSubMessage>);

    fn on_event(&mut self, event: Self::TSubMessage) -> Self::TMessage;

}

impl<T> GuiElement<T::TMessage> for T where T: GuiElementSubView {
    fn mouse_event(&mut self, mouse_event: &MouseEvent, event_result: &mut GuiEventResult<T::TMessage>)
     -> bool
    {
        // create result container
        let mut event_result_1: GuiEventResult<T::TSubMessage> = GuiEventResult::new();
        let mut res = false;

        // visit all gui elements
        let mut visitor = GuiElementVisitorClosure {
            func: &mut |layout: &mut Layout, elements: &mut [&mut dyn GuiElement<_>]| 
            {
                res = layout.mouse_event(mouse_event, &mut event_result_1, elements);
            }
        };

        self.visit_elements(&mut visitor);

        // convert events
        for event in event_result_1.events {
            match event {
                Some(event) =>  {
                    let event_converted = self.on_event(event);
                    event_result.add(event_converted);
                },
                None => {},
            }            
        }

        res
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
            func: &mut |layout: &mut Layout, _elements: &mut [&mut dyn GuiElement<_>]| 
            {
                res = layout.size();
            }
        };

        self.visit_elements(&mut visitor);

        res
    }
    

}



pub trait GuiElementEvent {
    type TMessage;
    type TSubMessage;

    fn on_event(&mut self, message: Self::TMessage) -> Self::TSubMessage;
}