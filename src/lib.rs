//!
//! jsonformat is a library for formatting json.
//!
//! It does not do anything more than that, which makes it so fast.

use std::error::Error;
use std::io::{BufReader, BufWriter, Read, Write};
use utf8_chars::BufReadCharsExt;

///
/// Set the indentation used for the formatting.
///
/// Note: It is *not* recommended to set indentation to anything oder than some spaces or some tabs,
/// but nothing is stopping you from doing that.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Indentation<'a> {
    /// Use the default indentation, which is two spaces
    Default,
    /// Use a custom indentation String
    Custom(&'a str),
}

///
/// # Formats a json string
///
/// The indentation can be set to any value using [Indentation](jsonformat::Indentation)
/// The default value is two spaces
/// The default indentation is faster than a custom one
///
pub fn format_json(json: &str, indentation: Indentation) -> String {
    let mut reader = BufReader::new(json.as_bytes());
    let mut writer = BufWriter::new(Vec::new());

    format_json_buffered(&mut reader, &mut writer, indentation).unwrap();
    String::from_utf8(writer.into_inner().unwrap()).unwrap()
}

///
/// # Formats a json string
///
/// The indentation can be set to any value using [Indentation](jsonformat::Indentation)
/// The default value is two spaces
/// The default indentation is faster than a custom one
///
pub fn format_json_buffered<R, W>(
    reader: &mut BufReader<R>,
    writer: &mut BufWriter<W>,
    indentation: Indentation,
) -> Result<(), Box<dyn Error>>
where
    R: Read,
    W: Write,
{
    let mut escaped = false;
    let mut in_string = false;
    let mut indent_level = 0usize;
    let mut newline_requested = false; // invalidated if next character is ] or }

    for char in reader.chars() {
        let char = char?;
        if in_string {
            let mut escape_here = false;
            match char {
                '"' => {
                    if !escaped {
                        in_string = false;
                    }
                }
                '\\' => {
                    if !escaped {
                        escape_here = true;
                    }
                }
                _ => {}
            }
            writer.write_all(char.encode_utf8(&mut [0; 4]).as_bytes())?;
            escaped = escape_here;
        } else {
            let mut auto_push = true;
            let mut request_newline = false;
            let old_level = indent_level;

            match char {
                '"' => in_string = true,
                ' ' | '\n' | '\t' => continue,
                '[' => {
                    indent_level += 1;
                    request_newline = true;
                }
                '{' => {
                    indent_level += 1;
                    request_newline = true;
                }
                '}' | ']' => {
                    indent_level = indent_level.saturating_sub(1);
                    if !newline_requested {
                        // see comment below about newline_requested
                        writeln!(writer)?;
                        indent_buffered(writer, indent_level, indentation)?;
                    }
                }
                ':' => {
                    auto_push = false;
                    writer.write_all(char.encode_utf8(&mut [0; 4]).as_bytes())?;
                    writer.write_all(" ".as_bytes())?;
                }
                ',' => {
                    request_newline = true;
                }
                _ => {}
            }
            if newline_requested && char != ']' && char != '}' {
                // newline only happens after { [ and ,
                // this means we can safely assume that it being followed up by } or ]
                // means an empty object/array
                writeln!(writer)?;
                indent_buffered(writer, old_level, indentation)?;
            }

            if auto_push {
                writer.write_all(char.encode_utf8(&mut [0; 4]).as_bytes())?;
            }

            newline_requested = request_newline;
        }
    }

    Ok(())
}

fn indent_buffered<W>(
    writer: &mut BufWriter<W>,
    level: usize,
    indent_str: Indentation,
) -> Result<(), Box<dyn Error>>
where
    W: std::io::Write,
{
    for _ in 0..level {
        match indent_str {
            Indentation::Default => {
                writer.write_all(" ".as_bytes())?;
                writer.write_all(" ".as_bytes())?;
            }
            Indentation::Custom(indent) => {
                writer.write_all(indent.as_bytes())?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn echoes_primitive() {
        let json = "1.35";
        assert_eq!(json, format_json(json, Indentation::Default));
    }

    #[test]
    fn ignore_whitespace_in_string() {
        let json = "\" hallo \"";
        assert_eq!(json, format_json(json, Indentation::Default));
    }

    #[test]
    fn remove_leading_whitespace() {
        let json = "   0";
        let expected = "0";
        assert_eq!(expected, format_json(json, Indentation::Default));
    }

    #[test]
    fn handle_escaped_strings() {
        let json = "  \" hallo \\\" \" ";
        let expected = "\" hallo \\\" \"";
        assert_eq!(expected, format_json(json, Indentation::Default));
    }

    #[test]
    fn simple_object() {
        let json = "{\"a\":0}";
        let expected = "{
  \"a\": 0
}";
        assert_eq!(expected, format_json(json, Indentation::Default));
    }

    #[test]
    fn simple_array() {
        let json = "[1,2,null]";
        let expected = "[
  1,
  2,
  null
]";
        assert_eq!(expected, format_json(json, Indentation::Default));
    }

    #[test]
    fn array_of_object() {
        let json = "[{\"a\": 0}, {}, {\"a\": null}]";
        let expected = "[
  {
    \"a\": 0
  },
  {},
  {
    \"a\": null
  }
]";

        assert_eq!(expected, format_json(json, Indentation::Default));
    }

    #[test]
    fn already_formatted() {
        let expected = "[
  {
    \"a\": 0
  },
  {},
  {
    \"a\": null
  }
]";

        assert_eq!(expected, format_json(expected, Indentation::Default));
    }
}
