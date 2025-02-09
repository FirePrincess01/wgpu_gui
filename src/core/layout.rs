use crate::widget::widget_renderer::WidgetRenderer;

use super::{gui_functions::{GuiElement, GuiEventResult}, mouse_event::MouseEvent, size::Size, wgpu_gui::LayoutElements};




pub enum Alignment {
    Center, 
    
    Left,
    Right,
    Top,
    Bottom,
    
    LeftTop,
    LeftBottom,

    RightTop,
    RightBottom,
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
        self.layout = LayoutKind::Horizontal;
        self
    }

    pub fn vertical_layout(mut self) -> Self {
        self.layout = LayoutKind::Vertical;
        self
    }

    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    fn calculate_element_size<TMessage>(&mut self, elements: &mut [&mut dyn GuiElement<TMessage>]) {

        let mut width =  0;
        let mut height = 0;

        for element in elements {

            let elem_size = element.size();

            match self.layout {
                LayoutKind::Horizontal => {
                    width = width + elem_size.width;
                    height = height.max(elem_size.height);
                },
                LayoutKind::Vertical => {
                    width = width.max(elem_size.width);
                    height = height + elem_size.height;
                },
            }
        };

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
        event_result: &mut GuiEventResult<TMessage>,
        elements: &mut [&mut dyn GuiElement<TMessage>]
    ) -> bool 
    {
        // let x = mouse_event.x;
        // let y = mouse_event.y;

        // if !self.is_inside(x, y) {
        //     return false;
        // }

        let mut res = false;
        for element in elements {
            res = res || element.mouse_event(mouse_event, event_result);
        }

        res
    }

    pub fn update<TMessage>(&mut self, 
        widget_renderer: &mut dyn WidgetRenderer,
        elements:&mut [&mut dyn GuiElement<TMessage>],
    )
    {
        for element in elements {
            element.update(widget_renderer);
        };

    }

    pub fn resize<TMessage>(&mut self, 
        widget_renderer: &mut dyn WidgetRenderer,
        abs_x: u32, abs_y: u32, size: Size,
        elements: &mut [&mut dyn GuiElement<TMessage>]
    )
    {
        self.calculate_element_size(elements);


        // alignment
        match self.alignment {
            Alignment::Center => {
                self.abs_x = size.width/2 - self.size.width/2;
                self.abs_y = size.height/2 - self.size.height/2;
            },
            Alignment::Left => {
                self.abs_x = 0;
                self.abs_y = size.height/2 - self.size.height/2;
            },
            Alignment::Right => {
                self.abs_x = size.width - self.size.width;
                self.abs_y = size.height/2 - self.size.height/2;
            },
            Alignment::Top => {
                self.abs_x = size.width/2 - self.size.width/2;
                self.abs_y = size.height - self.size.height;
            },
            Alignment::Bottom => {
                self.abs_x = size.width/2 - self.size.width/2;
                self.abs_y = 0;
            },
            Alignment::LeftTop => {
                self.abs_x = 0;
                self.abs_y = size.height - self.size.height;
            },
            Alignment::LeftBottom => {
                self.abs_x = 0;
                self.abs_y = 0;
            },
            Alignment::RightTop => {
                self.abs_x = size.width - self.size.width;
                self.abs_y = size.height - self.size.height;
            },
            Alignment::RightBottom => {
                self.abs_x = size.width - self.size.width;
                self.abs_y = 0;
            },
        }

        // layout
        // self.abs_x = abs_x;
        // self.abs_y = abs_y;
        let mut delta = 0;

        for element in elements {

            let elem_size = element.size();

            match self.layout {
                LayoutKind::Horizontal => {
                    let element_abs_x = self.abs_x + delta;
                    let element_abs_y = self.abs_y + self.size.height/2 - elem_size.height/2;
                    
                    element.resize(widget_renderer, element_abs_x, element_abs_y, elem_size);
                    delta += elem_size.width;
                },
                LayoutKind::Vertical => {
                    let element_abs_x = self.abs_x + self.size.width/2 - elem_size.width/2;
                    let element_abs_y = self.abs_y + delta;

                    element.resize(widget_renderer, element_abs_x, element_abs_y, elem_size);
                    delta += elem_size.height;
                },
            }            
        };
    }
}

