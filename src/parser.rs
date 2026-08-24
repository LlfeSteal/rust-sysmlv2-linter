//! Analyse syntaxique descendante récursive, avec récupération d'erreur :
//! on ne s'arrête pas à la première faute, on en signale le plus possible en une passe.

use crate::ast::*;
use crate::diag::{Diagnostic, Span};
use crate::lexer::{TokKind, Token};

// --------------------------------------------------------------------------
// Tables de mots-clés
// --------------------------------------------------------------------------

/// Mots-clés pouvant introduire un membre.
pub const MEMBER_KWS: &[&str] = &[
    // structure
    "package",
    "import",
    "alias",
    "doc",
    "comment",
    "rep",
    "dependency",
    "filter",
    "namespace",
    "feature",
    "metadata",
    "specialization",
    "subclassification",
    // définitions / usages
    "part",
    "item",
    "attribute",
    "port",
    "interface",
    "connection",
    "action",
    "state",
    "calc",
    "constraint",
    "requirement",
    "concern",
    "case",
    "enum",
    "occurrence",
    "allocation",
    "flow",
    "view",
    "viewpoint",
    "rendering",
    "ref",
    "event",
    "message",
    "snapshot",
    "timeslice",
    "individual",
    "analysis",
    "verification",
    "use",
    // relations
    "connect",
    "bind",
    "binding",
    "succession",
    "transition",
    "satisfy",
    "allocate",
    // comportement / exigences
    "perform",
    "exhibit",
    "include",
    "assert",
    "assume",
    "require",
    "verify",
    "frame",
    "expose",
    "render",
    "subject",
    "actor",
    "stakeholder",
    "objective",
    "return",
    "accept",
    "send",
    "assign",
    "entry",
    "exit",
    "do",
    "then",
    "first",
    "if",
    "else",
    "for",
    "while",
    "loop",
    "decide",
    "merge",
    "fork",
    "join",
    "terminate",
    "end",
    "in",
    "out",
    "inout",
    "variant",
    // modificateurs qui peuvent aussi commencer un membre
    "abstract",
    "variation",
    "readonly",
    "derived",
    "constant",
    "composite",
    "portion",
    "ordered",
    "nonunique",
    "public",
    "private",
    "protected",
    "standard",
    "library",
    // hérités SysML v1 / UML : acceptés pour pouvoir les diagnostiquer proprement
    "block",
    "value",
    "class",
    "association",
    "stereotype",
    "property",
    "operation",
];

/// Modificateurs pouvant précéder un mot-clé de membre.
pub const PREFIX_KWS: &[&str] = &[
    "abstract",
    "variation",
    "variant",
    "readonly",
    "derived",
    "constant",
    "end",
    "in",
    "out",
    "inout",
    "ref",
    "individual",
    "snapshot",
    "timeslice",
    "composite",
    "portion",
    "ordered",
    "nonunique",
    "public",
    "private",
    "protected",
    "standard",
];

/// Mots-clés « enveloppe » : `require constraint`, `perform action`, ...
const WRAPPER_KWS: &[&str] = &[
    "require", "assume", "assert", "perform", "exhibit", "include", "satisfy", "verify", "frame",
    "expose", "render", "event", "message", "entry", "exit", "do",
];

/// Membres dont la suite est une expression libre (consommée en bloc).
const BLOB_KWS: &[&str] = &[
    "accept",
    "send",
    "assign",
    "return",
    "if",
    "else",
    "for",
    "while",
    "loop",
    "decide",
    "merge",
    "fork",
    "join",
    "terminate",
    "then",
    "first",
    "entry",
    "exit",
    "do",
    "filter",
    "assert",
    "assume",
    "require",
    "expose",
];

/// Mots-clés pouvant suivre une déclaration (`connection c : C connect a to b;`).
const TAIL_KWS: &[&str] = &[
    "connect",
    "from",
    "to",
    "then",
    "first",
    "accept",
    "by",
    "of",
    "about",
    "via",
    "when",
    "references",
    "default",
];

/// Mots-clés d'expression : ne sont jamais des références de noms.
const EXPR_KWS: &[&str] = &[
    "if", "then", "else", "and", "or", "xor", "not", "true", "false", "null", "in", "as", "meta",
    "istype", "hastype", "all", "by", "from", "to", "of", "first", "accept", "when", "via", "that",
    "default", "return", "send", "at", "after", "do", "assign", "new",
];

/// Mots-clés qui ne peuvent pas être un nom déclaré.
const DECL_STOP_KWS: &[&str] = &[
    "specializes",
    "subsets",
    "redefines",
    "references",
    "conjugates",
    "crosses",
    "defined",
    "ordered",
    "nonunique",
    "default",
    "connect",
    "from",
    "to",
    "then",
    "first",
    "accept",
    "by",
    "of",
    "about",
    "for",
    "in",
    "when",
    "via",
    "do",
    "entry",
    "exit",
    "if",
];

/// Mots-clés SysML v1 / UML qui n'existent pas en SysML v2.
pub const LEGACY_KWS: &[&str] = &[
    "block",
    "value",
    "class",
    "association",
    "stereotype",
    "property",
    "operation",
];

pub fn is_member_kw(s: &str) -> bool {
    MEMBER_KWS.contains(&s)
}
pub fn is_prefix_kw(s: &str) -> bool {
    PREFIX_KWS.contains(&s)
}
fn is_wrapper_kw(s: &str) -> bool {
    WRAPPER_KWS.contains(&s)
}
fn is_blob_kw(s: &str) -> bool {
    BLOB_KWS.contains(&s)
}
fn is_tail_kw(s: &str) -> bool {
    TAIL_KWS.contains(&s)
}
fn is_expr_kw(s: &str) -> bool {
    EXPR_KWS.contains(&s)
}
fn is_decl_stop_kw(s: &str) -> bool {
    DECL_STOP_KWS.contains(&s)
}
pub fn is_legacy_kw(s: &str) -> bool {
    LEGACY_KWS.contains(&s)
}

/// Formes de spécialisation qui peuvent *ouvrir* un membre, sans mot-clé
/// devant : `:>> path = "x";`, `:> Base;`, `redefines path;`, ...
///
/// La notation textuelle autorise une `FeatureUsage` anonyme dont toute la
/// déclaration se réduit à sa `FeatureSpecializationPart` ; c'est la forme
/// courante pour affecter une valeur à une caractéristique héritée, en
/// particulier dans le corps d'une métadonnée (`@Meta { :>> path = "..."; }`).
/// Le `:` seul est volontairement exclu : `: Type;` en tête de membre est
/// bien plus probablement un mot-clé oublié qu'une déclaration anonyme, et le
/// diagnostic E100 reste alors plus utile.
fn starts_bare_specialization(tok: &Token) -> bool {
    if tok.kind == TokKind::Punct {
        return matches!(tok.text.as_str(), ":>>" | "::>" | ":>");
    }
    if tok.kind == TokKind::Ident && !tok.quoted {
        return matches!(
            tok.text.as_str(),
            "specializes" | "subsets" | "redefines" | "references"
        );
    }
    false
}

/// Mot réservé *réel* de SysML v2 : ne peut pas servir de nom sans échappement.
/// On exclut les mots hérités de SysML v1 (`block`, `value`, ...) qui ne sont
/// reconnus que pour produire un diagnostic dédié.
pub fn is_reserved_name(s: &str) -> bool {
    is_member_kw(s) && !is_legacy_kw(s)
}

