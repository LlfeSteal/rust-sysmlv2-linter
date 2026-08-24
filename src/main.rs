#![allow(dead_code)]

//! `sysml-check` — parseur et vérificateur de la notation textuelle SysML v2.
//!
//! Conçu pour être appelé par un agent : sortie JSON stable, codes de règles
//! neutres, aides contenant la correction à appliquer, code de sortie exploitable.

mod ast;
mod diag;
mod json;
mod lexer;
mod model;
mod parser;
mod rules;
mod spec;
mod stdlib;

use std::io::Read;
use std::io::Write;

use crate::ast::{NodeKind, QName, RefUse, Rel};
use crate::diag::{Diagnostic, Severity};
use crate::model::Model;
use crate::rules::{Options, UnresolvedMode};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Format {
    Human,
    Json,
    Gitlab,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Emit {
    Diagnostics,
    Ast,
    Both,
}

struct Cli {
    files: Vec<String>,
    format: Format,
    emit: Emit,
    stdin: bool,
    stdin_name: String,
    pedantic: bool,
    unresolved: UnresolvedMode,
    deny_warnings: bool,
    max_diags: usize,
    color: bool,
    quiet: bool,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli {
            files: Vec::new(),
            format: Format::Human,
            emit: Emit::Diagnostics,
            stdin: false,
            stdin_name: "<stdin>.sysml".to_string(),
            pedantic: false,
            unresolved: UnresolvedMode::Error,
            deny_warnings: false,
            max_diags: 500,
            color: false,
            quiet: false,
        }
    }
}

struct FileInfo {
    path: String,
    src: String,
}

const HELP: &str = r#"sysml-check — parseur / vérificateur SysML v2 (notation textuelle)

USAGE :
    sysml-check [OPTIONS] <FICHIER>...
    sysml-check [OPTIONS] --stdin

OPTIONS :
    -f, --format <human|json|gitlab>   Format de sortie (défaut : human)
        --emit <diagnostics|ast|both>  Contenu émis (défaut : diagnostics)
        --stdin                        Lit le modèle sur l'entrée standard
        --name <NOM>                   Nom de fichier affiché avec --stdin
        --pedantic                     Active les règles de style (W302/W306/W307/W309/W311/W312/W313)
        --unresolved <error|warn|off>  Sévérité des noms non résolus (défaut : error)
        --deny-warnings                Les avertissements deviennent bloquants
        --max-diags <N>                Nombre maximal de diagnostics (défaut : 500)
        --color                        Couleurs ANSI en sortie human
    -q, --quiet                        N'affiche que le résumé
        --list-rules                   Liste le catalogue de règles puis quitte
    -h, --help                         Affiche cette aide
    -V, --version                      Affiche la version

CODES DE SORTIE :
    0  aucune erreur (le modèle est syntaxiquement et sémantiquement cohérent)
    1  au moins une erreur (ou un avertissement avec --deny-warnings)
    2  erreur d'utilisation ou d'entrée/sortie

EXEMPLE (boucle d'auto-correction d'un agent) :
    sysml-check --format json modele.sysml
"#;

fn main() {
    let code = run();
    std::process::exit(code);
}

