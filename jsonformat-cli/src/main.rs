use std::{
    fs::File,
    io,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use jsonformat::{format_reader_writer, Indentation};

#[derive(Parser)]
#[clap(author, about, version)]
struct Options {
    #[clap(short, long)]
    indentation: Option<String>,
    #[clap(short, long)]
    output: Option<PathBuf>,
    input: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    // Note: on-stack dynamic dispatch
    // ugly af but works
    let (mut file, stdin, mut stdin_lock);
    let reader: &mut dyn Read = match &options.input {
        Some(path) => {
            file = File::open(path)
                .context(format!("Name: {}", path.display()))
                .context("Open input file")?;
            &mut file
        }
        None => {
            stdin = io::stdin();
            stdin_lock = stdin.lock();
            &mut stdin_lock
        }
    };

    let replaced_indent = options.indentation.map(|value| {
        value
            .to_lowercase()
            .chars()
            .filter(|c| ['s', 't'].contains(c))
            .collect::<String>()
            .replace('s', " ")
            .replace('t', "\t")
    });

    let indent = match replaced_indent {
        Some(ref str) => Indentation::Custom(str),
        None => Indentation::Default,
    };

    // Note: on-stack dynamic dispatch
    let (mut file, stdout, mut stdout_lock);
    let writer: &mut dyn Write = match &options.output {
        Some(path) => {
            file = File::create(path)
                .context(path.display().to_string())
                .context("Open output file")?;
            &mut file
        }
        None => {
            stdout = io::stdout();
            stdout_lock = stdout.lock();
            &mut stdout_lock
        }
    };

    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);
    format_reader_writer(&mut reader, &mut writer, indent).context("failed to read or write")?;

    Ok(())
}
