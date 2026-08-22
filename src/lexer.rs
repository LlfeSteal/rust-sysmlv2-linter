//! Analyse lexicale de la notation textuelle SysML v2 / KerML.

use crate::diag::{Diagnostic, Span};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokKind {
    Ident,
    Number,
    Str,
    Punct,
    BlockComment,
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokKind,
    pub text: String,
    /// Vrai pour les « unrestricted names » écrits entre apostrophes : `'ma part'`.
    pub quoted: bool,
    pub span: Span,
}

impl Token {
    pub fn is_ident(&self, s: &str) -> bool {
        self.kind == TokKind::Ident && !self.quoted && self.text == s
    }
    pub fn is_punct(&self, s: &str) -> bool {
        self.kind == TokKind::Punct && self.text == s
    }
}

const PUNCT3: &[&str] = &[":>>", "::>", "..."];
const PUNCT2: &[&str] = &[
    "::", ":>", "..", "->", "=>", "==", "!=", "<=", ">=", "&&", "||", "**", ":=", "|=", "&=", "<>",
    "??", "^^",
];

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

fn is_single_punct(c: char) -> bool {
    matches!(
        c,
        '{' | '}'
            | '['
            | ']'
            | '('
            | ')'
            | ';'
            | ','
            | ':'
            | '.'
            | '='
            | '<'
            | '>'
            | '+'
            | '-'
            | '*'
            | '/'
            | '%'
            | '!'
            | '&'
            | '|'
            | '?'
            | '@'
            | '#'
            | '~'
            | '^'
    )
}

/// Décode une séquence d'échappement `\x` (le jeu défini par la grammaire :
/// `\b \t \n \f \r \" \' \\`) ; tout autre caractère passe inchangé.
fn decode_escape(c: char) -> char {
    match c {
        'b' => '\u{08}',
        't' => '\t',
        'n' => '\n',
        'f' => '\u{0C}',
        'r' => '\r',
        other => other,
    }
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: u32,
    col: u32,
    file: u32,
    diags: Vec<Diagnostic>,
}