fn parse_args(args: Vec<String>) -> Result<Cli, String> {
    let mut cli = Cli::default();
    let mut i = 0;
    while i < args.len() {
        let a = args[i].clone();
        match a.as_str() {
            "-h" | "--help" => {
                print!("{HELP}");
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("sysml-check {VERSION}");
                std::process::exit(0);
            }
            "--list-rules" => {
                print_rules();
                std::process::exit(0);
            }
            "-f" | "--format" => {
                i += 1;
                let v = args.get(i).ok_or("--format attend une valeur")?;
                cli.format = match v.as_str() {
                    "human" => Format::Human,
                    "json" => Format::Json,
                    "gitlab" => Format::Gitlab,
                    other => return Err(format!("format inconnu : {other}")),
                };
            }
            "--emit" => {
                i += 1;
                let v = args.get(i).ok_or("--emit attend une valeur")?;
                cli.emit = match v.as_str() {
                    "diagnostics" => Emit::Diagnostics,
                    "ast" => Emit::Ast,
                    "both" => Emit::Both,
                    other => return Err(format!("valeur --emit inconnue : {other}")),
                };
            }
            "--stdin" => cli.stdin = true,
            "--name" => {
                i += 1;
                cli.stdin_name = args.get(i).ok_or("--name attend une valeur")?.clone();
            }
            "--pedantic" => cli.pedantic = true,
            "--unresolved" => {
                i += 1;
                let v = args.get(i).ok_or("--unresolved attend une valeur")?;
                cli.unresolved = match v.as_str() {
                    "error" => UnresolvedMode::Error,
                    "warn" => UnresolvedMode::Warn,
                    "off" => UnresolvedMode::Off,
                    other => return Err(format!("valeur --unresolved inconnue : {other}")),
                };
            }
            "--deny-warnings" => cli.deny_warnings = true,
            "--max-diags" => {
                i += 1;
                let v = args.get(i).ok_or("--max-diags attend une valeur")?;
                cli.max_diags = v
                    .parse::<usize>()
                    .map_err(|_| "--max-diags attend un entier".to_string())?;
            }
            "--color" => cli.color = true,
            "-q" | "--quiet" => cli.quiet = true,
            other => {
                if other.starts_with('-') && other.len() > 1 {
                    return Err(format!("option inconnue : {other}"));
                }
                cli.files.push(other.to_string());
            }
        }
        i += 1;
    }
    if std::env::var("NO_COLOR").is_ok() {
        cli.color = false;
    }
    Ok(cli)
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print!("{HELP}");
        return 2;
    }
    let cli = match parse_args(args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("sysml-check: {e}");
            return 2;
        }
    };

    let mut files: Vec<FileInfo> = Vec::new();
    if cli.stdin {
        let mut buf = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
            eprintln!("sysml-check: lecture de stdin impossible : {e}");
            return 2;
        }
        files.push(FileInfo {
            path: cli.stdin_name.clone(),
            src: buf,
        });
    }
    for p in cli.files.iter() {
        match std::fs::read_to_string(p) {
            Ok(src) => files.push(FileInfo {
                path: p.clone(),
                src,
            }),
            Err(e) => {
                eprintln!("sysml-check: {p} : {e}");
                return 2;
            }
        }
    }
    if files.is_empty() {
        eprintln!("sysml-check: aucun fichier à analyser");
        return 2;
    }

    // 1. Analyse lexicale + syntaxique de chaque fichier
    let mut diags: Vec<Diagnostic> = Vec::new();
    let mut roots: Vec<ast::Node> = Vec::new();
    for (idx, f) in files.iter().enumerate() {
        let lx = lexer::Lexer::new(&f.src, idx as u32);
        let (toks, ldiags) = lx.tokenize();
        diags.extend(ldiags);
        let mut p = parser::Parser::new(toks, cli.max_diags);
        let nodes = p.parse_unit();
        diags.extend(p.diags.clone());
        roots.extend(nodes);
    }

    // 2. Table des symboles commune à tous les fichiers, puis règles sémantiques
    let model = Model::build(&roots);
    let opts = Options {
        pedantic: cli.pedantic,
        unresolved: cli.unresolved,
    };
    diags.extend(rules::check(&model, &opts));

    // 3. Tri, dédoublonnage, plafonnement
    diags.sort_by(|a, b| {
        a.span
            .file
            .cmp(&b.span.file)
            .then(a.span.line.cmp(&b.span.line))
            .then(a.span.col.cmp(&b.span.col))
            .then(a.severity.rank().cmp(&b.severity.rank()))
            .then(a.code.cmp(b.code))
    });
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut unique: Vec<Diagnostic> = Vec::new();
    for d in diags.into_iter() {
        let key = format!(
            "{}|{}|{}|{}|{}",
            d.span.file, d.span.line, d.span.col, d.code, d.message
        );
        if seen.insert(key) {
            unique.push(d);
        }
    }
    if unique.len() > cli.max_diags {
        unique.truncate(cli.max_diags);
    }
    let diags = unique;

    let errors = diags
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();
    let warnings = diags
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();
    let infos = diags
        .iter()
        .filter(|d| d.severity == Severity::Info)
        .count();

    // 4. Sortie
    let stdout = std::io::stdout();
    let mut w = stdout.lock();
    match cli.format {
        Format::Human => {
            if !cli.quiet {
                for d in diags.iter() {
                    let f = &files[d.span.file as usize];
                    let _ = write!(w, "{}", diag::render_human(d, &f.path, &f.src, cli.color));
                    let _ = writeln!(w);
                }
            }
            if cli.emit != Emit::Diagnostics {
                let _ = writeln!(w, "{}", emit_ast_json(&model, &files));
            }
            let summary = if errors == 0 && warnings == 0 {
                format!("✔ {} fichier(s) analysé(s) — aucun problème", files.len())
            } else {
                format!(
                    "{} erreur(s), {} avertissement(s), {} info(s) dans {} fichier(s)",
                    errors,
                    warnings,
                    infos,
                    files.len()
                )
            };
            let _ = writeln!(w, "{summary}");
        }
        Format::Json => {
            let _ = writeln!(
                w,
                "{}",
                emit_json(&model, &files, &diags, errors, warnings, infos, cli.emit)
            );
        }
        Format::Gitlab => {
            let _ = writeln!(w, "{}", emit_gitlab(&files, &diags));
        }
    }
    let _ = w.flush();

    if errors > 0 {
        return 1;
    }
    if cli.deny_warnings && warnings > 0 {
        return 1;
    }
    0
}

