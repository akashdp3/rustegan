use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Echo,
    EchoOk,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageBody {
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,

    #[serde(flatten)]
    others: HashMap<String, Value>,
}

impl MessageBody {
    pub fn with_type(&self, kind: &str) -> anyhow::Result<Self> {
        // let kind: Kind = serde_json::from_str(kind).context("Failed to parse str to Kind")?;
        // let kind: Kind = serde_json::from_value(serde_json::Value::String(kind.to_owned()))
        //     .context("Failed to parse kind str to Kind enum")?;

        Ok(Self {
            kind: kind.to_owned(),
            id: self.id,
            in_reply_to: self.id,
            others: self.others.clone(),
        })
    }
}
