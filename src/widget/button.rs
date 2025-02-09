use crate::core::{gui_functions::GuiElement, size::Size};

use super::widget_renderer::WidgetRenderer;

pub struct Button<TMessage> {
    message_released: Option<TMessage>,
    render_index: usize,

    // geometry
    width: u32,
    height: u32,
    boarder: u32,

    // position on the screen
    abs_x: u32,
    abs_y: u32,

    // current state
    pressed: bool,

}

impl<TMessage> Button<TMessage>
{
    pub fn new<'a>(renderer: &'a mut dyn WidgetRenderer,
        text: &'static str, 
        scale: u32,
    ) -> Self 
    {
        let res = renderer.create_label(text, scale);

        let boarder = 5;
        let width = res.width + 2 * boarder;
        let height = res.height + 2 * boarder;

        let abs_x = 0;
        let abs_y = 0;

        let pressed = false;

        Self {
            message_released: None,
            render_index: res.index,

            boarder,
            width,
            height,

            abs_x,
            abs_y,

            pressed,
        }
    }

    pub fn on_released(mut self, message: TMessage) -> Self {
        self.message_released = Some(message);
        self
    }

    fn is_inside(&self, abs_x: u32, abs_y: u32) -> bool {
        abs_x >= self.abs_x && abs_x <= self.abs_x + self.width  &&
        abs_y >= self.abs_y && abs_y <= self.abs_y + self.height
    }
}

impl<TMessage> GuiElement<TMessage> for Button<TMessage> where TMessage: Copy, {
    fn mouse_event(&mut self, 
        mouse_event: &crate::core::mouse_event::MouseEvent, 
        event_result: &mut crate::core::gui_functions::GuiEventResult<TMessage>
    ) -> bool {
        if self.is_inside(mouse_event.x, mouse_event.y) {
            // println!("x:{}, y:{}", mouse_event.x, mouse_event.y);
            if self.pressed && !mouse_event.is_pressed {
                match self.message_released {
                    Some(message_released) => {
                        // println!("button.rs");
                        event_result.add(message_released);
                    },
                    None => {}
                }
            }

            self.pressed = mouse_event.is_pressed;

            return true
        }

        false
    }

    fn update(&mut self, widget_renderer: &mut dyn WidgetRenderer) {
        // nothing to do
    }

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, _size: Size) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;
        widget_renderer.set_label_position(self.render_index, abs_x, abs_y);
    }
    
    fn size(&mut self) -> Size {
        Size{
            height: self.height,
            width: self.width,
        }
    }
}