// --------------------------------------------------------------------------
// Sortie JSON
// --------------------------------------------------------------------------

fn diag_json(d: &Diagnostic, files: &[FileInfo], indent: &str) -> String {
    let path = files
        .get(d.span.file as usize)
        .map(|f| f.path.as_str())
        .unwrap_or("<inconnu>");
    let snippet = diag::line_text(
        files
            .get(d.span.file as usize)
            .map(|f| f.src.as_str())
            .unwrap_or(""),
        d.span.line,
    );
    format!(
        "{i}{{\n\
         {i}  \"file\": {file},\n\
         {i}  \"code\": {code},\n\
         {i}  \"rule\": {rule},\n\
         {i}  \"severity\": {sev},\n\
         {i}  \"message\": {msg},\n\
         {i}  \"hint\": {hint},\n\
         {i}  \"line\": {line},\n\
         {i}  \"column\": {col},\n\
         {i}  \"endLine\": {eline},\n\
         {i}  \"endColumn\": {ecol},\n\
         {i}  \"snippet\": {snip}\n\
         {i}}}",
        i = indent,
        file = json::qs(path),
        code = json::qs(d.code),
        rule = json::qs(d.rule),
        sev = json::qs(d.severity.as_str()),
        msg = json::qs(&d.message),
        hint = json::qs_opt(d.hint.as_deref()),
        line = d.span.line,
        col = d.span.col,
        eline = d.span.end_line,
        ecol = d.span.end_col,
        snip = json::qs(snippet.trim_end()),
    )
}

#[allow(clippy::too_many_arguments)]
fn emit_json(
    model: &Model,
    files: &[FileInfo],
    diags: &[Diagnostic],
    errors: usize,
    warnings: usize,
    infos: usize,
    emit: Emit,
) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"tool\": \"sysml-check\",\n");
    s.push_str(&format!("  \"version\": {},\n", json::qs(VERSION)));
    s.push_str("  \"summary\": {\n");
    s.push_str(&format!("    \"files\": {},\n", files.len()));
    s.push_str(&format!("    \"errors\": {errors},\n"));
    s.push_str(&format!("    \"warnings\": {warnings},\n"));
    s.push_str(&format!("    \"infos\": {infos},\n"));
    s.push_str(&format!(
        "    \"ok\": {}\n",
        if errors == 0 { "true" } else { "false" }
    ));
    s.push_str("  },\n");

    if emit != Emit::Ast {
        s.push_str("  \"diagnostics\": [\n");
        for (i, d) in diags.iter().enumerate() {
            s.push_str(&diag_json(d, files, "    "));
            if i + 1 < diags.len() {
                s.push(',');
            }
            s.push('\n');
        }
        s.push_str("  ]");
    } else {
        s.push_str("  \"diagnostics\": []");
    }

    if emit != Emit::Diagnostics {
        s.push_str(",\n  \"ast\": ");
        s.push_str(&emit_ast_json(model, files));
    }

    s.push_str("\n}");
    s
}

fn rel_json(model: &Model, scope: usize, r: &Rel) -> String {
    let resolved = rules::resolve_for_emit(model, scope, &r.target);
    format!(
        "{{\"kind\": {}, \"token\": {}, \"target\": {}, \"resolved\": {}}}",
        json::qs(r.kind.as_str()),
        json::qs(&r.token),
        json::qs(&r.target.text()),
        json::qs_opt(resolved.as_deref())
    )
}

fn ref_json(model: &Model, scope: usize, r: &RefUse) -> String {
    let resolved = rules::resolve_for_emit(model, scope, &r.qname);
    format!(
        "{{\"context\": {}, \"target\": {}, \"resolved\": {}}}",
        json::qs(r.ctx.as_str()),
        json::qs(&r.qname.text()),
        json::qs_opt(resolved.as_deref())
    )
}

fn qname_line(q: &QName) -> u32 {
    q.span.line
}

