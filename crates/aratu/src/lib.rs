//! Aratu, a live-coding language that reads like music.
//!
//! The crate reads a piece ([`lex`], then the parser), checks it and reports
//! problems in a musician's words ([`diag`]), and unrolls it into timed events.

#![forbid(unsafe_code)]

pub mod ast;
pub mod diag;
pub mod dynamics;
pub mod exercise;
pub mod grid;
pub mod lex;
pub mod model;
pub mod motifs;
pub mod music;
pub mod params;
pub mod parse;
pub mod ratio;
pub mod schedule;
pub mod span;
pub mod suggest;
pub mod theory;
pub mod time;
pub mod unroll;
pub mod voices;

pub use diag::{Diagnostic, Lang, Message, Severity};
pub use model::Piece;
pub use ratio::Ratio;
pub use span::Span;

/// A piece read and checked.
#[derive(Clone, Debug)]
pub struct Compiled {
    pub piece: Piece,
    pub diagnostics: Vec<Diagnostic>,
}

impl Compiled {
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter().filter(|d| d.is_error())
    }
}

/// Reads and checks a piece.
pub fn compile(src: &str) -> Compiled {
    let (ast, mut diagnostics) = parse::parse(src);
    let piece = model::build(&ast, src, &mut diagnostics);
    diagnostics.sort_by_key(|d| (d.span.start, d.span.end));
    Compiled { piece, diagnostics }
}
