use crate::core::{gui_functions::GuiElement, size::Size};
use wgpu_renderer::{self, renderer::WgpuRendererInterface};

use super::widget_factory::WidgetFactory;

pub struct Button<TMessage> {
    _text: &'static str, 
    message_released: Option<TMessage>,

    _label: wgpu_renderer::label::Label, 
    _label_mesh: wgpu_renderer::label::LabelMesh, 
}

impl<TMessage> Button<TMessage>
{
    pub fn new<'a>(widget_factory: &'a mut WidgetFactory,
        text: &'static str, 
        scale: u32,  
    ) -> Self 
    {
        let font = widget_factory.font;
        let wgpu_renderer: &'a mut dyn WgpuRendererInterface = widget_factory.wgpu_renderer;
        let texture_bind_group_layout = widget_factory.texture_bind_group_layout;

        let instance = wgpu_renderer::vertex_texture_shader::Instance::zero();

        let label = wgpu_renderer::label::Label::new(font, scale as f32, text);
        let label_mesh = wgpu_renderer::label::LabelMesh::new(wgpu_renderer, label.get_image(), texture_bind_group_layout, &instance);

        Self {
            _text: text,
            message_released: None,
            _label: label,
            _label_mesh: label_mesh,
        }
    }

    pub fn on_released(mut self, message: TMessage) -> Self {
        self.message_released = Some(message);
        self
    }
}

impl<TMessage> GuiElement<TMessage> for Button<TMessage> {
    fn mouse_event(&mut self, _mouse_event: &crate::core::mouse_event::MouseEvent, _model: &mut dyn FnMut(TMessage)) -> bool {
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