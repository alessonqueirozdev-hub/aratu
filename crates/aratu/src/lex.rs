//! The first pass over the text: lines, comments and blocks, and a cursor
//! for reading the words of a statement.
//!
//! Aratu is written one statement per line. A line that ends with `{` after a
//! name (`drums {`, `pattern groove {`) opens a block that a line holding only
//! `}` closes; a block can also sit on one line, `section a { bass = | c2 | }`,
//! with `;` between its statements. `//` starts a comment.

use crate::diag::{Diagnostic, Message};
use crate::span::Span;

/// One statement's text, without comment or surrounding spaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line<'a> {
    pub text: &'a str,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node<'a> {
    Line(Line<'a>),
    Block(Block<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block<'a> {
    /// The words before `{`, such as `drums` or `pattern groove`.
    pub header: Line<'a>,
    pub body: Vec<Node<'a>>,
    pub span: Span,
}

fn is_name_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Whether `text` is one or two names, the second optional: `drums`,
/// `pattern groove`.
fn is_block_header(text: &str) -> bool {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() || words.len() > 2 {
        return false;
    }
    words.iter().all(|w| {
        let mut cs = w.chars();
        cs.next().is_some_and(is_name_start) && cs.all(is_name_char)
    })
}

/// Trims `text` (found at `start`) and returns it with its span.
fn trimmed(text: &str, start: usize) -> Line<'_> {
    let lead = text.len() - text.trim_start().len();
    let t = text.trim();
    Line {
        text: t,
        span: Span::new(start + lead, start + lead + t.len()),
    }
}

/// Splits `text` at `;` into statements.
fn split_statements<'a>(text: &'a str, start: usize, out: &mut Vec<Node<'a>>) {
    let mut from = 0;
    for (i, c) in text.char_indices() {
        if c == ';' {
            push_line(&text[from..i], start + from, out);
            from = i + 1;
        }
    }
    push_line(&text[from..], start + from, out);
}

fn push_line<'a>(text: &'a str, start: usize, out: &mut Vec<Node<'a>>) {
    let l = trimmed(text, start);
    if !l.text.is_empty() {
        out.push(Node::Line(l));
    }
}

/// Reads the whole text into lines and blocks.
pub fn layout<'a>(src: &'a str, diags: &mut Vec<Diagnostic>) -> Vec<Node<'a>> {
    // A stack of open blocks; the bottom entry is the file itself.
    let mut stack: Vec<(Option<Line<'a>>, Vec<Node<'a>>)> = vec![(None, Vec::new())];
    let mut offset = 0;
    for raw in src.split_inclusive('\n') {
        let start = offset;
        offset += raw.len();
        let body = raw.trim_end_matches(['\n', '\r']);
        let body = match body.find("//") {
            Some(i) => &body[..i],
            None => body,
        };
        let line = trimmed(body, start);
        if line.text.is_empty() {
            continue;
        }
        if line.text == "}" {
            if stack.len() > 1 {
                let (header, items) = stack.pop().expect("stack has an open block");
                let header = header.expect("open blocks have a header");
                let block = Block {
                    span: header.span.join(line.span),
                    header,
                    body: items,
                };
                stack.last_mut().unwrap().1.push(Node::Block(block));
            } else {
                diags.push(Diagnostic::error(line.span, Message::StrayClose));
            }
            continue;
        }
        if let Some(open) = line.text.find('{') {
            let head = &line.text[..open];
            if is_block_header(head) {
                let header = trimmed(head, line.span.start);
                let rest = &line.text[open + 1..];
                let rest_start = line.span.start + open + 1;
                if rest.trim().is_empty() {
                    stack.push((Some(header), Vec::new()));
                    continue;
                }
                if let Some(close) = rest.rfind('}') {
                    if rest[close + 1..].trim().is_empty() {
                        let mut items = Vec::new();
                        split_statements(&rest[..close], rest_start, &mut items);
                        stack.last_mut().unwrap().1.push(Node::Block(Block {
                            header,
                            body: items,
                            span: line.span,
                        }));
                        continue;
                    }
                }
                // A block opened with statements after the brace.
                let mut items = Vec::new();
                split_statements(rest, rest_start, &mut items);
                stack.push((Some(header), items));
                continue;
            }
        }
        split_statements(line.text, line.span.start, &mut stack.last_mut().unwrap().1);
    }
    while stack.len() > 1 {
        let (header, items) = stack.pop().unwrap();
        let header = header.unwrap();
        diags.push(Diagnostic::error(
            header.span,
            Message::UnclosedBlock {
                name: header.text.to_string(),
            },
        ));
        let span = items.iter().fold(header.span, |s, n| match n {
            Node::Line(l) => s.join(l.span),
            Node::Block(b) => s.join(b.span),
        });
        stack.last_mut().unwrap().1.push(Node::Block(Block {
            header,
            body: items,
            span,
        }));
    }
    stack.pop().unwrap().1
}

