use crate::core::{gui_functions::GuiElement, mouse_event, size::Size};
use wgpu_renderer::{self, renderer::WgpuRendererInterface, vertex_color_shader::instance, vertex_texture_shader::VertexTextureShaderDraw};

use super::{widget_factory::WidgetFactory, widget_renderer::WidgetRenderer};

pub struct Button<TMessage> {
    // _text: &'static str, 
    message_released: Option<TMessage>,

    // _label: wgpu_renderer::label::Label, 
    // label_mesh: wgpu_renderer::label::LabelMesh, 
    // instance: wgpu_renderer::vertex_texture_shader::Instance,
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
    pub fn new<'a>(render_index: usize, text_width: u32, text_height:u32,
        // widget_factory: &'a mut WidgetFactory,
        // text: &'static str, 
        // scale: u32,  
    ) -> Self 
    {
        // let font = widget_factory.font;
        // let wgpu_renderer: &'a mut dyn WgpuRendererInterface = widget_factory.wgpu_renderer;
        // let texture_bind_group_layout = widget_factory.texture_bind_group_layout;

        // let instance = wgpu_renderer::vertex_texture_shader::Instance::zero();

        // let label = wgpu_renderer::label::Label::new(font, scale as f32, text);
        // let label_mesh = wgpu_renderer::label::LabelMesh::new(wgpu_renderer, label.get_image(), texture_bind_group_layout, &instance);

        let boarder = 5;
        let width = text_width + 2 * boarder;
        let height = text_height + 2 * boarder;

        let abs_x = 0;
        let abs_y = 0;

        let pressed = false;


        Self {
            // _text: text,
            message_released: None,
            // _label: label,
            // label_mesh,
            // instance,
            render_index,

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
    fn mouse_event(&mut self, mouse_event: &crate::core::mouse_event::MouseEvent, model: &mut dyn FnMut(TMessage)) -> bool {
        if self.is_inside(mouse_event.x, mouse_event.y) {
            // println!("x:{}, y:{}", mouse_event.x, mouse_event.y);
            if self.pressed && !mouse_event.is_pressed {
                match self.message_released {
                    Some(message_released) => model(message_released),
                    None => {}
                }
            }

            self.pressed = mouse_event.is_pressed;

            return true
        }

        false
    }

    // fn update_device(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface) {
    //     self.instance.position.x = self.abs_x as f32;
    //     self.instance.position.y = self.abs_y as f32;
    //     self.label_mesh.update_instance_buffer(wgpu_renderer.queue(), &self.instance);
    // }

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, _size: Size) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;
        widget_renderer.set_position(self.render_index, abs_x, abs_y);
    }

    // fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
    //     self.label_mesh.draw(render_pass);
    // }
    
    fn size(&mut self) -> Size {
        todo!()
    }

}