fn is_relationship_kw(kw: &str, is_def: bool) -> bool {
    if is_def {
        return false;
    }
    // Le dernier mot, pas le premier : `assert satisfy` (négation/assertion
    // optionnelles devant une relation, cf. `parse_member`) doit rester
    // reconnu comme relation `satisfy`, pas comme `assert`.
    let last = kw.split(' ').next_back().unwrap_or("");
    matches!(
        last,
        "connect"
            | "bind"
            | "binding"
            | "flow"
            | "succession"
            | "transition"
            | "satisfy"
            | "allocate"
            | "dependency"
    )
}

fn base_ctx_for(kw: &str) -> RefCtx {
    let last = kw.split(' ').next_back().unwrap_or("");
    match last {
        "connect" => RefCtx::ConnectEnd,
        "bind" | "binding" => RefCtx::BindEnd,
        "flow" => RefCtx::FlowEnd,
        "succession" => RefCtx::TransitionSource,
        "transition" => RefCtx::TransitionSource,
        "satisfy" => RefCtx::SatisfyTarget,
        "allocate" => RefCtx::AllocateSource,
        _ => RefCtx::Other,
    }
}

fn ctx_for_word(w: &str, base: RefCtx) -> RefCtx {
    match w {
        "by" => RefCtx::SatisfyBy,
        "then" => RefCtx::TransitionTarget,
        "first" => RefCtx::TransitionSource,
        "about" => RefCtx::About,
        "to" => match base {
            RefCtx::AllocateSource | RefCtx::AllocateTarget => RefCtx::AllocateTarget,
            other => other,
        },
        _ => base,
    }
}

/// Un corps `{ ... }` qui contient une expression plutôt que des membres.
fn kw_allows_expression(kw: &str) -> bool {
    kw.contains("constraint") || kw.contains("calc") || kw.contains("expr")
}

// --------------------------------------------------------------------------
// Parser
// --------------------------------------------------------------------------

pub struct Parser {
    toks: Vec<Token>,
    /// Commentaires de bloc précédant immédiatement `toks[i]`.
    pre: Vec<Vec<Token>>,
    pos: usize,
    pub diags: Vec<Diagnostic>,
    max_diags: usize,
}

impl Parser {
    pub fn new(raw: Vec<Token>, max_diags: usize) -> Parser {
        let mut toks: Vec<Token> = Vec::new();
        let mut pre: Vec<Vec<Token>> = Vec::new();
        let mut pending: Vec<Token> = Vec::new();
        for t in raw {
            if t.kind == TokKind::BlockComment {
                pending.push(t);
                continue;
            }
            toks.push(t);
            pre.push(std::mem::take(&mut pending));
        }
        if toks.is_empty() {
            toks.push(Token {
                kind: TokKind::Eof,
                text: String::new(),
                quoted: false,
                span: Span::dummy(),
            });
            pre.push(pending);
        } else if !pending.is_empty() {
            let n = pre.len();
            pre[n - 1].extend(pending);
        }
        Parser {
            toks,
            pre,
            pos: 0,
            diags: Vec::new(),
            max_diags,
        }
    }

    // ---- primitives ------------------------------------------------------

    fn cur(&self) -> &Token {
        &self.toks[self.pos]
    }

    fn peek(&self, k: usize) -> &Token {
        let last = self.toks.len() - 1;
        let i = if self.pos + k > last {
            last
        } else {
            self.pos + k
        };
        &self.toks[i]
    }

    fn prev_span(&self) -> Span {
        if self.pos > 0 {
            self.toks[self.pos - 1].span
        } else {
            self.toks[0].span
        }
    }

    fn is_eof(&self) -> bool {
        self.cur().kind == TokKind::Eof
    }

    fn bump(&mut self) -> Token {
        let t = self.toks[self.pos].clone();
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        t
    }

    fn at_punct(&self, s: &str) -> bool {
        self.cur().is_punct(s)
    }

    fn at_kw(&self, s: &str) -> bool {
        self.cur().is_ident(s)
    }

