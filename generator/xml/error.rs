//! Error handling

use extra::term;
use std::io::with_str_writer;
use std::str::raw::from_c_str;

use super::ffi;

/// The severity of the error
#[deriving(Clone, Eq)]
pub enum ErrorLevel {
    /// A simple warning
    Warning,
    /// A recoverable error
    Error,
    /// A fatal error
    Fatal,
}

impl ErrorLevel {
    fn from_constant(value: ffi::xmlErrorLevel) -> Option<ErrorLevel> {
        match value {
            ffi::XML_ERR_WARNING => Some(Warning),
            ffi::XML_ERR_ERROR   => Some(Error),
            ffi::XML_ERR_FATAL   => Some(Fatal),
            _                    => None,
        }
    }

    pub fn term_color(&self) -> u16 {
        match *self {
            Warning => term::color::YELLOW,
            Error   => term::color::RED,
            Fatal   => term::color::RED,
        }
    }

    pub fn to_color_str(&self) -> ~str {
        do with_str_writer |writer| {
            let term = term::Terminal::new(writer);
            term.map(|t| t.fg(self.term_color()));
            writer.write_str(self.to_str());
            term.map(|t| t.fg(term::color::BLACK));
        }
    }
}

impl ToStr for ErrorLevel {
    pub fn to_str(&self) -> ~str {
        match *self {
            Warning => ~"Warning",
            Error   => ~"Error",
            Fatal   => ~"Fatal",
        }
    }
}

/// An xml parse error
#[deriving(Clone)]
pub struct ErrorData {
    level: ErrorLevel,
    line: uint,
    column: uint,
    message: ~str,
}

impl ErrorData {
    pub unsafe fn from_ptr(error: *ffi::xmlError) -> Option<ErrorData> {
        do ErrorLevel::from_constant((*error).level).map |&level| {
            ErrorData {
                level:      level,
                message:    from_c_str((*error).message),
                line:       (*error).line as uint,
                column:     (*error).int2 as uint,
            }
        }
    }
}

impl ToStr for ErrorData {
    pub fn to_str(&self) -> ~str {
        fmt!("%?:%? %s: %s",
             self.line,
             self.column,
             self.level.to_color_str(),
             self.message)
    }
}
