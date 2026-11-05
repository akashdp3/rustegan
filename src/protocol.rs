use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Init,
    InitOk,
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
    pub kind: Kind,

    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}

impl MessageBody {
    pub fn with_type(&self, kind: Kind) -> anyhow::Result<Self> {
        Ok(Self {
            kind,
            id: self.id,
            in_reply_to: self.id,
            others: self.others.clone(),
        })
    }
}
