


use super::{gui_functions::GuiElement, layout::Layout};

pub struct WgpuGui<'a, TMessage> {
    layout_function: &'a mut dyn FnMut(&mut Layout, &mut dyn FnMut(&mut LayoutElements<TMessage>)),
}

impl<'a, TMessage>  WgpuGui<'a, TMessage>  {

    pub fn new(layout_function: &'a mut dyn FnMut(&mut Layout, &mut dyn FnMut(&mut LayoutElements<TMessage>))) -> Self {
        Self { layout_function }
    }
    
    pub fn layout(&mut self, layout: &mut Layout, elements: &mut impl FnMut(&mut LayoutElements<TMessage>) 
    ) {
        (self.layout_function)(layout, elements);
    }
}


pub struct LayoutElements<'a, TMessage> {
    visitor: &'a mut dyn FnMut(&mut dyn GuiElement<TMessage>),
}

impl<'a, TMessage>  LayoutElements<'a, TMessage>  {
    pub fn new(visitor: &'a mut dyn FnMut(&mut dyn GuiElement<TMessage>)) -> Self {
        Self { visitor }
    }
    
    pub fn add(&mut self, element: &mut dyn GuiElement<TMessage>) {
        (self.visitor)(element);
    }
}




