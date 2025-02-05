


pub trait WidgetRenderer  {
    fn set_visible(&mut self, index: usize, is_visible: bool);
    fn set_position(&mut self, index: usize, x: u32, y: u32);
}

