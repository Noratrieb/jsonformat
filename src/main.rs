use clap::clap_app;
use jsonformat::{Indentation, format_json_buffered};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(jsonformat =>
        (version: "1.1")
        (author: "nilstrieb <nilstrieb@gmail.com>")
        (about: "Formats json from stdin or from a file")
        (@arg stdout: -s --stdout "Output the result to stdout instead of the default output file. Windows only.")
        (@arg indentation: -i --indent +takes_value "Set the indentation used (\\s for space, \\t for tab)")
        (@arg output: -o --output +takes_value "The output file for the formatted json")
        (@arg input: "The input file to format")
    )
    .get_matches();

    let reader: Box<dyn Read> = match matches.value_of("input") {
        Some(path) => {
            let f = File::open(path)?;
            Box::new(BufReader::new(f))
        },
        None => {
            Box::new(std::io::stdin())
        }
    };

    let replaced_indent = matches.value_of("indentation").map(|value| {
        value
            .to_lowercase()
            .chars()
            .filter(|c| ['s', 't'].contains(c))
            .collect::<String>()
            .replace("s", " ")
            .replace("t", "\t")
    });

    let indent = match replaced_indent {
        Some(ref str) => Indentation::Custom(str),
        None => Indentation::Default,
    };

    let mut output = matches.value_of("output");
    let mut windows_output_default_file: Option<String> = None;

    #[cfg(windows)]
    if !matches.is_present("stdout") {
        if let Some(file) = matches.value_of("input") {
            // on windows, set the default output file if no stdout flag is provided
            // this makes it work with drag and drop in windows explorer
            windows_output_default_file = Some(file.replace(".json", "_f.json"))
        }
    }

    output = windows_output_default_file.as_deref().or(output);

    let writer : Box<dyn Write> = match output {
        Some(file) => {
            Box::new(File::create(file)?)
        }
        None => Box::new(std::io::stdout()),
    };

    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);
    format_json_buffered(&mut reader, &mut writer, indent)?;

    Ok(())
}
