use crate::protocol::Message;

pub trait Node<M> {
    fn reply(&mut self, input: Message<M>) -> anyhow::Result<Option<Message<M>>>;
}
