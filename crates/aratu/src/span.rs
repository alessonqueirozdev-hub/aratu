// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! Positions in the source text.

/// A byte range in the source text, `start..end`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    /// The smallest span covering both.
    pub fn join(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn len(self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Line and column (both starting at 1) of a byte offset, for messages
/// printed outside the editor.
pub fn line_col(src: &str, offset: usize) -> (usize, usize) {
    let offset = offset.min(src.len());
    let before = &src[..offset];
    let line = before.matches('\n').count() + 1;
    let col = before
        .rfind('\n')
        .map_or(before.chars().count(), |i| before[i + 1..].chars().count())
        + 1;
    (line, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_and_column() {
        let src = "meter: 4/4\nbass = | dó4 |";
        assert_eq!(line_col(src, 0), (1, 1));
        assert_eq!(line_col(src, 11), (2, 1));
        let o = src.find("dó4").unwrap();
        assert_eq!(line_col(src, o), (2, 10));
        assert_eq!(line_col(src, o + "dó".len()), (2, 12));
    }
}
