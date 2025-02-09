use std::ops::Index;

use wgpu_renderer::{label::LabelMesh, renderer::WgpuRendererInterface, vertex_texture_shader::VertexTextureShaderDraw};

use crate::widget::widget_renderer::{LabelResult, WidgetRenderer};


pub struct WidgetElementLabel {
    pub is_visible: bool,
    // pub update_position: bool,
    // pub update_text: bool,

    pub x: u32,
    pub y: u32,

    pub label: wgpu_renderer::label::Label,
    pub instance: wgpu_renderer::vertex_texture_shader::Instance,

    pub mesh: LabelMesh,
}

pub struct WgpuWidgetRendererStorage {
    pub labels: Vec<WidgetElementLabel>,
}

impl WgpuWidgetRendererStorage {
    pub fn new() -> Self 
    {
        let elements= Vec::new();
    
        Self { labels: elements }
    }

    pub fn draw<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>) {
        for elem in &self.labels {
            elem.mesh.draw(render_pass);
        }
    }
}

pub struct WgpuWidgetRenderer<'a>  {
    pub storage: &'a mut WgpuWidgetRendererStorage,

    // wgpu renderer
    pub font: &'a rusttype::Font<'static>,
    pub wgpu_renderer: &'a mut dyn wgpu_renderer::renderer::WgpuRendererInterface,
    pub texture_bind_group_layout: &'a wgpu_renderer::vertex_texture_shader::TextureBindGroupLayout,
    
}

impl<'a> WidgetRenderer for WgpuWidgetRenderer<'a> {   
    fn create_label(&mut self, text: &str, scale: u32) -> LabelResult {

        // member variables
        let font = self.font;
        let wgpu_renderer = &mut self.wgpu_renderer;
        let texture_bind_group_layout = self.texture_bind_group_layout;

        // WgpuWidget elements
        let is_visible = true;

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

        let width = label.width();
        let height = label.height();


        // create element entry
        let element = WidgetElementLabel {
            is_visible,
            x,
            y,
            label,
            instance,
            mesh,
        };

        let index = self.storage.labels.len();
        self.storage.labels.push(element);

        LabelResult{
            index,
            height,
            width,
        }

    }
    
    fn set_label_text(&mut self, index: usize, text: &str) {
        // member variables
        let font = self.font;
        let wgpu_renderer = &mut self.wgpu_renderer;

        let elem = &mut self.storage.labels[index];

        elem.label.update(font, text);
        elem.mesh.update_texture(wgpu_renderer.queue(), elem.label.get_image());
    }
    
    fn set_label_position(&mut self, index: usize, x: u32, y: u32) {

        let elem = &mut self.storage.labels[index];

        elem.x = x;
        elem.y = y;

        elem.instance.position.x = elem.x as f32;
        elem.instance.position.y = elem.y as f32;
        elem.mesh.update_instance_buffer(self.wgpu_renderer.queue(), &elem.instance);
    }
    
    fn set_label_visible(&mut self, index: usize, is_visible: bool) {
        let elem = &mut self.storage.labels[index];

        elem.is_visible = is_visible;
    }
}