    fn eat_punct(&mut self, s: &str) -> bool {
        if self.at_punct(s) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_kw(&mut self, s: &str) -> bool {
        if self.at_kw(s) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn push(&mut self, d: Diagnostic) {
        if self.diags.len() < self.max_diags {
            self.diags.push(d);
        }
    }

    fn describe_cur(&self) -> String {
        let t = self.cur();
        match t.kind {
            TokKind::Eof => "la fin du fichier".to_string(),
            TokKind::Str => format!("la chaîne \"{}\"", t.text),
            _ => format!("`{}`", t.text),
        }
    }

    fn err_expected(&mut self, what: &str, ctx: &str) {
        let span = self.cur().span;
        let found = self.describe_cur();
        self.push(Diagnostic::error(
            "E101",
            "expected-token",
            span,
            format!("{what} attendu après `{ctx}`, trouvé {found}"),
        ));
    }

    fn take_doc_body(&mut self) -> Option<Spanned> {
        let bucket = &mut self.pre[self.pos];
        if bucket.is_empty() {
            None
        } else {
            let t = bucket.remove(0);
            Some(Spanned::new(t.text.clone(), t.span))
        }
    }

    /// Saute jusqu'au prochain point de synchronisation (`;` consommé, `}` laissé au parent).
    fn recover(&mut self) {
        let mut depth: i32 = 0;
        loop {
            if self.is_eof() {
                return;
            }
            let t = self.cur().clone();
            if t.kind == TokKind::Punct {
                match t.text.as_str() {
                    "{" | "(" | "[" => {
                        depth += 1;
                    }
                    "}" | ")" | "]" => {
                        if depth == 0 {
                            return;
                        }
                        depth -= 1;
                    }
                    ";" if depth == 0 => {
                        self.bump();
                        return;
                    }
                    _ => {}
                }
            }
            self.bump();
        }
    }

    fn starts_member(&self) -> bool {
        if self.at_punct("#") || self.at_punct("@") {
            return true;
        }
        let t = self.cur();
        if t.kind == TokKind::Ident && !t.quoted && is_member_kw(&t.text) {
            return true;
        }
        if starts_bare_specialization(t) {
            return true;
        }
        // `not satisfy ...` : `not` lui-même n'est pas un mot-clé de membre,
        // seul ce qui le suit l'est (voir la même vérification dans
        // `parse_member`).
        if t.is_ident("not") {
            let nx = self.peek(1);
            return nx.kind == TokKind::Ident && !nx.quoted && is_relationship_kw(&nx.text, false);
        }
        false
    }

    // ---- points d'entrée -------------------------------------------------

    pub fn parse_unit(&mut self) -> Vec<Node> {
        let mut out = Vec::new();
        loop {
            if self.is_eof() {
                break;
            }
            if self.eat_punct(";") {
                continue;
            }
            if self.at_punct("}") {
                let span = self.cur().span;
                self.push(
                    Diagnostic::error(
                        "E102",
                        "unmatched-brace",
                        span,
                        "`}` sans `{` correspondant".to_string(),
                    )
                    .hint("supprime cette accolade ou ajoute le `{` manquant".to_string()),
                );
                self.bump();
                continue;
            }
            let before = self.pos;
            match self.parse_member(0) {
                Some(n) => out.push(n),
                None => self.recover(),
            }
            if self.pos == before {
                self.bump();
            }
        }
        out
    }

    // ---- membres ---------------------------------------------------------

    fn parse_member(&mut self, depth: u32) -> Option<Node> {
        if depth > 64 {
            let span = self.cur().span;
            self.push(Diagnostic::error(
                "E105",
                "nesting-too-deep",
                span,
                "imbrication trop profonde (>64 niveaux)".to_string(),
            ));
            return None;
        }

        let start = self.cur().span;
        let mut node = Node::new(start);

        // Annotation préfixe `#Meta` : ne s'attache qu'au membre qui suit,
        // ce n'est jamais une déclaration autonome (contrairement à `@`,
        // voir plus bas — les deux formes ne sont pas interchangeables).
        while self.at_punct("#") {
            let at = self.bump();
            match self.parse_qname(false) {
                Some(q) => node.refs.push(RefUse {
                    qname: q,
                    ctx: RefCtx::Annotation,
                }),
                None => {
                    let t = at.text.clone();
                    self.err_expected("un nom de métadonnée", &t);
                }
            }
            if self.at_punct("{") {
                let mut tmp = Node::new(self.cur().span);
                self.parse_body(&mut tmp, true, depth + 1);
            }
        }

        // `@Meta;` : usage de métadonnée autonome (`MetadataUsage`,
        // équivalent à `metadata : Meta;`), pas un préfixe — contrairement à
        // `#`, `@` n'introduit jamais le membre suivant.
        if self.at_punct("@") {
            let at = self.bump();
            node.keyword = "metadata".to_string();
            node.keyword_span = at.span;
            node.kind = NodeKind::Usage;
            match self.parse_qname(false) {
                Some(q) => node.rels.push(Rel {
                    kind: RelKind::TypedBy,
                    token: "@".to_string(),
                    op_span: at.span,
                    target: q,
                }),
                None => self.err_expected("un nom de métadonnée", "@"),
            }
            if self.eat_kw("about") {
                loop {
                    match self.parse_qname(false) {
                        Some(q) => node.refs.push(RefUse {
                            qname: q,
                            ctx: RefCtx::About,
                        }),
                        None => {
                            self.err_expected("un nom qualifié", "about");
                            break;
                        }
                    }
                    if self.eat_punct(",") {
                        continue;
                    }
                    break;
                }
            }
            self.finish_member(&mut node, depth);
            node.span = Span::join(start, self.prev_span());
            return Some(node);
        }

        // Modificateurs
        loop {
            let t = self.cur().clone();
            if t.kind != TokKind::Ident || t.quoted || !is_prefix_kw(&t.text) {
                break;
            }
            let nx = self.peek(1).clone();
            let nx_ok = nx.kind == TokKind::Ident
                && !nx.quoted
                && (is_prefix_kw(&nx.text) || is_member_kw(&nx.text));
            if !nx_ok {
                break;
            }
            let tk = self.bump();
            node.prefixes.push(Spanned::new(tk.text.clone(), tk.span));
        }

        // `not satisfy ...` : négation directe d'une relation, sans `assert`.
        // Doit être détecté avant la lecture du mot-clé : `not` lui-même
        // n'est pas un mot-clé de membre, seul ce qui le suit l'est.
        if self.at_kw("not") {
            let after = self.peek(1).clone();
            if after.kind == TokKind::Ident
                && !after.quoted
                && is_relationship_kw(&after.text, false)
            {
                let t = self.bump();
                node.prefixes.push(Spanned::new(t.text.clone(), t.span));
            }
        }

        // `FeatureUsage` anonyme : le membre commence directement par sa partie
        // de spécialisation (`:>> path = "x";`, `:> Base;`, `redefines x;`).
        // Aucun mot-clé n'est présent — on laisse `parse_decl` faire le reste,
        // il sait déjà lire un nom absent puis une suite de relations.
        if starts_bare_specialization(self.cur()) {
            node.keyword_span = self.cur().span;
            node.kind = classify("", false);
            self.parse_decl(&mut node, false);
            self.finish_member(&mut node, depth);
            node.span = Span::join(start, self.prev_span());
            return Some(node);
        }

        // Mot-clé
        let ct = self.cur().clone();
        if ct.kind != TokKind::Ident || ct.quoted || !is_member_kw(&ct.text) {
            let span = ct.span;
            let found = self.describe_cur();
            self.push(
                Diagnostic::error(
                    "E100",
                    "member-must-start-with-keyword",
                    span,
                    format!("membre inattendu : {found}"),
                )
                .hint(
                    "en SysML v2 chaque membre commence par un mot-clé (`part`, `attribute`, \
                     `ref`, `port`, `action`, `doc`, `import`, ...) — par exemple \
                     `attribute masse : ISQ::MassValue;`"
                        .to_string(),
                ),
            );
            return None;
        }

        let kwt = self.bump();
        let mut kw = kwt.text.clone();
        let kw_span = kwt.span;

        if (kw == "use" || kw == "analysis" || kw == "verification") && self.at_kw("case") {
            self.bump();
            kw.push_str(" case");
        }
        if kw == "library" && self.at_kw("package") {
            self.bump();
            kw = "library package".to_string();
        }
        // `succession flow ...` : un flux implicitement séquencé — reste
        // classé comme un `flow` (dernier mot), voir `ast::classify`.
        if kw == "succession" && self.at_kw("flow") {
            self.bump();
            kw = "succession flow".to_string();
        }

        let mut is_def = false;
        if self.at_kw("def") {
            self.bump();
            kw.push_str(" def");
            is_def = true;
        }

        // Enveloppes : `require constraint`, `perform action`, `include use case`, ...
        // `assert` fait exception à l'exclusion des mots-clés de relation :
        // `assert satisfy X;` est la forme normative de SysML v2 pour
        // `[assert] [not] satisfy <requirement> [by <subject>];`.
        if !is_def && is_wrapper_kw(&kw) {
            let nx = self.cur().clone();
            if nx.kind == TokKind::Ident
                && !nx.quoted
                && is_member_kw(&nx.text)
                && (!is_relationship_kw(&nx.text, false) || kw == "assert")
                && nx.text != "def"
            {
                self.bump();
                kw.push(' ');
                kw.push_str(&nx.text);
                if (nx.text == "use" || nx.text == "analysis" || nx.text == "verification")
                    && self.at_kw("case")
                {
                    self.bump();
                    kw.push_str(" case");
                }
                if self.at_kw("def") {
                    self.bump();
                    kw.push_str(" def");
                    is_def = true;
                }
            }
        }

        // `assert not satisfy ...` : la négation peut aussi suivre l'enveloppe
        // `assert` plutôt que la précéder (cf. le premier bloc `not` plus haut).
        if self.at_kw("not") {
            let after = self.peek(1).clone();
            if after.kind == TokKind::Ident
                && !after.quoted
                && is_relationship_kw(&after.text, false)
            {
                let t = self.bump();
                node.prefixes.push(Spanned::new(t.text.clone(), t.span));
                let rel = self.bump();
                kw.push(' ');
                kw.push_str(&rel.text);
            }
        }

        node.keyword = kw.clone();
        node.keyword_span = kw_span;
        node.is_def = is_def;
        node.kind = classify(&kw, is_def);

        match kw.as_str() {
            // `verify X;` : X *référence* une exigence existante, ce n'est pas
            // la déclaration d'un nouvel élément nommé X. Le métamodèle est
            // explicite — `VerificationCase*::verifiedRequirement` pointe vers
            // une `RequirementUsage` — et la cible peut donc être qualifiée
            // (`Pkg::Req`) ou chaînée (`UseCase.objectif`), ce que
            // `parse_decl` ne sait pas lire. La forme enveloppée
            // (`verify requirement r : R { ... }`) a déjà été absorbée plus
            // haut par `is_wrapper_kw` et ne passe pas ici.
            "verify" => self.parse_verify_tail(&mut node, depth),
            "import" => self.parse_import_tail(&mut node),
            "alias" => self.parse_alias_tail(&mut node),
            "doc" => self.parse_doc_tail(&mut node),
            "comment" => self.parse_comment_tail(&mut node),
            "rep" => self.parse_rep_tail(&mut node),
            _ => {
                if is_relationship_kw(&kw, is_def) {
                    self.parse_relationship_tail(&mut node, depth);
                } else if !is_def && is_blob_kw(&kw) {
                    let base = if kw == "expose" {
                        RefCtx::ExposeTarget
                    } else {
                        RefCtx::Other
                    };
                    self.parse_blob(&mut node, base);
                    self.finish_member(&mut node, depth);
                } else {
                    self.parse_decl(&mut node, is_def);
                    self.finish_member(&mut node, depth);
                }
            }
        }

        node.span = Span::join(start, self.prev_span());
        Some(node)
    }

    // ---- déclarations ----------------------------------------------------

    fn parse_decl(&mut self, node: &mut Node, is_def: bool) {
        // Nom court `<S1>`
        if self.at_punct("<") {
            self.bump();
            if self.cur().kind == TokKind::Ident {
                let t = self.bump();
                node.short_name = Some(Spanned::new(t.text.clone(), t.span));
            } else {
                self.err_expected("un nom court", "<");
            }
            if !self.eat_punct(">") {
                self.err_expected("`>`", "nom court");
            }
        }

        // Nom
        let ct = self.cur().clone();
        if ct.kind == TokKind::Ident && (ct.quoted || !is_decl_stop_kw(&ct.text)) {
            let is_kw = !ct.quoted && is_reserved_name(&ct.text);
            let t = self.bump();
            node.name = Some(Spanned::new(t.text.clone(), t.span));
            node.name_quoted = t.quoted;
            if is_kw {
                self.push(
                    Diagnostic::error(
                        "E225",
                        "reserved-word-as-name",
                        t.span,
                        format!(
                            "`{}` est un mot-clé SysML v2 et ne peut pas servir de nom",
                            t.text
                        ),
                    )
                    .hint(format!(
                        "renomme l'élément, ou échappe-le avec des apostrophes : `'{}'`",
                        t.text
                    )),
                );
            }
        }

        // Relations, multiplicité, modificateurs de collection
        loop {
            if self.parse_rel_into(node, is_def) {
                continue;
            }
            if node.mult.is_none() && self.at_punct("[") {
                if let Some(m) = self.parse_mult() {
                    node.mult = Some(m);
                    continue;
                }
            }
            if self.at_kw("ordered") || self.at_kw("nonunique") {
                let t = self.bump();
                node.prefixes.push(Spanned::new(t.text.clone(), t.span));
                continue;
            }
            break;
        }

        // Valeur
        if self.at_punct("=") || self.at_punct(":=") || self.at_kw("default") {
            let was_default = self.at_kw("default");
            self.bump();
            if was_default {
                // forme `default = expr`
                self.eat_punct("=");
            }
            let v = self.parse_blob(node, RefCtx::Value);
            node.value = Some(v);
        }
    }

    fn parse_rel_into(&mut self, node: &mut Node, is_def: bool) -> bool {
        let (kind, token, consume_extra) = if self.at_punct(":>>") {
            (RelKind::Redefines, ":>>", 0)
        } else if self.at_punct("::>") {
            (RelKind::References, "::>", 0)
        } else if self.at_punct(":>") {
            (
                if is_def {
                    RelKind::Specializes
                } else {
                    RelKind::Subsets
                },
                ":>",
                0,
            )
        } else if self.at_punct(":") {
            (RelKind::TypedBy, ":", 0)
        } else if self.at_kw("specializes") {
            (RelKind::Specializes, "specializes", 0)
        } else if self.at_kw("subsets") {
            (RelKind::Subsets, "subsets", 0)
        } else if self.at_kw("redefines") {
            (RelKind::Redefines, "redefines", 0)
        } else if self.at_kw("references") {
            (RelKind::References, "references", 0)
        } else if self.at_kw("conjugates") {
            (RelKind::Conjugates, "conjugates", 0)
        } else if self.at_punct("=>") {
            (RelKind::Crosses, "=>", 0)
        } else if self.at_kw("crosses") {
            (RelKind::Crosses, "crosses", 0)
        } else if self.at_kw("defined") && self.peek(1).is_ident("by") {
            (RelKind::TypedBy, "defined by", 1)
        } else {
            return false;
        };

        let op = self.bump();
        for _ in 0..consume_extra {
            self.bump();
        }

        loop {
            let conj = self.eat_punct("~");
            match self.parse_qname(false) {
                Some(q) => {
                    node.rels.push(Rel {
                        kind: if conj { RelKind::Conjugates } else { kind },
                        token: token.to_string(),
                        op_span: op.span,
                        target: q,
                    });
                }
                None => {
                    self.err_expected("un nom qualifié", token);
                    break;
                }
            }
            if self.eat_punct(",") {
                continue;
            }
            break;
        }
        true
    }

    fn parse_mult(&mut self) -> Option<Mult> {
        if !self.at_punct("[") {
            return None;
        }
        let open = self.bump();
        let mut depth: i32 = 0;
        let mut buf: Vec<Token> = Vec::new();
        let mut lower: Option<Spanned> = None;
        let mut has_range = false;
        let mut close = open.span;

        loop {
            if self.is_eof() {
                self.push(Diagnostic::error(
                    "E102",
                    "unclosed-multiplicity",
                    open.span,
                    "`[` non fermé".to_string(),
                ));
                break;
            }
            let t = self.cur().clone();
            if t.kind == TokKind::Punct {
                if t.text == "[" {
                    depth += 1;
                } else if t.text == "]" {
                    if depth == 0 {
                        close = t.span;
                        self.bump();
                        break;
                    }
                    depth -= 1;
                } else if t.text == ".." && depth == 0 {
                    has_range = true;
                    lower = Some(join_tokens(&buf, open.span));
                    buf.clear();
                    self.bump();
                    continue;
                } else if (t.text == "}" || t.text == ";") && depth == 0 {
                    self.push(Diagnostic::error(
                        "E102",
                        "unclosed-multiplicity",
                        open.span,
                        "`[` non fermé".to_string(),
                    ));
                    break;
                }
            }
            buf.push(t);
            self.bump();
        }

        let upper = join_tokens(&buf, close);
        Some(Mult {
            lower: if has_range { lower } else { None },
            upper,
            span: Span::join(open.span, close),
        })
    }

    // ---- fins de membre --------------------------------------------------

    fn finish_member(&mut self, node: &mut Node, depth: u32) {
        // Queue éventuelle : `connection c : C connect a to b;`
        let ct = self.cur().clone();
        if ct.kind == TokKind::Ident && !ct.quoted && is_tail_kw(&ct.text) {
            let base = base_ctx_for(&ct.text);
            let base = if base == RefCtx::Other {
                RefCtx::Other
            } else {
                base
            };
            self.parse_blob(node, base);
        }

        // `state def Modes parallel { ... }` — régions orthogonales.
        if self.at_kw("parallel") {
            let t = self.bump();
            node.prefixes.push(Spanned::new(t.text.clone(), t.span));
        }

        if self.at_punct("{") {
            let expr_ok = kw_allows_expression(&node.keyword);
            self.parse_body(node, expr_ok, depth);
            self.eat_punct(";");
            return;
        }

        if self.at_punct(";") {
            self.bump();
            return;
        }

        if self.is_eof() || self.at_punct("}") {
            let span = self.prev_span();
            self.push(
                Diagnostic::error(
                    "E103",
                    "missing-semicolon",
                    span,
                    "`;` manquant en fin de déclaration".to_string(),
                )
                .hint("termine la déclaration par `;` ou ouvre un corps `{ ... }`".to_string()),
            );
            return;
        }

        if self.starts_member() {
            let span = self.prev_span();
            self.push(
                Diagnostic::error(
                    "E103",
                    "missing-semicolon",
                    span,
                    "`;` manquant avant le membre suivant".to_string(),
                )
                .hint(
                    "chaque déclaration se termine par `;` ou par un corps `{ ... }`".to_string(),
                ),
            );
            return;
        }

        let span = self.cur().span;
        let found = self.describe_cur();
        self.push(
            Diagnostic::error(
                "E100",
                "unexpected-token",
                span,
                format!("{found} inattendu dans cette déclaration"),
            )
            .hint(
                "attendu ici : `:` (typage), `:>` (spécialisation), `[..]` (multiplicité), \
                 `=` (valeur), `{ ... }` (corps) ou `;`"
                    .to_string(),
            ),
        );
        self.recover();
    }

    fn parse_body(&mut self, node: &mut Node, expr_ok: bool, depth: u32) {
        let open = self.bump(); // `{`
        node.has_body = true;
        // Dans un `enum def`, les valeurs énumérées peuvent s'écrire sans mot-clé.
        let enum_ok = node.keyword.contains("enum");
        loop {
            if self.is_eof() {
                self.push(
                    Diagnostic::error(
                        "E102",
                        "unclosed-brace",
                        open.span,
                        "`{` non fermé : il manque `}`".to_string(),
                    )
                    .hint("vérifie l'équilibre des accolades du modèle".to_string()),
                );
                break;
            }
            if self.at_punct("}") {
                self.bump();
                break;
            }
            if self.eat_punct(";") {
                continue;
            }

            let before = self.pos;
            if self.starts_member() {
                if let Some(child) = self.parse_member(depth + 1) {
                    node.children.push(child);
                } else {
                    self.recover();
                }
            } else if enum_ok && self.cur().kind == TokKind::Ident {
                let kwspan = self.cur().span;
                let mut child = Node::new(kwspan);
                child.kind = NodeKind::Usage;
                child.keyword = "enum".to_string();
                child.keyword_span = kwspan;
                self.parse_decl(&mut child, false);
                self.finish_member(&mut child, depth + 1);
                child.span = Span::join(kwspan, self.prev_span());
                node.children.push(child);
            } else if expr_ok {
                self.parse_blob(node, RefCtx::Value);
            } else {
                let span = self.cur().span;
                let found = self.describe_cur();
                self.push(
                    Diagnostic::error(
                        "E100",
                        "member-must-start-with-keyword",
                        span,
                        format!("membre inattendu : {found}"),
                    )
                    .hint(
                        "il manque probablement un mot-clé : écris par exemple \
                         `attribute masse : ISQ::MassValue;` plutôt que `masse : ISQ::MassValue;`"
                            .to_string(),
                    ),
                );
                self.recover();
            }
            if self.pos == before {
                self.bump();
            }
        }
    }

    // ---- relations -------------------------------------------------------

    fn parse_verify_tail(&mut self, node: &mut Node, depth: u32) {
        loop {
            match self.parse_qname(false) {
                Some(q) => node.refs.push(RefUse {
                    qname: q,
                    ctx: RefCtx::VerifyTarget,
                }),
                None => {
                    self.err_expected("une exigence à vérifier", "verify");
                    break;
                }
            }
            if self.eat_punct(",") {
                continue;
            }
            break;
        }
        self.finish_member(node, depth);
    }

    fn parse_relationship_tail(&mut self, node: &mut Node, depth: u32) {
        let head = node.keyword.split(' ').next().unwrap_or("").to_string();

        // Nom optionnel : `transition t1 first S1 then S2;`
        if matches!(
            head.as_str(),
            "transition" | "succession" | "flow" | "allocate" | "dependency" | "connect"
        ) {
            let ct = self.cur().clone();
            if ct.kind == TokKind::Ident && (ct.quoted || !is_member_kw(&ct.text)) {
                let nx = self.peek(1).clone();
                let named = (nx.kind == TokKind::Ident
                    && !nx.quoted
                    && matches!(nx.text.as_str(), "first" | "from" | "of"))
                    || nx.is_punct(":");
                if named {
                    let t = self.bump();
                    node.name = Some(Spanned::new(t.text.clone(), t.span));
                    node.name_quoted = t.quoted;
                    self.parse_rel_into(node, false);
                }
            }
        }

        let base = base_ctx_for(&node.keyword);

        // Forme n-aire : `connect (a, b, c);`
        if self.at_punct("(") {
            self.bump();
            loop {
                if self.is_eof() || self.at_punct(")") {
                    break;
                }
                if self.eat_punct(",") {
                    continue;
                }
                let before = self.pos;
                match self.parse_qname(false) {
                    Some(q) => node.refs.push(RefUse {
                        qname: q,
                        ctx: base,
                    }),
                    None => {
                        self.bump();
                    }
                }
                if self.pos == before {
                    self.bump();
                }
            }
            self.eat_punct(")");
        } else {
            self.parse_blob(node, base);
        }

        self.finish_member(node, depth);
    }

    /// Consomme une expression / une queue de relation jusqu'à `;`, `{`, `}` ou EOF,
    /// en collectant au passage tous les noms référencés.
    fn parse_blob(&mut self, node: &mut Node, base: RefCtx) -> Spanned {
        let start = self.cur().span;
        let mut text = String::new();
        let mut span = start;
        let mut depth: i32 = 0;
        let mut ctx = base;

        loop {
            let t = self.cur().clone();
            if t.kind == TokKind::Eof {
                break;
            }
            if t.kind == TokKind::Punct {
                match t.text.as_str() {
                    "(" | "[" => depth += 1,
                    ")" | "]" => {
                        if depth == 0 {
                            break;
                        }
                        depth -= 1;
                    }
                    ";" | "{" | "}" if depth == 0 => break,
                    _ => {}
                }
            }

            if t.kind == TokKind::Ident {
                if !t.quoted && (is_expr_kw(&t.text) || is_member_kw(&t.text)) {
                    ctx = ctx_for_word(&t.text, ctx);
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(&t.text);
                    span = Span::join(span, t.span);
                    self.bump();
                    continue;
                }
                if let Some(q) = self.parse_qname(false) {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(&q.text());
                    span = Span::join(span, q.span);
                    node.refs.push(RefUse { qname: q, ctx });
                    continue;
                }
            }

            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(&t.text);
            span = Span::join(span, t.span);
            self.bump();
        }

        Spanned::new(text, span)
    }

    // ---- membres particuliers -------------------------------------------

    fn parse_import_tail(&mut self, node: &mut Node) {
        self.eat_kw("all");
        match self.parse_qname(true) {
            Some(q) => {
                node.refs.push(RefUse {
                    qname: q.clone(),
                    ctx: RefCtx::ImportTarget,
                });
                node.import_target = Some(q);
            }
            None => {
                self.err_expected("un nom qualifié", "import");
            }
        }
        while self.at_punct("[") {
            let _ = self.parse_mult();
        }
        if self.at_punct("{") {
            let mut tmp = Node::new(self.cur().span);
            self.parse_body(&mut tmp, true, 1);
            return;
        }
        if !self.eat_punct(";") {
            let span = self.prev_span();
            self.push(
                Diagnostic::error(
                    "E103",
                    "missing-semicolon",
                    span,
                    "`;` manquant après l'import".to_string(),
                )
                .hint("écris par exemple `import ISQ::*;`".to_string()),
            );
        }
    }

    fn parse_alias_tail(&mut self, node: &mut Node) {
        if self.at_punct("<") {
            self.bump();
            if self.cur().kind == TokKind::Ident {
                let t = self.bump();
                node.short_name = Some(Spanned::new(t.text.clone(), t.span));
            }
            self.eat_punct(">");
        }
        let ct = self.cur().clone();
        if ct.kind == TokKind::Ident && !ct.is_ident("for") {
            let t = self.bump();
            node.name = Some(Spanned::new(t.text.clone(), t.span));
            node.name_quoted = t.quoted;
        }
        if !self.eat_kw("for") {
            self.err_expected("`for`", "alias");
        }
        match self.parse_qname(false) {
            Some(q) => node.refs.push(RefUse {
                qname: q,
                ctx: RefCtx::AliasTarget,
            }),
            None => self.err_expected("un nom qualifié", "for"),
        }
        self.eat_punct(";");
    }

    fn parse_doc_tail(&mut self, node: &mut Node) {
        let ct = self.cur().clone();
        if ct.kind == TokKind::Ident
            && !ct.is_ident("locale")
            && (ct.quoted || !is_member_kw(&ct.text))
        {
            let t = self.bump();
            node.name = Some(Spanned::new(t.text.clone(), t.span));
            node.name_quoted = t.quoted;
        }
        if self.eat_kw("locale") {
            if self.cur().kind == TokKind::Str {
                let t = self.bump();
                node.value = Some(Spanned::new(t.text.clone(), t.span));
            } else {
                self.err_expected("une locale entre guillemets", "locale");
            }
        }
        match self.take_doc_body() {
            Some(s) => node.doc = Some(s.text),
            None => {
                let span = node.keyword_span;
                self.push(
                    Diagnostic::error(
                        "E104",
                        "doc-without-body",
                        span,
                        "`doc` sans corps de documentation".to_string(),
                    )
                    .hint("la documentation s'écrit `doc /* mon texte */`".to_string()),
                );
            }
        }
        self.eat_punct(";");
    }

    /// `rep [Nom] language "lang" /* texte */;` — représentation textuelle
    /// alternative (`TextualRepresentation`), même mécanique que `doc`.
    fn parse_rep_tail(&mut self, node: &mut Node) {
        let ct = self.cur().clone();
        if ct.kind == TokKind::Ident
            && !ct.is_ident("language")
            && (ct.quoted || !is_member_kw(&ct.text))
        {
            let t = self.bump();
            node.name = Some(Spanned::new(t.text.clone(), t.span));
            node.name_quoted = t.quoted;
        }
        if !self.eat_kw("language") {
            self.err_expected("`language`", "rep");
        } else if self.cur().kind == TokKind::Str {
            let t = self.bump();
            node.value = Some(Spanned::new(t.text.clone(), t.span));
        } else {
            self.err_expected("une chaîne de langue", "language");
        }
        match self.take_doc_body() {
            Some(s) => node.doc = Some(s.text),
            None => {
                let span = node.keyword_span;
                self.push(
                    Diagnostic::error(
                        "E104",
                        "rep-without-body",
                        span,
                        "`rep` sans corps de représentation".to_string(),
                    )
                    .hint(
                        "une représentation s'écrit `rep language \"OCL\" /* ... */`".to_string(),
                    ),
                );
            }
        }
        self.eat_punct(";");
    }

    fn parse_comment_tail(&mut self, node: &mut Node) {
        let ct = self.cur().clone();
        if ct.kind == TokKind::Ident
            && !ct.is_ident("about")
            && !ct.is_ident("locale")
            && (ct.quoted || !is_member_kw(&ct.text))
        {
            let t = self.bump();
            node.name = Some(Spanned::new(t.text.clone(), t.span));
            node.name_quoted = t.quoted;
        }
        if self.eat_kw("about") {
            loop {
                match self.parse_qname(false) {
                    Some(q) => node.refs.push(RefUse {
                        qname: q,
                        ctx: RefCtx::About,
                    }),
                    None => {
                        self.err_expected("un nom qualifié", "about");
                        break;
                    }
                }
                if self.eat_punct(",") {
                    continue;
                }
                break;
            }
        }
        if self.eat_kw("locale") {
            if self.cur().kind == TokKind::Str {
                let t = self.bump();
                node.value = Some(Spanned::new(t.text.clone(), t.span));
            } else {
                self.err_expected("une locale entre guillemets", "locale");
            }
        }
        match self.take_doc_body() {
            Some(s) => node.doc = Some(s.text),
            None => {
                let span = node.keyword_span;
                self.push(
                    Diagnostic::error(
                        "E104",
                        "comment-without-body",
                        span,
                        "`comment` sans corps".to_string(),
                    )
                    .hint("un commentaire s'écrit `comment /* mon texte */`".to_string()),
                );
            }
        }
        self.eat_punct(";");
    }

    // ---- noms qualifiés --------------------------------------------------

    fn parse_qname(&mut self, allow_wildcard: bool) -> Option<QName> {
        if self.cur().kind != TokKind::Ident {
            return None;
        }
        let first = self.bump();
        let mut span = first.span;
        let mut parts = vec![Spanned::new(first.text.clone(), first.span)];
        let mut seps: Vec<char> = Vec::new();
        let mut wildcard: Option<String> = None;

        loop {
            let is_cc = self.at_punct("::");
            let is_dot = self.at_punct(".");
            if !is_cc && !is_dot {
                break;
            }
            let nx = self.peek(1).clone();

            if is_cc && allow_wildcard && nx.is_punct("*") {
                self.bump();
                let star = self.bump();
                span = Span::join(span, star.span);
                let mut w = String::from("*");
                if self.at_punct("::") && self.peek(1).is_punct("**") {
                    self.bump();
                    let d = self.bump();
                    span = Span::join(span, d.span);
                    w.push_str("::**");
                }
                wildcard = Some(w);
                break;
            }

            // Import membre récursif sans joker : `A::B::**` (distinct de
            // `A::*::**` ci-dessus, qui passe par un `*` explicite).
            if is_cc && allow_wildcard && nx.is_punct("**") {
                self.bump();
                let d = self.bump();
                span = Span::join(span, d.span);
                wildcard = Some("**".to_string());
                break;
            }

            if nx.kind != TokKind::Ident {
                break;
            }
            self.bump();
            let t = self.bump();
            span = Span::join(span, t.span);
            seps.push(if is_dot { '.' } else { ':' });
            parts.push(Spanned::new(t.text.clone(), t.span));
        }

        Some(QName {
            parts,
            seps,
            wildcard,
            span,
        })
    }
}

fn join_tokens(toks: &[Token], fallback: Span) -> Spanned {
    if toks.is_empty() {
        return Spanned::new(String::new(), fallback);
    }
    let mut s = String::new();
    let mut span = toks[0].span;
    for t in toks {
        s.push_str(&t.text);
        span = Span::join(span, t.span);
    }
    Spanned::new(s, span)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diag::Diagnostic;
    use crate::lexer::Lexer;

    fn parse(src: &str) -> (Vec<Node>, Vec<Diagnostic>) {
        let (toks, mut diags) = Lexer::new(src, 0).tokenize();
        let mut p = Parser::new(toks, 500);
        let nodes = p.parse_unit();
        diags.extend(p.diags.clone());
        (nodes, diags)
    }

    fn codes(d: &[Diagnostic]) -> Vec<&'static str> {
        d.iter().map(|x| x.code).collect()
    }

    #[test]
    fn package_with_import_and_part_def_parses_cleanly() {
        let (nodes, diags) = parse("package P { import ISQ::*; part def Robot; }");
        assert!(diags.is_empty(), "{:?}", codes(&diags));
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].kind, NodeKind::Package);
        assert_eq!(nodes[0].children.len(), 2);
        assert_eq!(nodes[0].children[1].kind, NodeKind::Def);
        assert_eq!(nodes[0].children[1].name.as_ref().unwrap().text, "Robot");
    }

    #[test]
    fn member_must_start_with_keyword_reports_e100() {
        let (_n, d) = parse("package P { x : Integer; }");
        assert!(codes(&d).contains(&"E100"));
    }

    #[test]
    fn missing_semicolon_before_next_member_reports_e103() {
        let (_n, d) = parse("package P { part def Robot part def Wheel; }");
        assert!(codes(&d).contains(&"E103"));
    }

    #[test]
    fn unclosed_brace_reports_e102() {
        let (_n, d) = parse("package P { part def Robot {");
        assert!(codes(&d).contains(&"E102"));
    }

    #[test]
    fn doc_without_body_reports_e104() {
        let (_n, d) = parse("package P { doc; }");
        assert!(codes(&d).contains(&"E104"));
    }

    #[test]
    fn comment_without_body_reports_e104() {
        let (_n, d) = parse("package P { comment; }");
        assert!(codes(&d).contains(&"E104"));
    }

    #[test]
    fn reserved_word_as_unquoted_name_reports_e225() {
        let (_n, d) = parse("package P { part def part; }");
        assert!(codes(&d).contains(&"E225"));
    }

    #[test]
    fn quoted_reserved_word_as_name_is_accepted_without_e225() {
        let (_n, d) = parse("package P { part def Robot { part 'part' : Robot; } }");
        assert!(!codes(&d).contains(&"E225"), "{:?}", codes(&d));
    }

    #[test]
    fn legacy_keywords_still_parse_so_they_can_be_diagnosed_by_rules() {
        let (nodes, d) = parse("package P { block Robot; }");
        assert!(d.is_empty(), "le parseur accepte `block` : {:?}", codes(&d));
        assert_eq!(nodes[0].children[0].keyword, "block");
    }

    #[test]
    fn single_star_wildcard_import_parses() {
        let (nodes, d) = parse("import A::*;");
        assert!(d.is_empty(), "{:?}", codes(&d));
        assert_eq!(
            nodes[0].import_target.as_ref().unwrap().wildcard.as_deref(),
            Some("*")
        );
    }

    #[test]
    fn deep_wildcard_import_parses() {
        let (nodes, d) = parse("import A::*::**;");
        assert!(d.is_empty(), "{:?}", codes(&d));
        assert_eq!(
            nodes[0].import_target.as_ref().unwrap().wildcard.as_deref(),
            Some("*::**")
        );
    }

    #[test]
    fn qualified_name_mixes_colon_colon_and_dot_separators() {
        let (nodes, d) = parse("package P { part def Robot { attribute x = a::b.c; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let attr = &nodes[0].children[0].children[0];
        assert_eq!(attr.refs[0].qname.text(), "a::b.c");
    }

    #[test]
    fn multiplicity_range_is_parsed_with_lower_and_upper() {
        let (nodes, d) = parse("package P { part def S; part def R { part s : S[1..4]; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let mult = nodes[0].children[1].children[0].mult.as_ref().unwrap();
        assert_eq!(mult.lower.as_ref().unwrap().text, "1");
        assert_eq!(mult.upper.text, "4");
    }

    #[test]
    fn multiplicity_star_upper_has_no_lower_bound() {
        let (nodes, d) = parse("package P { part def S; part def R { part s : S[*]; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let mult = nodes[0].children[1].children[0].mult.as_ref().unwrap();
        assert!(mult.lower.is_none());
        assert_eq!(mult.upper.text, "*");
    }

    #[test]
    fn conjugated_port_type_is_recorded_as_conjugates() {
        let (nodes, d) = parse("package P { port def FP; part def V { port p : ~FP; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let port = &nodes[0].children[1].children[0];
        assert_eq!(port.rels[0].kind, RelKind::Conjugates);
    }

    #[test]
    fn definition_specializes_with_arrow_colon_greater() {
        let (nodes, d) = parse("package P { part def A; part def B :> A; }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        assert_eq!(nodes[0].children[1].rels[0].kind, RelKind::Specializes);
    }

    #[test]
    fn usage_subsets_with_arrow_colon_greater() {
        let (nodes, d) = parse("package P { part def A; part def R { part a : A; part b :> a; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        assert_eq!(
            nodes[0].children[1].children[1].rels[0].kind,
            RelKind::Subsets
        );
    }

    #[test]
    fn redefines_operator_is_recognized() {
        let (nodes, d) = parse("package P { part def A { attribute x : Integer; } part def B :> A { attribute y :>> x; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let y = &nodes[0].children[1].children[0];
        assert_eq!(y.rels[0].kind, RelKind::Redefines);
    }

    #[test]
    fn nesting_deeper_than_the_limit_reports_e105() {
        let mut src = String::from("package P { ");
        for i in 0..70 {
            src.push_str(&format!("part def A{i} {{ "));
        }
        for _ in 0..70 {
            src.push_str("} ");
        }
        src.push('}');
        let (_n, d) = parse(&src);
        assert!(codes(&d).contains(&"E105"), "{:?}", codes(&d));
    }

    #[test]
    fn nesting_within_the_limit_does_not_report_e105() {
        let mut src = String::from("package P { ");
        for i in 0..10 {
            src.push_str(&format!("part def A{i} {{ "));
        }
        for _ in 0..10 {
            src.push_str("} ");
        }
        src.push('}');
        let (_n, d) = parse(&src);
        assert!(!codes(&d).contains(&"E105"), "{:?}", codes(&d));
    }

    #[test]
    fn end_member_directly_followed_by_a_name_keeps_end_as_the_keyword() {
        // Forme la plus courante : `end nom : Type;` — `end` devient le mot-clé
        // du membre (pas un préfixe), voir `rules::is_end_member`.
        let (nodes, d) = parse("package P { connection def C { end source : X; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let end_member = &nodes[0].children[0].children[0];
        assert_eq!(end_member.keyword, "end");
        assert_eq!(end_member.name.as_ref().unwrap().text, "source");
    }

    #[test]
    fn crosses_arrow_operator_is_recognized_as_crosses_rel() {
        let (nodes, d) =
            parse("package P { part def A; part def Link { part x : A; attribute cross => x; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let cross = &nodes[0].children[1].children[1];
        assert_eq!(cross.rels[0].kind, RelKind::Crosses);
        assert_eq!(cross.rels[0].token, "=>");
    }

    #[test]
    fn crosses_keyword_form_is_recognized_as_crosses_rel() {
        let (nodes, d) = parse(
            "package P { part def A; part def Link { part x : A; attribute cross crosses x; } }",
        );
        assert!(d.is_empty(), "{:?}", codes(&d));
        let cross = &nodes[0].children[1].children[1];
        assert_eq!(cross.rels[0].kind, RelKind::Crosses);
        assert_eq!(cross.rels[0].token, "crosses");
    }

    #[test]
    fn bare_analysis_def_without_case_suffix_parses() {
        let (nodes, d) = parse("package P { analysis def Study; }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        assert_eq!(nodes[0].children[0].keyword, "analysis def");
        assert_eq!(nodes[0].children[0].kind, NodeKind::Def);
    }

    #[test]
    fn bare_verification_def_without_case_suffix_parses() {
        let (nodes, d) = parse("package P { verification def Check; }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        assert_eq!(nodes[0].children[0].keyword, "verification def");
        assert_eq!(nodes[0].children[0].kind, NodeKind::Def);
    }

    #[test]
    fn assert_satisfy_is_a_single_relationship_node() {
        let (nodes, d) = parse(
            "package P { requirement def Req; part def Robot; part r : Robot { assert satisfy Req by r; } }",
        );
        assert!(d.is_empty(), "{:?}", codes(&d));
        let sat = &nodes[0].children[2].children[0];
        assert_eq!(sat.kind, NodeKind::Relationship);
        assert_eq!(sat.keyword, "assert satisfy");
        assert_eq!(sat.refs[0].ctx, RefCtx::SatisfyTarget);
        assert_eq!(sat.refs[0].qname.text(), "Req");
        assert_eq!(sat.refs[1].ctx, RefCtx::SatisfyBy);
        assert_eq!(sat.refs[1].qname.text(), "r");
    }

    #[test]
    fn not_satisfy_is_a_single_relationship_node() {
        let (nodes, d) = parse(
            "package P { requirement def Req; part def Robot; part r : Robot { not satisfy Req by r; } }",
        );
        assert!(d.is_empty(), "{:?}", codes(&d));
        let sat = &nodes[0].children[2].children[0];
        assert_eq!(sat.kind, NodeKind::Relationship);
        assert_eq!(sat.keyword, "satisfy");
        assert!(sat.has_prefix("not"));
        assert_eq!(sat.refs[0].ctx, RefCtx::SatisfyTarget);
        assert_eq!(sat.refs[0].qname.text(), "Req");
    }

    #[test]
    fn assert_not_satisfy_is_a_single_relationship_node() {
        let (nodes, d) = parse(
            "package P { requirement def Req; part def Robot; part r : Robot { assert not satisfy Req by r; } }",
        );
        assert!(d.is_empty(), "{:?}", codes(&d));
        let sat = &nodes[0].children[2].children[0];
        assert_eq!(sat.kind, NodeKind::Relationship);
        assert_eq!(sat.keyword, "assert satisfy");
        assert!(sat.has_prefix("not"));
        assert_eq!(sat.refs[0].ctx, RefCtx::SatisfyTarget);
        assert_eq!(sat.refs[0].qname.text(), "Req");
    }

    #[test]
    fn constant_prefix_is_accepted_before_a_member_keyword() {
        let (nodes, d) =
            parse("package P { part def Robot { constant attribute pi : Real = 3; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let pi = &nodes[0].children[0].children[0];
        assert!(pi.has_prefix("constant"));
        assert_eq!(pi.keyword, "attribute");
    }

    #[test]
    fn standalone_metadata_usage_at_meta_parses_as_metadata_typed_by_target() {
        let (nodes, d) = parse("package P { metadata def MyMeta; part def Engine { @MyMeta; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let m = &nodes[0].children[1].children[0];
        assert_eq!(m.keyword, "metadata");
        assert_eq!(m.kind, NodeKind::Usage);
        assert_eq!(m.rels[0].kind, RelKind::TypedBy);
        assert_eq!(m.rels[0].target.text(), "MyMeta");
    }

    #[test]
    fn hash_prefix_annotation_still_requires_a_following_member() {
        // `#Meta` reste une simple annotation-préfixe : contrairement à `@`,
        // elle ne forme jamais un membre autonome.
        let (nodes, d) = parse(
            "package P { metadata def MyMeta; part def Engine { #MyMeta part def Sensor; } }",
        );
        assert!(d.is_empty(), "{:?}", codes(&d));
        let sensor = &nodes[0].children[1].children[0];
        assert_eq!(sensor.keyword, "part def");
        assert_eq!(sensor.name.as_ref().unwrap().text, "Sensor");
    }

    #[test]
    fn parallel_state_modifier_is_accepted_before_the_body() {
        let (nodes, d) = parse("package P { state def Modes parallel { state A; state B; } }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let modes = &nodes[0].children[0];
        assert!(modes.has_prefix("parallel"));
        assert_eq!(modes.children.len(), 2);
    }

    #[test]
    fn recursive_member_import_without_wildcard_parses() {
        let (nodes, d) = parse("import ISQ::MassValue::**;");
        assert!(d.is_empty(), "{:?}", codes(&d));
        assert_eq!(
            nodes[0].import_target.as_ref().unwrap().wildcard.as_deref(),
            Some("**")
        );
        assert_eq!(
            nodes[0].import_target.as_ref().unwrap().text(),
            "ISQ::MassValue::**"
        );
    }

    #[test]
    fn rep_textual_representation_parses_with_language_and_body() {
        let (nodes, d) = parse(
            r#"package P { part def Robot { rep myRep language "OCL" /* self.mass > 0 */; } }"#,
        );
        assert!(d.is_empty(), "{:?}", codes(&d));
        let rep = &nodes[0].children[0].children[0];
        assert_eq!(rep.keyword, "rep");
        assert_eq!(rep.kind, NodeKind::Doc);
        assert_eq!(rep.name.as_ref().unwrap().text, "myRep");
        assert_eq!(rep.value.as_ref().unwrap().text, "OCL");
        assert_eq!(rep.doc.as_deref(), Some(" self.mass > 0 "));
    }

    #[test]
    fn rep_without_language_reports_expected_token() {
        let (_n, d) = parse(r#"package P { part def Robot { rep myRep /* body */; } }"#);
        assert!(codes(&d).contains(&"E101"), "{:?}", codes(&d));
    }

    #[test]
    fn doc_with_locale_parses() {
        let (nodes, d) = parse(r#"package P { doc D locale "en" /* hello */; }"#);
        assert!(d.is_empty(), "{:?}", codes(&d));
        let doc = &nodes[0].children[0];
        assert_eq!(doc.keyword, "doc");
        assert_eq!(doc.name.as_ref().unwrap().text, "D");
        assert_eq!(doc.value.as_ref().unwrap().text, "en");
        assert_eq!(doc.doc.as_deref(), Some(" hello "));
    }

    #[test]
    fn doc_without_a_name_but_with_locale_parses() {
        let (nodes, d) = parse(r#"package P { doc locale "en" /* hello */; }"#);
        assert!(d.is_empty(), "{:?}", codes(&d));
        let doc = &nodes[0].children[0];
        assert!(doc.name.is_none());
        assert_eq!(doc.value.as_ref().unwrap().text, "en");
    }

    #[test]
    fn comment_with_about_and_locale_parses() {
        let (nodes, d) = parse(
            r#"package P { part def Robot; comment C about Robot locale "fr" /* bonjour */; } "#,
        );
        assert!(d.is_empty(), "{:?}", codes(&d));
        let c = &nodes[0].children[1];
        assert_eq!(c.refs[0].ctx, RefCtx::About);
        assert_eq!(c.value.as_ref().unwrap().text, "fr");
    }

    #[test]
    fn succession_flow_is_recognized_as_a_single_flow_keyword() {
        let (nodes, d) = parse(
            "package P { part def A; part def Link { port pa : A; port pb : A; succession flow pa to pb; } }",
        );
        assert!(d.is_empty(), "{:?}", codes(&d));
        let sf = &nodes[0].children[1].children[2];
        assert_eq!(sf.keyword, "succession flow");
        assert_eq!(sf.kind, NodeKind::Relationship);
    }

    #[test]
    fn expose_target_is_tagged_with_expose_context() {
        let (nodes, d) = parse("package Q { part def X; } package P { expose Q::X; }");
        assert!(d.is_empty(), "{:?}", codes(&d));
        let expose = &nodes[1].children[0];
        assert_eq!(expose.refs[0].ctx, RefCtx::ExposeTarget);
    }
}
