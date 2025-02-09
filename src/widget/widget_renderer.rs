

pub struct LabelResult {
    pub index: usize,
    pub height: u32,
    pub width: u32,
}

pub trait WidgetRenderer  {
    // fn set_visible(&mut self, index: usize, is_visible: bool);
    // fn set_position(&mut self, index: usize, x: u32, y: u32);

    fn create_label(&mut self, text: &str, scale: u32) -> LabelResult;
    fn set_label_text(&mut self, index: usize, text: &str);
    fn set_label_position(&mut self, index: usize, x: u32, y: u32);
    fn set_label_visible(&mut self, index: usize, is_visible: bool);
}

