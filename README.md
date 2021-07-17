# Extremely fast JSON formatter

`jsonformat` is an extremely fast JSON formatter.

It formats over 60MB of nested JSON in under 0.4s.

## Install
Currently, you have to build and install it yourself.  
`cargo build --release`  
The executable can then be found in `target/release/jsonformat`

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

## How?
`jsonformat` does not actually parse the json, it just loops through each character and keeps track of some flags. It then copies these characters to the output buffer, adding and removing whitespace.

The code is currently a bit chaotic, but it works and is fast, so good enough for now. Maybe it could profit from SIMD in the future, but I have never used it and I don't know whether it would work. Maybe some day...
