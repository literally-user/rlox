mod tokenizer;

use std::{
    fs::File,
    io::{BufRead, Read, Write},
};

use anyhow::{Context, anyhow};

use crate::tokenizer::Tokenizer;

fn execute(content: &[u8]) -> anyhow::Result<()> {
    let tokens = Tokenizer::new(content);

    for token in tokens {
        let token = token?;
        println!("{token:?}");
    }

    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    loop {
        print!(";; ");
        std::io::stdout()
            .flush()
            .context("Failed to flush stdout")?;

        let mut content = Vec::new();
        stdin
            .lock()
            .read_until(b'\n', &mut content)
            .context("Failed to read command from stdin")?;

        execute(&content).context("Failed to execute REPL command")?;
    }
}

fn interpret(source: &str) -> anyhow::Result<()> {
    let mut source = File::open(source).context("Failed to open source file")?;

    let mut content = Vec::new();
    source
        .read_to_end(&mut content)
        .context("Failed to read source contents")?;

    execute(&content).context("Failed to execute source file")
}

// Very simple implementation. Maybe i'll improve it in the near future
fn handle_exception_trace(trace: anyhow::Error) -> ! {
    let mut chain = trace.chain();

    println!("{}", chain.next().unwrap());

    let mut spaces = 0;
    for error in chain {
        spaces += 2;
        println!("{}└─ {}", " ".repeat(spaces), error);
    }

    std::process::exit(1);
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    match arguments.as_slice() {
        [] => repl(),
        [source] => interpret(source),
        _ => Err(anyhow!("Invalid command provided; usage: rlox [source]")),
    }
    .map_err(handle_exception_trace);
}
