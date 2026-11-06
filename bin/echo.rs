use rustegan::node::Node;
use rustegan::protocol::Message;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MessageBody {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: usize,
    },
    Echo {
        msg_id: usize,
        echo: String,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
}

struct EchoNode {
    id: Option<String>,
    next_msg_id: usize,
}

impl Node<MessageBody> for EchoNode {
    fn reply(
        &mut self,
        input: Message<MessageBody>,
    ) -> anyhow::Result<Option<Message<MessageBody>>> {
        let body = match input.body {
            MessageBody::Init {
                msg_id, node_id, ..
            } => {
                self.id = Some(node_id);
                Some(MessageBody::InitOk {
                    in_reply_to: msg_id,
                })
            }

            MessageBody::Echo { msg_id, echo } => {
                let reply_id = self.next_msg_id;
                self.next_msg_id += 1;

                Some(MessageBody::EchoOk {
                    msg_id,
                    in_reply_to: msg_id,
                    echo,
                })
            }

            MessageBody::EchoOk { .. } | MessageBody::InitOk { .. } => None,
        };

        match body {
            Some(msg_body) => Ok(Some(Message {
                src: input.dest,
                dest: input.src,
                body: msg_body,
            })),
            None => Ok(None),
        }
    }
}

fn main() -> anyhow::Result<()> {
    use std::io::{self, Write};

    let mut node = EchoNode {
        id: None,
        next_msg_id: 0,
    };
    let mut stdout = io::stdout().lock();

    for line in io::stdin().lines() {
        let input: Message<MessageBody> = serde_json::from_str(&line?)?;

        if let Some(output) = node.reply(input)? {
            serde_json::to_writer(&mut stdout, &output)?;
            writeln!(stdout)?;
            stdout.flush()?;
        }
    }

    Ok(())
}
