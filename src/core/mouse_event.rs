

#[derive(Clone)]
pub struct MouseEvent {
    pub x: u32,
    pub y: u32,
    pub is_pressed: bool,
}

impl MouseEvent {
    pub fn new() -> Self {
        Self { 
            x: 0, 
            y: 0, 
            is_pressed: false, 
        }
    }
}

