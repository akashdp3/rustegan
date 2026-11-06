mod node;
mod protocol;

use std::io::{self, Write};

use anyhow::Context;

use crate::node::Node;
use crate::protocol::Message;

fn main() -> anyhow::Result<()> {
    let input_lines = io::stdin().lines();
    let mut stdout = io::stdout().lock();

    for line in input_lines {
        let line = line.context("Failed to read input from STDIN")?;
        let input: Message = serde_json::from_str(&line)
            .context("Failed to parse from stdin input to Message using serde")?;

        let mut node = Node::new();
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
