use clap::clap_app;
use jsonformat::format_json;
use std::fs;
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let matches = clap_app!(jsonformat =>
        (version: "1.0")
        (author: "nilstrieb <nilstrieb@gmail.com>")
        (about: "Formats json")
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

    let replaced_indent = matches
        .value_of("indentation")
        .map(|value| value.replace("s", " ").replace("t", "\t"));

    let formatted = format_json(&str, replaced_indent.as_deref());

    match matches.value_of("output") {
        Some(file) => {
            fs::write(file, formatted)?;
        }
        None => println!("{}", formatted),
    }

    Ok(())
}
