# wgpu_gui
A lightweight gui for any wgpu based renderer.
Everything is explicitly defined, no hidden control-flow in the background (which results in slightly more code, but is easier to understand).

```Rust

/// Model

enum Message {
    IncrementPressed,
    DecrementPressed,
}

struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> {
        Self {
            value: 0,
        }
    }

    pub fn message(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    pub fn value(&self) {
        value
    }
}

/// GUI Implementation

struct CounterGui {
    type Button = wgpu_gui::Button<Message, wgpu::gui::default_pipeline::Button>;
    type Text = wgpu_gui::Text<Message, wgpu::gui::default_pipeline::Text>;

    button_increment: Button,
    button_decrement: Button,
    text: Text,
    layout: wgpu_gui::Layout,
}

impl CounterGui {
    pub fn new() -> Self 
    {
        let button_increment = Button::new("increment").on_released(Message::IncrementPressed);
        let text = Text::from_space(5).size(50);
        let button_decrement = Button::new("decrement").on_released(Message::DecrementPressed);
        let layout = wgpu_gui::Layout::new();

        Self {
            button_increment,
            text,
            button_decrement,
            layout,
        }
    }

    fn layout(&self) -> Elements {
        let elements = layout.align(Alignment::Center).vertical_layout([&button_pressed, &text &button_released]);
        elements
    }

    pub fn mouse_event(&mut self, counter: &Counter, mouse_event: wgpu_gui::MouseEvent,) {
         // handle messages
        let messages = self.layout().event(mouse_event);
        match messages {
            Message(message):
            counter.update(message);
        }
    }

    pub fn update(counter: &Counter, mouse_event: wgpu_gui::MouseEvent) {
        // update gui elements
        self.text.set_value(counter.value);
        self.layout().update();   // updates all changed textures on the gpu
    }

    // Draw functions

    pub fn resize(&mut self, size: Size) {
        self.layout().resize(size);
    }

    pub fn draw(&self) {
        self.layout().draw();
    }

}



/// User Renderer Implementation

struct YourApplication{
    ...
    renderer: YoureRenderer,
    counter: Counter,
    counter_gui: CounterGui,
    ...
}

impl YourApplication {
    ...
    pub fn mouse_event(&mut self) {
        self.counter_gui.update(&mut self.counter);
    }
    
    pub fn update(&mut self) {
        self.counter_gui.update(&self.counter);
    }

    pub fn resize(&mut self, size: Size) {
        self.counter_gui.resize(&size);
    }

    pub fn draw(&self) {
        self.counter_gui.draw();
    }

    ...

}

```


