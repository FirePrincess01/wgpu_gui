// use crate::{core::{gui_functions::{GuiElement, GuiElementSubView}, layout::{self, Layout}, size::Size, wgpu_gui::LayoutElements}, widget::button::Button};

// struct ButtonMock {

// }

// impl ButtonMock {
//     fn new() -> Self {
//         Self {  }
//     }
// }

// impl<TMessage> GuiElement<TMessage> for ButtonMock{
//     fn mouse_event(&mut self, _mouse_event: &crate::core::mouse_event::MouseEvent, _model: &mut dyn FnMut(TMessage)) -> bool {
//         todo!()
//     }

//     fn update_device(&mut self) {
//         todo!()
//     }

//     fn resize(&mut self, _abs_x: u32, _abs_y: u32, _size: Size) {
//         todo!()
//     }

//     fn draw(&mut self) {
//         todo!()
//     }
    
//     fn size(&mut self) -> Size {
//         todo!()
//     }
// }



// enum MessageSubView {
//     Button1Pressed,
//     Button2Pressed,
// }

// enum MessageView {
//     SubView1(MessageSubView),
//     SubView2(MessageSubView),
//     Button3Pressed,
// }

// struct SubView {
//     button1: ButtonMock,
//     button2: ButtonMock,
//     layout: Layout,
//     on_changed: fn(MessageSubView) -> MessageView,
// }

// impl SubView {
//     fn new(on_changed: fn(MessageSubView) -> MessageView) -> Self {
//         let button1 = ButtonMock::new();
//         let button2 = ButtonMock::new();
//         let layout = Layout::new();

//         Self { button1, button2, layout, on_changed }
//     }
// }

// impl GuiElementSubView for SubView {
//     type TMessage = MessageView;

//     type TSubMessage = MessageSubView;

//     fn get_elements(&mut self, ui: &mut crate::core::wgpu_gui::WgpuGui<Self::TSubMessage>) {
//         ui.layout(&mut self.layout, &mut |elements: &mut LayoutElements<Self::TSubMessage>| {
//             elements.add(&mut self.button1);
//             elements.add(&mut self.button2);
//         });
//     }

//     fn get_event_conversion_function(&self) -> fn(Self::TSubMessage) -> Self::TMessage {
//         self.on_changed
//     }
// } 

// struct View {
//     sub_view1: SubView,
//     sub_view2: SubView,
//     button3: ButtonMock,
//     layout: Layout,
// }

// impl View {
//     fn new() -> Self {
//         let sub_view1 = SubView::new(MessageView::SubView1); 
//         let sub_view2 = SubView::new(MessageView::SubView2); 
//         let button3 = ButtonMock::new(); 
//         let layout = Layout::new();        
        
//         Self { sub_view1, sub_view2, button3, layout }
//     }
// }

// impl GuiElementSubView for View {
//     type TMessage = MessageView;

//     type TSubMessage = MessageView;

//     fn get_elements(&mut self, ui: &mut crate::core::wgpu_gui::WgpuGui<Self::TSubMessage>) {
//         ui.layout(&mut self.layout, &mut |elements: &mut LayoutElements<Self::TSubMessage>| {
//             elements.add(&mut self.sub_view1);
//             elements.add(&mut self.sub_view2);
//             elements.add(&mut self.button3);
//         });
//     }

//     fn get_event_conversion_function(&self) -> fn(Self::TSubMessage) -> Self::TMessage {
//         |message: Self::TMessage| -> Self::TMessage { message }
//     }
// } 


// #[ignore] // not yet implemented
// #[test]
// fn test_update_device() {
//     let mut view = View::new();
//     view.update_device();


// }

