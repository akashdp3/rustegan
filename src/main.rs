mod protocol;

use std::io::{self, Write};

use anyhow::{Context, Result};

use crate::protocol::Message;

fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin().lines();
    let mut stdout = io::stdout().lock();

    let input = stdin
        .next()
        .expect("No input given")
        .context("Failed to get input from stdin")?;
    let input: Message = serde_json::from_str(&input)
        .context("Failed to parse from stdin input to Message using serde")?;

    let output = echo(input).context("Failed to echo output")?;
    let output =
        serde_json::to_string(&output).context("Failed to parse from Message to String")?;

    stdout
        .write_all(&output.as_bytes())
        .context("Failed to write output to stdout")?;

    Ok(())
}

fn echo(input: Message) -> Result<Message> {
    let output = Message {
        src: input.dest,
        dest: input.src,
        body: input.body.with_type("echo_ok")?,
    };

    Ok(output)
}
