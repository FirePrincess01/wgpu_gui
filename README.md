# wgpu_gui
Trying to implement an extendable graphical user interface while satisfying the borrow checker

It works by implementing a widget renderer/factory which holds all the gpu specific information and some
composition of objects representing the physical shape of the gui. The extensibility is achieved by 
allowing to easily create sub-objects which can be re-instantiated. The communication to the data model is
done by events using generics and enums.

```Rust

// Example 

#[derive(Copy, Clone)]
pub enum CounterMessage {
    IncrementPressed,
    DecrementPressed,
}

#[derive(Copy, Clone)]
pub enum Message {
    SubView1(CounterMessage),
    SubView2(CounterMessage),
}


pub struct CounterGui {
    text: Label,
    sub_view1: CounterGuiSubView,
    sub_view2: CounterGuiSubView,
    layout: Layout,
}

impl CounterGui {
    pub fn new(
        renderer: &mut dyn WidgetRenderer,
    ) -> Self 
    {
        let text = Label::new(renderer, "Hello World!", 32);
        let sub_view1 = CounterGuiSubView::new(renderer, Message::SubView1);
        let sub_view2 = CounterGuiSubView::new(renderer, Message::SubView2);
        let layout = Layout::new().align(Alignment::Center).horizontal_layout();

        Self {
            text,
            sub_view1,
            sub_view2,
            layout,
            on_changed,
        }
    }

    pub fn update(&mut self, counter1: &counter::Counter, counter2: &counter::Counter) {
        self.text.set(String::from("lalalallalal"));
        self.sub_view1.update(counter1.value());
        self.sub_view2.update(counter2.value());
    }
}


impl GuiElementSubView for CounterGui{
    type TMessage = Message;
    type TSubMessage = Message;
    
    fn visit_elements(&mut self, visitor: &mut dyn wgpu_gui::core::gui_functions::GuiElementVisitor<Self::TSubMessage>) {
        visitor.visit(&mut self.layout, &mut [
            &mut self.text,
            &mut self.sub_view1,
            &mut self.sub_view2,
        ]);
    }
        
    fn on_event(&mut self, event: Self::TSubMessage) -> Self::TMessage {
        event
    }  
}


```


