

pub struct WidgetFactory<'a> {
    pub font: &'a rusttype::Font<'static>, 
    pub wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
    pub texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,
}

impl<'a> WidgetFactory<'a> {
    pub fn new(font: &'a rusttype::Font<'static>, wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface, texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout) -> Self {
        Self { font, wgpu_renderer, texture_bind_group_layout }
    }
}

