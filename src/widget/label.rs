use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::core::{gui_functions::GuiElement, mouse_event::MouseEvent, size::Size};

use super::widget_renderer::WidgetRenderer;

pub struct Label {
    text: String,

    render_index: usize,

    // geometry
    width: u32,
    height: u32,
    boarder: u32,

    // position on the screen
    abs_x: u32,
    abs_y: u32,

    // current state
    update_pending: bool,
}

impl  Label {
    pub fn new<'b>(renderer: &'b mut dyn WidgetRenderer, text: &'static str, scale: u32) -> Self {
        let res = renderer.create_label(text, scale);

        let boarder = 5;
        let width = res.width + 2 * boarder;
        let height = res.height + 2 * boarder;

        let abs_x = 0;
        let abs_y = 0;

        Self {
            text: String::from(text),

            render_index: res.index,

            boarder,
            width,
            height,

            abs_x,
            abs_y,

            update_pending: false,
        }
    }

    pub fn set(&mut self, text: String) {
        self.text = text;
        self.update_pending = true;
    }

    fn is_inside(&self, abs_x: u32, abs_y: u32) -> bool {
        abs_x >= self.abs_x
            && abs_x <= self.abs_x + self.width
            && abs_y >= self.abs_y
            && abs_y <= self.abs_y + self.height
    }
}

impl<TMessage> GuiElement<TMessage> for Label {
    fn mouse_event(&mut self, 
        mouse_event: &crate::core::mouse_event::MouseEvent, 
        event_result: &mut crate::core::gui_functions::GuiEventResult<TMessage>
    ) -> bool {
        if self.is_inside(mouse_event.x, mouse_event.y) {
            // println!("x:{}, y:{}", mouse_event.x, mouse_event.y);
            return true;
        }

        false
    }

    fn update(&mut self, widget_renderer: &mut dyn WidgetRenderer) {
        if self.update_pending {
            widget_renderer.set_label_text(self.render_index, &self.text);
            self.update_pending = false;
        }
    }

    fn resize(
        &mut self,
        widget_renderer: &mut dyn WidgetRenderer,
        abs_x: u32,
        abs_y: u32,
        _size: Size,
    ) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;
        widget_renderer.set_label_position(self.render_index, abs_x, abs_y);
    }

    fn size(&mut self) -> Size {
        Size {
            height: self.height,
            width: self.width,
        }
    }
}
