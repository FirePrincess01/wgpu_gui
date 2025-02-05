use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::core::{gui_functions::GuiElement, mouse_event::MouseEvent, size::Size};

use super::widget_renderer::WidgetRenderer;





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

    // fn update_device(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface) {
    //     todo!()
    // }

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, _abs_x: u32, _abs_y: u32, _size: Size) {
        todo!()
    }

    // fn draw<'a>(&mut self, render_pass: &mut wgpu::RenderPass<'a>) {
    //     todo!()
    // }
    
    fn size(&mut self) -> Size {
        todo!()
    }

}