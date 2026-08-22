//! Écriture JSON minimaliste (aucune dépendance externe).

pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            other => {
                if (other as u32) < 0x20 {
                    out.push_str(&format!("\\u{:04x}", other as u32));
                } else {
                    out.push(other);
                }
            }
        }
    }
    out
}

/// Chaîne JSON entre guillemets.
pub fn qs(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    out.push_str(&escape(s));
    out.push('"');
    out
}

/// `Option<&str>` -> chaîne JSON ou `null`.
pub fn qs_opt(s: Option<&str>) -> String {
    match s {
        Some(v) => qs(v),
        None => "null".to_string(),
    }
}

pub fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

pub fn fingerprint(parts: &[&str]) -> String {
    let joined = parts.join("\u{1}");
    format!("{:016x}", fnv1a(&joined))
}

pub struct Indent(pub usize);

impl Indent {
    pub fn s(&self) -> String {
        "  ".repeat(self.0)
    }
}