/// Reads the words of one statement.
#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    text: &'a str,
    base: usize,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(line: Line<'a>) -> Cursor<'a> {
        Cursor {
            text: line.text,
            base: line.span.start,
            pos: 0,
        }
    }

    pub fn from_str(text: &'a str, base: usize) -> Cursor<'a> {
        Cursor { text, base, pos: 0 }
    }

    /// Byte offset in the source of the current position.
    pub fn offset(&self) -> usize {
        self.base + self.pos
    }

    pub fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.offset())
    }

    pub fn skip_ws(&mut self) {
        let rest = &self.text[self.pos..];
        self.pos += rest.len() - rest.trim_start().len();
    }

    pub fn at_end(&mut self) -> bool {
        self.skip_ws();
        self.pos >= self.text.len()
    }

    pub fn peek(&mut self) -> Option<char> {
        self.skip_ws();
        self.text[self.pos..].chars().next()
    }

    /// The rest of the statement, trimmed, with its span.
    pub fn rest(&mut self) -> Line<'a> {
        self.skip_ws();
        let l = trimmed(&self.text[self.pos..], self.offset());
        self.pos = self.text.len();
        l
    }

    /// Consumes `s` if the text continues with it.
    pub fn eat(&mut self, s: &str) -> bool {
        self.skip_ws();
        if self.text[self.pos..].starts_with(s) {
            self.pos += s.len();
            true
        } else {
            false
        }
    }

    /// Consumes the word `kw` (ignoring case) if it comes next as a whole word.
    pub fn keyword(&mut self, kw: &str) -> bool {
        let save = self.pos;
        match self.name() {
            Some((w, _)) if w.eq_ignore_ascii_case(kw) => true,
            _ => {
                self.pos = save;
                false
            }
        }
    }

    /// Whether the word `kw` comes next, without consuming it.
    pub fn peek_keyword(&mut self, kw: &str) -> bool {
        let save = self.pos;
        let found = self.keyword(kw);
        self.pos = save;
        found
    }

    /// A name: a letter or `_`, then letters, digits and `_`.
    pub fn name(&mut self) -> Option<(&'a str, Span)> {
        self.skip_ws();
        let rest = &self.text[self.pos..];
        let mut cs = rest.char_indices();
        match cs.next() {
            Some((_, c)) if is_name_start(c) => {}
            _ => return None,
        }
        let end = cs
            .find(|&(_, c)| !is_name_char(c))
            .map_or(rest.len(), |(i, _)| i);
        let start = self.offset();
        self.pos += end;
        Some((&rest[..end], Span::new(start, start + end)))
    }

    /// Names joined by dots: `bass.reverb.mix`.
    pub fn path(&mut self) -> Option<(Vec<&'a str>, Span)> {
        let (first, mut span) = self.name()?;
        let mut parts = vec![first];
        loop {
            let save = self.pos;
            if self.text[self.pos..].starts_with('.') {
                self.pos += 1;
                if let Some((n, s)) = self.name_here() {
                    parts.push(n);
                    span = span.join(s);
                    continue;
                }
            }
            self.pos = save;
            break;
        }
        Some((parts, span))
    }

    /// A name starting exactly at the current position (no spaces skipped).
    fn name_here(&mut self) -> Option<(&'a str, Span)> {
        let rest = &self.text[self.pos..];
        if !rest.chars().next().is_some_and(is_name_start) {
            return None;
        }
        self.name()
    }

    /// A number: an optional sign, digits, an optional decimal part.
    pub fn number(&mut self) -> Option<(f64, Span)> {
        self.skip_ws();
        let rest = &self.text[self.pos..];
        let bytes = rest.as_bytes();
        let mut i = 0;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        let digits_start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i < bytes.len()
            && bytes[i] == b'.'
            && i + 1 < bytes.len()
            && bytes[i + 1].is_ascii_digit()
        {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
        }
        if i == digits_start {
            return None;
        }
        let value: f64 = rest[..i].parse().ok()?;
        let start = self.offset();
        self.pos += i;
        Some((value, Span::new(start, start + i)))
    }

    /// A whole number without sign.
    pub fn integer(&mut self) -> Option<(i64, Span)> {
        let save = self.pos;
        match self.number() {
            Some((v, s))
                if v.fract() == 0.0
                    && v >= 0.0
                    && !self.text[save..].trim_start().starts_with('+') =>
            {
                Some((v as i64, s))
            }
            _ => {
                self.pos = save;
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines<'a>(nodes: &[Node<'a>]) -> Vec<&'a str> {
        nodes
            .iter()
            .map(|n| match n {
                Node::Line(l) => l.text,
                Node::Block(b) => b.header.text,
            })
            .collect()
    }

    #[test]
    fn lines_comments_and_blocks() {
        let src = "meter: 4/4\n\ndrums {\n  hh = cc cc co cc\n  sd = - s - S   // backbeat\n}\nbass = | a2 _ e2 _ |\n";
        let mut d = Vec::new();
        let nodes = layout(src, &mut d);
        assert!(d.is_empty());
        assert_eq!(
            lines(&nodes),
            ["meter: 4/4", "drums", "bass = | a2 _ e2 _ |"]
        );
        let Node::Block(b) = &nodes[1] else { panic!() };
        assert_eq!(lines(&b.body), ["hh = cc cc co cc", "sd = - s - S"]);
        let Node::Line(l) = &b.body[1] else { panic!() };
        assert_eq!(&src[l.span.start..l.span.end], "sd = - s - S");
    }

    #[test]
    fn braces_of_a_choice_are_not_a_block() {
        let mut d = Vec::new();
        let nodes = layout("hh = {cc|co} cc cc cc\nbass = | {c2|g1} _ c2 _ |", &mut d);
        assert!(d.is_empty());
        assert_eq!(nodes.len(), 2);
        assert!(nodes.iter().all(|n| matches!(n, Node::Line(_))));
    }

    #[test]
    fn one_line_block_with_semicolons() {
        let mut d = Vec::new();
        let nodes = layout("section verse { bass = | c2 | ; hh = cc cc cc cc }", &mut d);
        assert!(d.is_empty());
        let Node::Block(b) = &nodes[0] else { panic!() };
        assert_eq!(b.header.text, "section verse");
        assert_eq!(lines(&b.body), ["bass = | c2 |", "hh = cc cc cc cc"]);
    }

    #[test]
    fn nested_blocks() {
        let mut d = Vec::new();
        let nodes = layout("section a {\n  drums {\n    hh = c c c c\n  }\n}\n", &mut d);
        assert!(d.is_empty());
        let Node::Block(a) = &nodes[0] else { panic!() };
        let Node::Block(drums) = &a.body[0] else {
            panic!()
        };
        assert_eq!(drums.header.text, "drums");
        assert_eq!(lines(&drums.body), ["hh = c c c c"]);
    }

    #[test]
    fn unclosed_and_stray_braces() {
        let mut d = Vec::new();
        layout("drums {\n hh = c c c c\n", &mut d);
        assert_eq!(
            d[0].message,
            Message::UnclosedBlock {
                name: "drums".into()
            }
        );
        let mut d = Vec::new();
        layout("}\n", &mut d);
        assert_eq!(d[0].message, Message::StrayClose);
    }

    #[test]
    fn cursor_reads_words_paths_and_numbers() {
        let mut c = Cursor::from_str("bass.reverb.mix = 0.42 Hz", 10);
        let (p, s) = c.path().unwrap();
        assert_eq!(p, ["bass", "reverb", "mix"]);
        assert_eq!(s, Span::new(10, 25));
        assert!(c.eat("="));
        assert_eq!(c.number().unwrap().0, 0.42);
        assert!(c.keyword("hz"));
        assert!(c.at_end());

        let mut c = Cursor::from_str("at bar 30: tempo 130", 0);
        assert!(c.keyword("at"));
        assert!(!c.keyword("tempo"));
        assert!(c.keyword("bar"));
        assert_eq!(c.integer().unwrap().0, 30);
        assert!(c.eat(":"));
        assert!(c.peek_keyword("tempo"));
        assert_eq!(c.rest().text, "tempo 130");

        let mut c = Cursor::from_str("transpose +12 -3", 0);
        c.name();
        assert_eq!(c.number().unwrap().0, 12.0);
        assert_eq!(c.number().unwrap().0, -3.0);
    }
}
