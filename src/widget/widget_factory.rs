

// pub struct WidgetFactory<'a> {
//     pub font: &'a rusttype::Font<'static>, 
//     pub wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
//     pub texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,
// }

// impl<'a> WidgetFactory<'a> {
//     pub fn new(font: &'a rusttype::Font<'static>, wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface, texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout) -> Self {
//         Self { font, wgpu_renderer, texture_bind_group_layout }
//     }
// }

use super::button::Button;


pub trait WidgetFactory<TMessage> {
    fn button(&mut self, text: &'static str, scale: u32, message: TMessage) -> Button<TMessage>;
    fn text_from_space(&mut self, space: u32, scale: u32);
}