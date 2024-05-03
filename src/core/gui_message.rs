


pub trait GuiMessage<TMessage> {
    fn message(&mut self, message: TMessage);
}
