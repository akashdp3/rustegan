use rustegan::node::Node;
use rustegan::protocol::Message;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MessageBody {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: usize,
    },
    Generate {},
    GenerateOk {
        id: String,
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

            MessageBody::Generate { .. } => {
                let reply_id = self.next_msg_id;
                self.next_msg_id += 1;

                let unique_id = match self.id {
                    Some(ref id) => {
                        format!("{}-{}", id, self.next_msg_id)
                    }
                    None => {
                        return Ok(None);
                    }
                };

                Some(MessageBody::GenerateOk { id: unique_id })
            }

            MessageBody::GenerateOk { .. } | MessageBody::InitOk { .. } => None,
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

        let output = node.reply(input).context("Failed to reply to input")?;

        match output {
            Some(value) => {
                let value = serde_json::to_string(&value).context("Failed to serialize output")?;
                writeln!(stdout, "{value}")
                    .context("Failed to write output to STDOUT with NEW LINE")?;
            }
            None => {
                continue;
            }
        }

        stdout
            .flush()
            .context("Failed to flush all content to destination from STDOUT")?;
    }

    Ok(())
}
