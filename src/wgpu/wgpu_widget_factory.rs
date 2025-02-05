use crate::widget::widget_factory::WidgetFactory;

use super::wgpu_widget_renderer::{WgpuWidgetRenderer, WidgetElement};

pub struct WgpuWidgetFactory<'a> {
    // wgpu renderer
    pub font: &'a rusttype::Font<'static>,
    pub wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
    pub texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,

    // widget renderer (device storage)
    pub widget_render: &'a mut WgpuWidgetRenderer,
}

impl<'a> WgpuWidgetFactory<'a> {
    pub fn new(
        font: &'a rusttype::Font<'static>,
        wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
        texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,
        widget_renderer: &'a mut WgpuWidgetRenderer,
    ) -> Self {
        Self {
            font,
            wgpu_renderer,
            texture_bind_group_layout,
            widget_render: widget_renderer
        }
    }
}

impl<'a, TMessage> WidgetFactory<TMessage> for WgpuWidgetFactory<'a> {
    fn button(
        &mut self,
        text: &'static str,
        scale: u32,
        message: TMessage,
    ) -> crate::widget::button::Button<TMessage> {
        // member variables
        let font = self.font;
        let wgpu_renderer = &mut self.wgpu_renderer;
        let texture_bind_group_layout = self.texture_bind_group_layout;

        // WgpuWidget elements
        let is_visible = true;
        let update_pending = true;

        let x = 0;
        let y = 0;

        let label = wgpu_renderer::label::Label::new(font, scale as f32, text);
        let instance = wgpu_renderer::vertex_texture_shader::Instance::zero();

        let mesh = wgpu_renderer::label::LabelMesh::new(
            *wgpu_renderer,
            label.get_image(),
            texture_bind_group_layout,
            &instance,
        );

        let text_width = label.width();
        let text_height = label.height();

        // create visual element
        let element = WidgetElement {
            is_visible,
            update_pending,
            x,
            y,
            text,
            label,
            instance,
            mesh,
        };

        let render_index = self.widget_render.add_element(element);

        // create physical element
        let button = crate::widget::button::Button::new(render_index, text_width, text_height)
            .on_released(message);

        button

    }

    fn text_from_space(&mut self, space: u32, scale: u32) {
        todo!()
    }
}
