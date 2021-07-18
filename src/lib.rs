///
/// # Formats a json string
///
/// The indentation can be set to any custom value  
/// The default value is two spaces
/// The default indentation is faster than a custom one
///
pub fn format_json(json: &str, indentation: Option<&str>) -> String {
    // at least as big as the input to avoid resizing
    // this might be too big if the input string is formatted in a weird way, but that's not expected
    let mut out = String::with_capacity(json.len());

    let mut escaped = false;
    let mut in_string = false;
    let mut indent_level = 0usize;
    let mut newline_requested = false; // invalidated if next character is ] or }

    for char in json.chars() {
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
            out.push(char);
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
                        out.push('\n');
                        indent(&mut out, indent_level, indentation);
                    }
                }
                ':' => {
                    auto_push = false;
                    out.push(char);
                    out.push(' ');
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
                out.push('\n');
                indent(&mut out, old_level, indentation);
            }

            if auto_push {
                out.push(char);
            }

            newline_requested = request_newline;
        }
    }

    out
}

fn indent(buf: &mut String, level: usize, indent_str: Option<&str>) {
    for _ in 0..level {
        match indent_str {
            None => {
                buf.push(' ');
                buf.push(' ');
            }
            Some(indent) => {
                buf.push_str(indent);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn echoes_primitive() {
        let json = "1.35";
        assert_eq!(json, format_json(json, None));
    }

    #[test]
    fn ignore_whitespace_in_string() {
        let json = "\" hallo \"";
        assert_eq!(json, format_json(json, None));
    }

    #[test]
    fn remove_leading_whitespace() {
        let json = "   0";
        let expected = "0";
        assert_eq!(expected, format_json(json, None));
    }

    #[test]
    fn handle_escaped_strings() {
        let json = "  \" hallo \\\" \" ";
        let expected = "\" hallo \\\" \"";
        assert_eq!(expected, format_json(json, None));
    }

    #[test]
    fn simple_object() {
        let json = "{\"a\":0}";
        let expected = "{
  \"a\": 0
}";
        assert_eq!(expected, format_json(json, None));
    }

    #[test]
    fn simple_array() {
        let json = "[1,2,null]";
        let expected = "[
  1,
  2,
  null
]";
        assert_eq!(expected, format_json(json, None));
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

        assert_eq!(expected, format_json(json, None));
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

        assert_eq!(expected, format_json(expected, None));
    }
}
