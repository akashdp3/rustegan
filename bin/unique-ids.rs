use rustegan::node::Node;
use rustegan::protocol::Message;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    Generate {
        msg_id: usize,
    },
    GenerateOk {
        id: String,
        msg_id: usize,
        in_reply_to: usize,
    },
}

struct UniqueNode {
    id: Option<String>,
    next_msg_id: usize,
}

impl Node<MessageBody> for UniqueNode {
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

            MessageBody::Generate { msg_id } => {
                let reply_id = self.next_msg_id;
                self.next_msg_id += 1;
                // let unique_id = Uuid::new_v4();

                // let unique_id = match self.id {
                //     Some(ref id) => format!("{}-{}", id, self.next_msg_id),
                //     None => Uuid::new_v4().to_string(),
                // };

                let node_id = self.id.clone().expect("Node ID cannot be NONE");
                let unique_id = format!("{}-{}", node_id, reply_id);

                Some(MessageBody::GenerateOk {
                    msg_id,
                    in_reply_to: msg_id,
                    id: unique_id.to_string(),
                })
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

    let mut node = UniqueNode {
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
// {
//   "type":     "init",
//   "msg_id":   1,
//   "node_id":  "n3",
//   "node_ids": ["n1", "n2", "n3"]
// }
// {"src": "c1", "dest": "n1", "body": { "type": "init","msg_id":  1, "node_id":  "n3", "node_ids": ["n1", "n2", "n3"] }}
// {"src": "c1", "dest": "n1", "body": { "type": "generate" }}
