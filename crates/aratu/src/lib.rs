//! Aratu, a live-coding language that reads like music.
//!
//! The crate reads a piece ([`lex`], then the parser), checks it and reports
//! problems in a musician's words ([`diag`]), and unrolls it into timed events.

#![forbid(unsafe_code)]

pub mod ast;
pub mod diag;
pub mod lex;
pub mod music;
pub mod parse;
pub mod ratio;
pub mod span;
pub mod theory;

pub use diag::{Diagnostic, Lang, Message, Severity};
pub use ratio::Ratio;
pub use span::Span;
