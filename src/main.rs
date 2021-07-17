use jsonformat::format_json;
use std::fs;
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let filename = std::env::args().skip(1).next();

    let str = match filename {
        Some(path) => fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            let stdin = std::io::stdin();
            stdin.lock().read_to_string(&mut buf)?;
            buf
        }
    };

    println!("{}", format_json(&str, "  "));

    Ok(())
}
