use crate::widget::widget_renderer::WidgetRenderer;

use super::{gui_functions::GuiElement, mouse_event::MouseEvent, size::Size, wgpu_gui::LayoutElements};




pub enum Alignment {
    Center, 
    Left,
    Right
}

pub enum LayoutKind {
    Horizontal,
    Vertical,
}

pub struct Layout {
    alignment: Alignment,
    layout: LayoutKind,

    size: Size,
    abs_x: u32,
    abs_y: u32,
}

impl Layout 
{
    pub fn new() -> Self {
        Self {
            alignment: Alignment::Center,
            layout: LayoutKind::Vertical,
            size: Size::new(),
            abs_x: 0,
            abs_y: 0,
        }
    }

    pub fn horizontal_layout(mut self) -> Self {
        self.layout = LayoutKind::Vertical;
        self
    }

    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    fn calculate_element_size<TMessage>(&mut self, elements: &mut dyn FnMut(&mut LayoutElements<TMessage>)) {

        let mut width =  0;
        let mut height = 0;

        elements(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<TMessage>| {
            let elem_size = element.size();

            width = width + elem_size.width;
            height = height.max(elem_size.height);
        }));

        self.size.width = width;
        self.size.height = height;
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x && x < self.abs_x + self.size.width &&
        y >= self.abs_y && y < self.abs_y + self.size.height 
    }

    pub fn mouse_event<TMessage>(&self, 
        mouse_event: &MouseEvent, 
        model: &mut dyn FnMut(TMessage),
        elements: &mut dyn FnMut(&mut LayoutElements<TMessage>)
    ) -> bool 
    {
        // let x = mouse_event.x;
        // let y = mouse_event.y;

        // if !self.is_inside(x, y) {
        //     return false;
        // }

        let mut res = false;
        elements(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<TMessage>| {
            res = res || element.mouse_event(mouse_event, model);
        }));
        res
    }

    pub fn resize<TMessage>(&mut self, 
        widget_renderer: &mut dyn WidgetRenderer,
        abs_x: u32, abs_y: u32, _size: Size,
        elements: &mut dyn FnMut(&mut LayoutElements<TMessage>)
    )
    {
        self.calculate_element_size(elements);


        self.abs_x = abs_x;
        self.abs_y = abs_y;
        let mut delta_width = 0;

        elements(&mut LayoutElements::new(&mut |element: &mut dyn GuiElement<TMessage>| {

            let elem_size = element.size();

            let element_abs_x = abs_x + delta_width;
            let element_abs_y = abs_y + self.size.height/2 - elem_size.height/2;
            element.resize(widget_renderer, element_abs_x, element_abs_y, elem_size);

            delta_width += elem_size.width;
        }));
    }
}

