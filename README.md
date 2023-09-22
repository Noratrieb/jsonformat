# Extremely fast JSON formatter

`jsonformat` is an extremely fast JSON formatter.

It formats over 20MB of nested JSON in 60ms.

## Library crate

For the library crate, look at [docs.rs](https://docs.rs/jsonformat)

## Binary Install
You need Rust installed on your system  
`cargo install jsonformat-cli`

## Binary Usage
```
jsonformat-cli 0.2.0
Formats JSON extremely fast

USAGE:
    jsonformat [OPTIONS] [INPUT]

ARGS:
    <INPUT>    The input file

OPTIONS:
    -h, --help                         Print help information
    -i, --indentation <INDENTATION>    The indentation, s will replaced by a space and t by a tab.
                                       ss is the default
    -o, --output <OUTPUT>              The output file
    -V, --version                      Print version information
```

Reads from stdin if no file is supplied.
Outputs to stdout if no output file is specified.

## Error handling
`jsonformat` does not report malformed json - it can't even fully know whether the json is actually malformed. 
Malformed json is just formatted kind of incorrectly, with no data lost and no crashes. If you find one, open an issue,


## How?
`jsonformat` does not actually parse the json, it just loops through each character and keeps track of some flags. 
It then copies these characters to the output buffer, adding and removing whitespace.
