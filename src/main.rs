use clap::clap_app;
use jsonformat::{format_json, Indentation};
use std::fs;
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
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

    let str = match matches.value_of("input") {
        Some(path) => fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            let stdin = std::io::stdin();
            stdin.lock().read_to_string(&mut buf)?;
            buf
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

    let formatted = format_json(&str, indent);

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

    match output {
        Some(file) => {
            fs::write(file, formatted)?;
        }
        None => println!("{}", formatted),
    }

    Ok(())
}
