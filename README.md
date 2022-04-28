# Extremely fast JSON formatter

`jsonformat` is an extremely fast JSON formatter.

It formats over 20MB of nested JSON in 60ms.

For the library, look at [docs.rs](https://docs.rs/jsonformat)

## Install
You need Rust installed on your system  
`cargo install jsonformat-cli`

## Usage
```
USAGE:
    jsonformat [OPTIONS] [input]

ARGS:
    <input>    The input file to format

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --indent <indentation>    Set the indentation used (\s for space, \t for tab)
    -o, --output <output>         The output file for the formatted json
```

Reads from stdin if no file is supplied.
Outputs to stdout if no output file is specified.

On Windows, it writes to a file called `<filename>_f.json`, unless the `--stdout` flag is used or a custom output
file is provided. This it to enable drag-and-drop in Windows explorer.

## Error handling
`jsonformat` does not report malformed json - it can't even fully know whether the json is actually malformed. 
Malformed json is just formatted kind of incorrectly, with no data lost and no crashes. If you find one, open an issue,


## How?
`jsonformat` does not actually parse the json, it just loops through each character and keeps track of some flags. 
It then copies these characters to the output buffer, adding and removing whitespace.
