//! Sailfish runtime

#[macro_use]
mod utils;

mod buffer;
pub mod escape;
mod macros;
mod render;
mod size_hint;

pub use buffer::*;
pub use render::*;
pub use size_hint::*;

use std::fmt;

#[doc(hidden)]
pub use crate::{render, render_escaped, render_noop, render_text};

#[derive(Clone, Debug)]
enum RenderErrorKind {
    Msg(String),
    Fmt(fmt::Error),
}

/// The error type which is returned from template function
#[derive(Clone, Debug)]
pub struct RenderError {
    // currently RenderError simply wraps the fmt::Error
    kind: RenderErrorKind,
}

impl RenderError {
    /// Construct a new error with custom message
    pub fn new(msg: &str) -> Self {
        Self {
            kind: RenderErrorKind::Msg(msg.to_owned()),
        }
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            RenderErrorKind::Msg(ref s) => f.write_str(&**s),
            RenderErrorKind::Fmt(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            RenderErrorKind::Msg(_) => None,
            RenderErrorKind::Fmt(ref e) => Some(e),
        }
    }
}

impl From<fmt::Error> for RenderError {
    #[inline]
    fn from(other: fmt::Error) -> Self {
        Self {
            kind: RenderErrorKind::Fmt(other),
        }
    }
}

pub type RenderResult = Result<String, RenderError>;

#[doc(hidden)]
pub struct Context {
    #[doc(hidden)]
    pub buf: Buffer,
}

impl Context {
    #[inline]
    pub fn into_result(self) -> RenderResult {
        Ok(self.buf.into_string())
    }
}

// #[inline(never)]
// pub fn _instantiate(table: Vec<Vec<usize>>) -> String {
//     let mut buffer = Buffer::with_capacity(130000);
//     buffer.push_str("<table>");
//     for r1 in table {
//         buffer.push_str("<tr><td>");
//         for r2 in r1 {
//             let _ = (&r2).render(&mut buffer);
//             buffer.push_str("</td><td>");
//         }
//         unsafe { buffer.set_len(buffer.len() - 4) }
//         buffer.push_str("</tr>");
//     }
//     buffer.push_str("</table>");
//     buffer.into_string()
// }
