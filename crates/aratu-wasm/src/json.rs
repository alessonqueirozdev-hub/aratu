//! A small JSON writer, enough for the answers this module gives.

pub struct Json {
    out: String,
    /// For each open container, whether a value was already written in it.
    filled: Vec<bool>,
    /// A key was just written, so the next value follows it without a comma.
    after_key: bool,
}

impl Default for Json {
    fn default() -> Json {
        Json::new()
    }
}

impl Json {
    pub fn new() -> Json {
        Json {
            out: String::with_capacity(1024),
            filled: vec![false],
            after_key: false,
        }
    }

    /// Writes the comma that goes before a value or key, if one is due.
    fn before_value(&mut self) {
        if self.after_key {
            self.after_key = false;
            return;
        }
        if let Some(f) = self.filled.last_mut() {
            if *f {
                self.out.push(',');
            }
            *f = true;
        }
    }

    pub fn open_obj(&mut self) -> &mut Json {
        self.before_value();
        self.out.push('{');
        self.filled.push(false);
        self
    }

    pub fn close_obj(&mut self) -> &mut Json {
        self.out.push('}');
        self.filled.pop();
        self
    }

    pub fn open_arr(&mut self) -> &mut Json {
        self.before_value();
        self.out.push('[');
        self.filled.push(false);
        self
    }

    pub fn close_arr(&mut self) -> &mut Json {
        self.out.push(']');
        self.filled.pop();
        self
    }

    pub fn key(&mut self, name: &str) -> &mut Json {
        self.before_value();
        escape(name, &mut self.out);
        self.out.push(':');
        self.after_key = true;
        self
    }

    pub fn str(&mut self, s: &str) -> &mut Json {
        self.before_value();
        escape(s, &mut self.out);
        self
    }

    pub fn num(&mut self, n: f64) -> &mut Json {
        self.before_value();
        if !n.is_finite() {
            self.out.push('0');
        } else if n.fract() == 0.0 && n.abs() < 1e15 {
            self.out.push_str(&(n as i64).to_string());
        } else {
            self.out.push_str(&n.to_string());
        }
        self
    }

    pub fn bool(&mut self, b: bool) -> &mut Json {
        self.before_value();
        self.out.push_str(if b { "true" } else { "false" });
        self
    }

    pub fn finish(self) -> String {
        self.out
    }
}

fn escape(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_nested_values_with_commas() {
        let mut j = Json::new();
        j.open_obj();
        j.key("a").num(1.0);
        j.key("b").open_arr();
        j.num(0.5).str("x\"y").bool(true);
        j.open_obj().key("c").str("é\n").close_obj();
        j.close_arr();
        j.key("d").num(f64::NAN);
        j.close_obj();
        assert_eq!(
            j.finish(),
            r#"{"a":1,"b":[0.5,"x\"y",true,{"c":"é\n"}],"d":0}"#
        );
    }
}
