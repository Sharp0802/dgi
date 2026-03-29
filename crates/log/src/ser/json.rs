use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub indent: &'static str,
    pub linebreak: &'static str,
    pub member_name_replacement: char,
    pub quoted_member_name: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            indent: "  ",
            linebreak: "\n",
            member_name_replacement: '-',
            quoted_member_name: false,
        }
    }
}

pub struct Pretty<'a> {
    indent: usize,
    value: &'a Value,
    options: Options,
}

impl<'a> Pretty<'a> {
    pub fn new(indent: usize, value: &'a Value, options: Options) -> Self {
        Self {
            indent,
            value,
            options,
        }
    }

    fn indented(&self, value: &'a Value) -> Self {
        Self {
            indent: self.indent + 1,
            value,
            options: self.options,
        }
    }

    fn indent(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.indent {
            write!(f, "{}", self.options.indent)?;
        }

        Ok(())
    }

    fn br(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.options.linebreak)
    }

    fn begin_array(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")
    }

    fn between_element(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ",")?;
        self.br(f)?;
        Ok(())
    }

    fn end_array(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "]")
    }

    fn begin_object(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")
    }

    fn write_member_name(&self, name: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.options.quoted_member_name {
            write_string(name, f)?;
        } else {
            for ch in name.chars() {
                write!(
                    f,
                    "{}",
                    if ch.is_whitespace() || ch.is_control() {
                        self.options.member_name_replacement
                    } else {
                        ch
                    }
                )?;
            }
        }

        write!(f, ": ")
    }

    fn between_member(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ",")?;
        self.br(f)?;
        Ok(())
    }

    fn end_object(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "}}")
    }
}

fn write_string(v: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "\"")?;

    for ch in v.chars() {
        let escaped = match ch {
            '\x08' => "\\b",
            '\t' => "\\t",
            '\n' => "\\n",
            '\x0C' => "\\f",
            '\r' => "\\r",
            '"' => "\\\"",
            '\\' => "\\\\",

            ch if ch.is_control() => {
                for chunk in ch.encode_utf16(&mut [0; char::MAX_LEN_UTF16]) {
                    write!(f, "\\u{:04X}", chunk)?;
                }

                continue;
            }

            ch => {
                write!(f, "{}", ch)?;
                continue;
            }
        };

        write!(f, "{}", escaped)?;
    }

    write!(f, "\"")?;
    Ok(())
}

impl<'a> Display for Pretty<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Value::Null => write!(f, "null"),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Number(v) => write!(f, "{}", v),
            Value::String(v) => write_string(&v, f),

            Value::Array(v) => {
                self.begin_array(f)?;

                if v.len() > 0 {
                    self.br(f)?;
                    
                    for (i, value) in v.iter().enumerate() {
                        if i > 0 {
                            self.between_element(f)?;
                        }

                        let ser = self.indented(value);
                        ser.indent(f)?;
                        ser.fmt(f)?;
                    }
                    
                    self.br(f)?;
                    self.indent(f)?;
                }

                self.end_array(f)
            }

            Value::Object(v) => {
                self.begin_object(f)?;

                if v.len() > 0 {
                    self.br(f)?;

                    for (i, (name, value)) in v.iter().enumerate() {
                        if i > 0 {
                            self.between_member(f)?;
                        }

                        let ser = self.indented(value);
                        ser.indent(f)?;
                        ser.write_member_name(name, f)?;
                        ser.fmt(f)?;
                    }

                    self.br(f)?;
                    self.indent(f)?;
                }

                self.end_object(f)
            }
        }
    }
}
