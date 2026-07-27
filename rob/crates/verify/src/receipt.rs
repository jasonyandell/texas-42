//! Deterministic receipt writer (BRIEF §9).
//!
//! Receipts are plain deterministic UTF-8 text in the ingest style:
//! `rob <stage> verification: PASS` followed by `name: exact-number` lines.
//! Once committed, formats change only by explicit recorded decision.

/// Format a nonnegative integer with thousands separators, ingest style
/// (e.g. `737,100`).
pub fn fmt_commas(n: u128) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    let offset = digits.len() % 3;
    for (i, ch) in digits.chars().enumerate() {
        if i != 0 && (i + 3 - offset).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

/// A deterministic stage receipt under construction.
pub struct Receipt {
    buf: String,
}

impl Receipt {
    /// Start a stage receipt with the canonical PASS header.
    pub fn new(stage: &str) -> Receipt {
        Receipt {
            buf: format!("rob {stage} verification: PASS\n"),
        }
    }

    /// Append one `name: value` line.
    pub fn line(&mut self, name: &str, value: &str) {
        self.buf.push_str(name);
        self.buf.push_str(": ");
        self.buf.push_str(value);
        self.buf.push('\n');
    }

    /// The finished receipt text.
    pub fn finish(self) -> String {
        self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::fmt_commas;

    #[test]
    fn commas() {
        assert_eq!(fmt_commas(0), "0");
        assert_eq!(fmt_commas(999), "999");
        assert_eq!(fmt_commas(1000), "1,000");
        assert_eq!(fmt_commas(737_100), "737,100");
        assert_eq!(fmt_commas(472_518_347_558_400), "472,518,347,558,400");
    }
}