fn emit_sym_json(model: &Model, files: &[FileInfo], id: usize, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let s = &model.syms[id];
    let scope = s.parent.unwrap_or(0);
    let path = files
        .get(s.span.file as usize)
        .map(|f| f.path.as_str())
        .unwrap_or("<inconnu>");

    out.push_str(&format!("{pad}{{\n"));
    out.push_str(&format!("{pad}  \"id\": {id},\n"));
    out.push_str(&format!(
        "{}  \"kind\": {},\n",
        pad,
        json::qs(s.kind.as_str())
    ));
    out.push_str(&format!(
        "{}  \"keyword\": {},\n",
        pad,
        json::qs(&s.keyword)
    ));
    out.push_str(&format!(
        "{}  \"name\": {},\n",
        pad,
        json::qs_opt(s.name.as_deref())
    ));
    out.push_str(&format!(
        "{}  \"shortName\": {},\n",
        pad,
        json::qs_opt(s.short_name.as_deref())
    ));
    out.push_str(&format!(
        "{}  \"qualifiedName\": {},\n",
        pad,
        json::qs(&s.qname)
    ));
    out.push_str(&format!("{}  \"isDefinition\": {},\n", pad, s.is_def));
    out.push_str(&format!(
        "{}  \"modifiers\": [{}],\n",
        pad,
        s.prefixes
            .iter()
            .map(|p| json::qs(p))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "{}  \"relationships\": [{}],\n",
        pad,
        s.rels
            .iter()
            .map(|r| rel_json(model, scope, r))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "{}  \"references\": [{}],\n",
        pad,
        s.refs
            .iter()
            .map(|r| ref_json(model, scope, r))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    match &s.mult {
        Some(mu) => {
            let lo = mu.lower.as_ref().map(|l| l.text.clone());
            out.push_str(&format!(
                "{}  \"multiplicity\": {{\"lower\": {}, \"upper\": {}}},\n",
                pad,
                json::qs_opt(lo.as_deref()),
                json::qs(&mu.upper.text)
            ));
        }
        None => {
            out.push_str(&format!("{pad}  \"multiplicity\": null,\n"));
        }
    }
    out.push_str(&format!("{}  \"file\": {},\n", pad, json::qs(path)));
    out.push_str(&format!("{}  \"line\": {},\n", pad, s.span.line));
    out.push_str(&format!("{}  \"column\": {},\n", pad, s.span.col));

    if s.children.is_empty() {
        out.push_str(&format!("{pad}  \"children\": []\n"));
    } else {
        out.push_str(&format!("{pad}  \"children\": [\n"));
        for (i, &c) in s.children.iter().enumerate() {
            emit_sym_json(model, files, c, depth + 2, out);
            if i + 1 < s.children.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str(&format!("{pad}  ]\n"));
    }
    out.push_str(&format!("{pad}}}"));
}

fn emit_ast_json(model: &Model, files: &[FileInfo]) -> String {
    let mut out = String::new();
    let roots = model.syms[0].children.clone();
    out.push_str("[\n");
    for (i, &c) in roots.iter().enumerate() {
        emit_sym_json(model, files, c, 2, &mut out);
        if i + 1 < roots.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ]");
    out
}

// --------------------------------------------------------------------------
// Sortie GitLab Code Quality
// --------------------------------------------------------------------------

fn emit_gitlab(files: &[FileInfo], diags: &[Diagnostic]) -> String {
    let mut s = String::new();
    s.push_str("[\n");
    for (i, d) in diags.iter().enumerate() {
        let path = files
            .get(d.span.file as usize)
            .map(|f| f.path.as_str())
            .unwrap_or("<inconnu>");
        let line = format!("{}", d.span.line);
        let col = format!("{}", d.span.col);
        let fp = json::fingerprint(&[path, d.code, &line, &col, &d.message]);
        let desc = match &d.hint {
            Some(h) => format!("{} — {}", d.message, h),
            None => d.message.clone(),
        };
        s.push_str("  {\"description\": ");
        s.push_str(&json::qs(&desc));
        s.push_str(", \"check_name\": ");
        s.push_str(&json::qs(d.code));
        s.push_str(", \"fingerprint\": ");
        s.push_str(&json::qs(&fp));
        s.push_str(", \"severity\": ");
        s.push_str(&json::qs(d.severity.gitlab()));
        s.push_str(", \"location\": {\"path\": ");
        s.push_str(&json::qs(path));
        s.push_str(", \"lines\": {\"begin\": ");
        s.push_str(&format!("{}", d.span.line));
        s.push_str("}}}");
        if i + 1 < diags.len() {
            s.push(',');
        }
        s.push('\n');
    }
    s.push(']');
    s
}

// --------------------------------------------------------------------------
// Catalogue de règles
// --------------------------------------------------------------------------

fn print_rules() {
    println!("[");
    let n = rules::CATALOG.len();
    for (i, entry) in rules::CATALOG.iter().enumerate() {
        let (code, rule, authority, desc) = *entry;
        print!(
            "  {{\"code\": {}, \"rule\": {}, \"authority\": {}, \"description\": {}}}",
            json::qs(code),
            json::qs(rule),
            json::qs(authority.as_str()),
            json::qs(desc)
        );
        if i + 1 < n {
            print!(",");
        }
        println!();
    }
    println!("]");
}

fn _unused(_: NodeKind) {}
