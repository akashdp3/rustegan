use crate::protocol::{Message, MessageBody};

pub struct Node {
    id: Option<String>,
}

impl Node {
    pub fn new() -> Self {
        Self { id: None }
    }

    pub fn reply(&mut self, input: Message) -> anyhow::Result<Option<Message>> {
        let output_body = match input.body {
            MessageBody::Init {
                msg_id, node_id, ..
            } => {
                self.id = Some(node_id);
                Some(MessageBody::InitOk {
                    in_reply_to: msg_id,
                })
            }
            MessageBody::InitOk { .. } => {
                anyhow::bail!("Unexpected init_ok received")
            }
            MessageBody::Echo { msg_id, echo } => Some(MessageBody::EchoOk {
                msg_id,
                in_reply_to: msg_id,
                echo,
            }),
            MessageBody::EchoOk { .. } => None,
        };

        match output_body {
            Some(body) => Ok(Some(Message {
                src: input.dest,
                dest: input.src,
                body,
            })),
            None => Ok(None),
        }
    }
}
