use std::ops::Index;

use wgpu_renderer::{label::LabelMesh, renderer::WgpuRendererInterface, vertex_texture_shader::VertexTextureShaderDraw};

use crate::widget::widget_renderer::WidgetRenderer;


pub struct WidgetElement {
    pub is_visible: bool,
    pub update_pending: bool,

    pub x: u32,
    pub y: u32,

    pub text: &'static str,
    pub label: wgpu_renderer::label::Label,
    pub instance: wgpu_renderer::vertex_texture_shader::Instance,

    pub mesh: LabelMesh,
}

pub struct WgpuWidgetRenderer {
    elements: Vec<WidgetElement>,
}

impl WgpuWidgetRenderer {
    pub fn new() -> Self 
    {
        let elements= Vec::new();
    
        Self { elements }
    }

    pub fn add_element(&mut self, element: WidgetElement) -> usize {
        let index = self.elements.len();
        self.elements.push(element);
        index // return index
    }

    pub fn update(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface) {
        for elem in &mut self.elements {
            if elem.update_pending {
                elem.instance.position.x = elem.x as f32;
                elem.instance.position.y = elem.y as f32;
                elem.mesh.update_instance_buffer(wgpu_renderer.queue(), &elem.instance);
                elem.update_pending = false;
            }
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for elem in &self.elements {
            elem.mesh.draw(render_pass);
        }
    }
}

impl WidgetRenderer for WgpuWidgetRenderer {
    fn set_visible(&mut self, index: usize, is_visible: bool) {
        self.elements[index].is_visible = is_visible;
    }

    fn set_position(&mut self, index: usize, x: u32, y: u32) {
        self.elements[index].x = x;
        self.elements[index].y = y;
        self.elements[index].update_pending = true;
    }
}