impl Lexer {
    pub fn new(src: &str, file: u32) -> Lexer {
        Lexer {
            chars: src.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            file,
            diags: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_at(&self, k: usize) -> Option<char> {
        self.chars.get(self.pos + k).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if let Some(ch) = c {
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        c
    }

    fn span_from(&self, start: usize, line: u32, col: u32) -> Span {
        Span {
            file: self.file,
            start,
            end: self.pos,
            line,
            col,
            end_line: self.line,
            end_col: self.col,
        }
    }

    fn starts_with(&self, s: &str) -> bool {
        for (i, c) in s.chars().enumerate() {
            if self.peek_at(i) != Some(c) {
                return false;
            }
        }
        true
    }

    fn skip_trivia(&mut self) {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() => {
                    self.bump();
                }
                Some('/') if self.peek_at(1) == Some('/') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.bump();
                    }
                }
                _ => break,
            }
        }
    }

    pub fn tokenize(mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        let mut toks: Vec<Token> = Vec::new();
        loop {
            self.skip_trivia();
            let start = self.pos;
            let line = self.line;
            let col = self.col;

            let c = match self.peek() {
                Some(c) => c,
                None => {
                    toks.push(Token {
                        kind: TokKind::Eof,
                        text: String::new(),
                        quoted: false,
                        span: self.span_from(start, line, col),
                    });
                    break;
                }
            };

            if c == '/' && self.peek_at(1) == Some('*') {
                self.bump();
                self.bump();
                let mut body = String::new();
                let mut closed = false;
                while let Some(ch) = self.peek() {
                    if ch == '*' && self.peek_at(1) == Some('/') {
                        self.bump();
                        self.bump();
                        closed = true;
                        break;
                    }
                    body.push(ch);
                    self.bump();
                }
                let span = self.span_from(start, line, col);
                if !closed {
                    self.diags.push(Diagnostic::error(
                        "E001",
                        "unterminated-block-comment",
                        span,
                        "commentaire de bloc non terminé".to_string(),
                    ).hint("ferme le commentaire avec `*/`".to_string()));
                }
                toks.push(Token {
                    kind: TokKind::BlockComment,
                    text: body,
                    quoted: false,
                    span,
                });
                continue;
            }

            if is_ident_start(c) {
                let mut s = String::new();
                while let Some(ch) = self.peek() {
                    if is_ident_continue(ch) {
                        s.push(ch);
                        self.bump();
                    } else {
                        break;
                    }
                }
                toks.push(Token {
                    kind: TokKind::Ident,
                    text: s,
                    quoted: false,
                    span: self.span_from(start, line, col),
                });
                continue;
            }

            if c == '\'' {
                self.bump();
                let mut s = String::new();
                let mut closed = false;
                while let Some(ch) = self.peek() {
                    if ch == '\\' {
                        self.bump();
                        if let Some(esc) = self.bump() {
                            s.push(decode_escape(esc));
                        }
                        continue;
                    }
                    if ch == '\'' {
                        self.bump();
                        closed = true;
                        break;
                    }
                    if ch == '\n' {
                        break;
                    }
                    s.push(ch);
                    self.bump();
                }
                let span = self.span_from(start, line, col);
                if !closed {
                    self.diags.push(Diagnostic::error(
                        "E002",
                        "unterminated-name",
                        span,
                        "nom entre apostrophes non terminé".to_string(),
                    ));
                }
                toks.push(Token {
                    kind: TokKind::Ident,
                    text: s,
                    quoted: true,
                    span,
                });
                continue;
            }

            if c == '"' {
                self.bump();
                let mut s = String::new();
                let mut closed = false;
                while let Some(ch) = self.peek() {
                    if ch == '\\' {
                        self.bump();
                        if let Some(esc) = self.bump() {
                            s.push(decode_escape(esc));
                        }
                        continue;
                    }
                    if ch == '"' {
                        self.bump();
                        closed = true;
                        break;
                    }
                    s.push(ch);
                    self.bump();
                }
                let span = self.span_from(start, line, col);
                if !closed {
                    self.diags.push(Diagnostic::error(
                        "E002",
                        "unterminated-string",
                        span,
                        "chaîne de caractères non terminée".to_string(),
                    ));
                }
                toks.push(Token {
                    kind: TokKind::Str,
                    text: s,
                    quoted: false,
                    span,
                });
                continue;
            }

            if c.is_ascii_digit() {
                let mut s = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        s.push(ch);
                        self.bump();
                    } else {
                        break;
                    }
                }
                // Partie décimale : uniquement si `.` est suivi d'un chiffre
                // (sinon on casserait la multiplicité `1..5`).
                if self.peek() == Some('.') {
                    if let Some(d) = self.peek_at(1) {
                        if d.is_ascii_digit() {
                            s.push('.');
                            self.bump();
                            while let Some(ch) = self.peek() {
                                if ch.is_ascii_digit() {
                                    s.push(ch);
                                    self.bump();
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                if matches!(self.peek(), Some('e') | Some('E')) {
                    let save_pos = self.pos;
                    let save_line = self.line;
                    let save_col = self.col;
                    let mut exp = String::new();
                    exp.push(self.bump().unwrap_or('e'));
                    if matches!(self.peek(), Some('+') | Some('-')) {
                        exp.push(self.bump().unwrap_or('+'));
                    }
                    let mut digits = 0;
                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_digit() {
                            exp.push(ch);
                            self.bump();
                            digits += 1;
                        } else {
                            break;
                        }
                    }
                    if digits > 0 {
                        s.push_str(&exp);
                    } else {
                        self.pos = save_pos;
                        self.line = save_line;
                        self.col = save_col;
                    }
                }
                toks.push(Token {
                    kind: TokKind::Number,
                    text: s,
                    quoted: false,
                    span: self.span_from(start, line, col),
                });
                continue;
            }

            let mut matched: Option<&'static str> = None;
            for p in PUNCT3 {
                if self.starts_with(p) {
                    matched = Some(p);
                    break;
                }
            }
            if matched.is_none() {
                for p in PUNCT2 {
                    if self.starts_with(p) {
                        matched = Some(p);
                        break;
                    }
                }
            }
            if let Some(p) = matched {
                for _ in 0..p.chars().count() {
                    self.bump();
                }
                toks.push(Token {
                    kind: TokKind::Punct,
                    text: p.to_string(),
                    quoted: false,
                    span: self.span_from(start, line, col),
                });
                continue;
            }

            if is_single_punct(c) {
                self.bump();
                toks.push(Token {
                    kind: TokKind::Punct,
                    text: c.to_string(),
                    quoted: false,
                    span: self.span_from(start, line, col),
                });
                continue;
            }

            // Caractère inattendu : on signale et on avance pour garantir la progression.
            self.bump();
            let span = self.span_from(start, line, col);
            let hint = if c == '«' || c == '»' {
                Some("les stéréotypes UML/SysML v1 (`«...»`) n'existent pas en SysML v2 ; utilise `metadata def` et l'annotation `#MonMetadata`".to_string())
            } else {
                None
            };
            let mut d = Diagnostic::error(
                "E003",
                "unexpected-character",
                span,
                format!("caractère inattendu `{}`", c),
            );
            if let Some(h) = hint {
                d = d.hint(h);
            }
            self.diags.push(d);
        }

        (toks, self.diags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(src: &str) -> Vec<Token> {
        let (t, d) = Lexer::new(src, 0).tokenize();
        assert!(d.is_empty(), "diagnostics inattendus : {:?}", d.iter().map(|x| x.code).collect::<Vec<_>>());
        t
    }

    fn texts(t: &[Token]) -> Vec<&str> {
        t.iter().map(|x| x.text.as_str()).collect()
    }

    #[test]
    fn identifiers_allow_unicode_and_underscore() {
        let t = toks("véhicule_1 Übung _privé");
        assert_eq!(t[0].text, "véhicule_1");
        assert_eq!(t[1].text, "Übung");
        assert_eq!(t[2].text, "_privé");
    }

    #[test]
    fn decimal_number_is_a_single_token() {
        let t = toks("3.14");
        assert_eq!(t[0].kind, TokKind::Number);
        assert_eq!(t[0].text, "3.14");
    }

    #[test]
    fn range_dots_are_not_swallowed_by_a_decimal_number() {
        // `1..5` doit rester `1`, `..`, `5` et non `1.` `.5`.
        let t = toks("[1..5]");
        assert_eq!(&texts(&t)[..5], &["[", "1", "..", "5", "]"]);
    }

    #[test]
    fn number_with_exponent_is_a_single_token() {
        let t = toks("1e10 2E-3 5e+2");
        assert_eq!(t[0].text, "1e10");
        assert_eq!(t[1].text, "2E-3");
        assert_eq!(t[2].text, "5e+2");
    }

    #[test]
    fn dangling_exponent_marker_does_not_consume_following_ident() {
        // `1e` sans chiffre après ne doit pas avaler le `e` : `1` puis `e`.
        let t = toks("1e x");
        assert_eq!(t[0].kind, TokKind::Number);
        assert_eq!(t[0].text, "1");
        assert_eq!(t[1].text, "e");
    }

    #[test]
    fn multi_char_punctuation_uses_longest_match() {
        let t = toks(":>> ::> ...");
        assert_eq!(&texts(&t)[..3], &[":>>", "::>", "..."]);
    }

    #[test]
    fn two_char_punctuation_is_preferred_over_two_singles() {
        let t = toks("::");
        assert_eq!(texts(&t)[0], "::");
    }

    #[test]
    fn quoted_name_supports_backslash_escapes() {
        let t = toks(r"'a\'b'");
        assert_eq!(t[0].text, "a'b");
        assert!(t[0].quoted);
    }

    #[test]
    fn quoted_name_decodes_control_character_escapes() {
        // `\t`/`\n`/`\r`/`\f`/`\b` doivent devenir de vrais caractères de
        // contrôle, pas les lettres littérales `t`/`n`/`r`/`f`/`b`.
        let t = toks(r"'a\tb\nc\rd\fe\bf\\g'");
        assert_eq!(t[0].text, "a\tb\nc\rd\u{0C}e\u{08}f\\g");
    }

    #[test]
    fn double_quoted_string_decodes_control_character_escapes() {
        let (t, d) = Lexer::new(r#""x\ty""#, 0).tokenize();
        assert!(d.is_empty());
        assert_eq!(t[0].text, "x\ty");
    }

    #[test]
    fn unknown_escape_passes_through_unchanged() {
        let t = toks(r"'a\qb'");
        assert_eq!(t[0].text, "aqb");
    }

    #[test]
    fn double_quoted_string_is_not_marked_quoted_identifier() {
        let t = toks(r#""hello""#);
        assert_eq!(t[0].kind, TokKind::Str);
        assert_eq!(t[0].text, "hello");
        assert!(!t[0].quoted);
    }

    #[test]
    fn line_comment_stops_at_newline() {
        let t = toks("part // un commentaire\ndef");
        assert_eq!(t[0].text, "part");
        assert_eq!(t[1].text, "def");
    }

    #[test]
    fn block_comment_is_captured_as_its_own_token() {
        let (t, d) = Lexer::new("/* salut */ part", 0).tokenize();
        assert!(d.is_empty());
        assert_eq!(t[0].kind, TokKind::BlockComment);
        assert_eq!(t[0].text, " salut ");
        assert_eq!(t[1].text, "part");
    }

    #[test]
    fn unterminated_block_comment_reports_e001() {
        let (_t, d) = Lexer::new("/* jamais refermé", 0).tokenize();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].code, "E001");
    }

    #[test]
    fn unterminated_quoted_name_reports_e002() {
        let (_t, d) = Lexer::new("'oups", 0).tokenize();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].code, "E002");
    }

    #[test]
    fn unterminated_double_quoted_string_reports_e002() {
        let (_t, d) = Lexer::new("\"oups", 0).tokenize();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].code, "E002");
    }

    #[test]
    fn stereotype_guillemets_report_e003_with_a_helpful_hint() {
        let (_t, d) = Lexer::new("«block»", 0).tokenize();
        assert_eq!(d.len(), 2);
        assert!(d.iter().all(|x| x.code == "E003"));
        assert!(d[0].hint.as_deref().unwrap_or("").contains("metadata def"));
    }

    #[test]
    fn every_token_has_a_final_eof() {
        let t = toks("package P { }");
        assert_eq!(t.last().unwrap().kind, TokKind::Eof);
    }

    #[test]
    fn empty_source_yields_only_eof() {
        let t = toks("");
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].kind, TokKind::Eof);
    }

    #[test]
    fn tabs_and_crlf_are_treated_as_whitespace() {
        let t = toks("part\t\r\ndef");
        assert_eq!(t[0].text, "part");
        assert_eq!(t[1].text, "def");
    }
}
