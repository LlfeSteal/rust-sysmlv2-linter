//! Diagnostics : spans, sévérités, rendu humain.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }
    pub fn label_fr(self) -> &'static str {
        match self {
            Severity::Error => "erreur",
            Severity::Warning => "avertissement",
            Severity::Info => "info",
        }
    }
    pub fn gitlab(self) -> &'static str {
        match self {
            Severity::Error => "major",
            Severity::Warning => "minor",
            Severity::Info => "info",
        }
    }
    pub fn rank(self) -> u8 {
        match self {
            Severity::Error => 0,
            Severity::Warning => 1,
            Severity::Info => 2,
        }
    }
}

/// Position dans un fichier. Les offsets sont exprimés en **caractères** (pas en octets).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub file: u32,
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

impl Span {
    pub fn dummy() -> Span {
        Span {
            file: 0,
            start: 0,
            end: 0,
            line: 1,
            col: 1,
            end_line: 1,
            end_col: 1,
        }
    }

    pub fn join(a: Span, b: Span) -> Span {
        if b.start < a.start {
            return Span::join(b, a);
        }
        Span {
            file: a.file,
            start: a.start,
            end: if b.end > a.end { b.end } else { a.end },
            line: a.line,
            col: a.col,
            end_line: b.end_line,
            end_col: b.end_col,
        }
    }

    pub fn len(&self) -> usize {
        if self.end > self.start {
            self.end - self.start
        } else {
            1
        }
    }
}

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub code: &'static str,
    pub rule: &'static str,
    pub severity: Severity,
    pub message: String,
    pub hint: Option<String>,
    pub span: Span,
}

impl Diagnostic {
    pub fn new(
        code: &'static str,
        rule: &'static str,
        severity: Severity,
        span: Span,
        message: String,
    ) -> Diagnostic {
        Diagnostic {
            code,
            rule,
            severity,
            message,
            hint: None,
            span,
        }
    }

    pub fn error(
        code: &'static str,
        rule: &'static str,
        span: Span,
        message: String,
    ) -> Diagnostic {
        Diagnostic::new(code, rule, Severity::Error, span, message)
    }

    pub fn warn(code: &'static str, rule: &'static str, span: Span, message: String) -> Diagnostic {
        Diagnostic::new(code, rule, Severity::Warning, span, message)
    }

    pub fn info(code: &'static str, rule: &'static str, span: Span, message: String) -> Diagnostic {
        Diagnostic::new(code, rule, Severity::Info, span, message)
    }

    pub fn hint(mut self, h: String) -> Diagnostic {
        self.hint = Some(h);
        self
    }
}

/// Récupère le texte d'une ligne (1-indexée), tabulations remplacées par des espaces.
pub fn line_text(src: &str, line: u32) -> String {
    let idx = if line == 0 { 0 } else { (line - 1) as usize };
    let raw = src.lines().nth(idx).unwrap_or("");
    raw.replace('\t', "    ")
}

fn tab_adjusted_col(raw_line: &str, col: u32) -> u32 {
    // Chaque tabulation avant la colonne compte pour 4 caractères à l'affichage.
    let mut extra = 0u32;
    for (i, c) in raw_line.chars().enumerate() {
        if (i as u32) + 1 >= col {
            break;
        }
        if c == '\t' {
            extra += 3;
        }
    }
    col + extra
}

pub struct Palette {
    pub red: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub cyan: &'static str,
    pub dim: &'static str,
    pub bold: &'static str,
    pub reset: &'static str,
}

pub fn palette(color: bool) -> Palette {
    if color {
        Palette {
            red: "\x1b[31m",
            yellow: "\x1b[33m",
            blue: "\x1b[34m",
            cyan: "\x1b[36m",
            dim: "\x1b[2m",
            bold: "\x1b[1m",
            reset: "\x1b[0m",
        }
    } else {
        Palette {
            red: "",
            yellow: "",
            blue: "",
            cyan: "",
            dim: "",
            bold: "",
            reset: "",
        }
    }
}

/// Rendu type « rustc » d'un diagnostic.
pub fn render_human(d: &Diagnostic, path: &str, src: &str, color: bool) -> String {
    let p = palette(color);
    let sev_color = match d.severity {
        Severity::Error => p.red,
        Severity::Warning => p.yellow,
        Severity::Info => p.blue,
    };

    let mut out = String::new();
    out.push_str(&format!(
        "{}{}{}[{}]{}: {}{}{}\n",
        p.bold,
        sev_color,
        d.severity.label_fr(),
        d.code,
        p.reset,
        p.bold,
        d.message,
        p.reset
    ));

    let line_no = d.span.line;
    let gutter_w = format!("{line_no}").len().max(2);
    let pad = " ".repeat(gutter_w);

    out.push_str(&format!(
        "{}{}--> {}:{}:{}{}\n",
        p.dim, pad, path, d.span.line, d.span.col, p.reset
    ));

    let raw = src
        .lines()
        .nth((line_no.saturating_sub(1)) as usize)
        .unwrap_or("");
    let shown = line_text(src, line_no);
    let disp_col = tab_adjusted_col(raw, d.span.col);

    out.push_str(&format!("{}{} |{}\n", p.dim, pad, p.reset));
    out.push_str(&format!(
        "{}{:>w$} |{} {}\n",
        p.dim,
        line_no,
        p.reset,
        shown,
        w = gutter_w
    ));

    let caret_len = if d.span.end_line == d.span.line && d.span.end_col > d.span.col {
        (d.span.end_col - d.span.col) as usize
    } else {
        1
    };
    let caret = "^".repeat(caret_len.clamp(1, 200));
    out.push_str(&format!(
        "{}{} |{} {}{}{}{}\n",
        p.dim,
        pad,
        p.reset,
        " ".repeat(disp_col.saturating_sub(1) as usize),
        sev_color,
        caret,
        p.reset
    ));

    if let Some(h) = &d.hint {
        out.push_str(&format!(
            "{}{} ={} {}aide{}: {}\n",
            p.dim, pad, p.reset, p.cyan, p.reset, h
        ));
    }
    out.push_str(&format!(
        "{}{} ={} règle: {}{}{}\n",
        p.dim, pad, p.reset, p.dim, d.rule, p.reset
    ));
    out
}
