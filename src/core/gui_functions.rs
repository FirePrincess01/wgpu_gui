use wgpu_renderer::renderer::WgpuRendererInterface;

use crate::widget::widget_renderer::WidgetRenderer;

use super::layout::Layout;
use super::wgpu_gui::{LayoutElements, WgpuGui};
use super::{mouse_event::MouseEvent, size::Size};


struct SubView<'a, TMessage> {
    layout: &'a mut Layout, 
    elements: &'a mut dyn FnMut(&mut LayoutElements<TMessage>)
}

impl<'a, TMessage> SubView<'a, TMessage> {
    fn new(layout: &'a mut Layout, elements: &'a mut dyn FnMut(&mut LayoutElements<TMessage>)) -> Self {
        Self { layout, elements }
    }
}

impl<'a, TMessage> GuiElement<TMessage> for SubView<'a, TMessage> {
    fn size(&mut self) -> Size {
        self.layout.size()
    }

    fn mouse_event(&mut self, mouse_event: &MouseEvent, model: &mut dyn FnMut(TMessage)) -> bool {
        self.layout.mouse_event(mouse_event, model, &mut self.elements);
        true
    }

    fn update(&mut self, widget_renderer: &mut dyn WidgetRenderer) {
        self.layout.update(widget_renderer);
    }

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size) {
        self.layout.resize(widget_renderer, abs_x, abs_y, size, &mut self.elements);
    }


    
    // fn draw<'b>(&'b mut self, render_pass: &mut wgpu::RenderPass<'b>) {
    //     // (self.elements)(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<TMessage>| {
    //     //     element.draw(render_pass);
    //     // }));

    //     todo!()
    // }
}

// fn create_sub_view<'a, TMessage>() -> WgpuGui<'a, TMessage> {
//     let sub_view = WgpuGui::new(&mut |layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<TMessage>)| {

//         let mut sub_view = SubView::new(layout, elements);
//         sub_view.update_device();

//         // elements(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<T::TSubMessage>| {
//         //     element.update_device();
//         // }));
//     });

//     // sub_view
// }

pub trait GuiElement<TMessage> {    
    fn size(&mut self) -> Size;

    fn mouse_event(&mut self, mouse_event: &MouseEvent, model: &mut dyn FnMut(TMessage)) -> bool;

    fn update(&mut self, widget_renderer: &mut dyn WidgetRenderer);

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size);

    // fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}


pub trait GuiElementSubView
{
    type TMessage;
    type TSubMessage;

    fn get_elements(&mut self, ui: &mut WgpuGui<Self::TSubMessage>);

    fn get_event_conversion_function(&self) -> fn(Self::TSubMessage) -> Self::TMessage;
}

impl<T> GuiElement<T::TMessage> for T where T: GuiElementSubView {
    fn mouse_event(&mut self, mouse_event: &MouseEvent, model: &mut dyn FnMut(T::TMessage)) -> bool
    {
        let conversion_function = self.get_event_conversion_function();
        let mut model_converted = |sub_message: T::TSubMessage| {
            model(conversion_function(sub_message));
        };

        self.get_elements(&mut WgpuGui::new(&mut |layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            layout.mouse_event(mouse_event, &mut model_converted, elements);
        }));

        true
    }

    // fn update_device(&mut self, wgpu_renderer: &mut dyn WgpuRendererInterface)
    // {
    //     self.get_elements(&mut WgpuGui::new(&mut |layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {

    //         let mut sub_view = SubView::new(layout, elements);
    //         sub_view.update_device(wgpu_renderer);

    //         // elements(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<T::TSubMessage>| {
    //         //     element.update_device();
    //         // }));
    //     }));
    // }

    fn resize(&mut self, widget_renderer: &mut dyn WidgetRenderer, abs_x: u32, abs_y: u32, size: Size)
    {
        self.get_elements(&mut WgpuGui::new(&mut |layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            layout.resize(widget_renderer, abs_x, abs_y, size, elements);
        }));
    }

    // fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>)
    // {
    //     // self.get_elements(&mut WgpuGui::new(&mut |_layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
    //     //     elements(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<T::TSubMessage>| {
    //     //         element.draw(render_pass);
    //     //     }));
    //     // }));

    //     todo!()
    // }

    fn update(&mut self, widget_renderer: &mut dyn WidgetRenderer) {
        self.get_elements(&mut WgpuGui::new(&mut |layout: &mut Layout, elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            layout.update(widget_renderer);
        }));
    }
    
    fn size(&mut self) -> Size {
        let mut res = Size::new();
        self.get_elements(&mut WgpuGui::new(&mut |layout: &mut Layout, _elements: &mut dyn FnMut(&mut LayoutElements<T::TSubMessage>)| {
            res = layout.size();
        }));

        res
    }
    

}
