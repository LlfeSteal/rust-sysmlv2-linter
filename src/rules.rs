//! Résolution de noms et règles sémantiques.
//!
//! Chaque diagnostic porte un code stable (`E210`), un nom de règle neutre
//! linguistiquement (`def-typed-by-colon`) et, autant que possible, une *aide*
//! contenant la correction à appliquer telle quelle.

use std::collections::HashMap;
use std::collections::HashSet;

use crate::ast::{NodeKind, QName, RefCtx, RelKind};
use crate::diag::{Diagnostic, Span};
use crate::model::Model;
use crate::parser::is_legacy_kw;
use crate::stdlib;

/// Catalogue exposé par `--list-rules` : (code, règle, description).
pub const CATALOG: &[(&str, &str, &str)] = &[
    ("E001", "unterminated-block-comment", "Commentaire `/* ... */` non fermé."),
    ("E002", "unterminated-string", "Chaîne ou nom entre apostrophes non terminé."),
    ("E003", "unexpected-character", "Caractère hors grammaire (souvent un stéréotype UML `«...»`)."),
    ("E100", "member-must-start-with-keyword", "Un membre doit commencer par un mot-clé SysML v2."),
    ("E100", "unexpected-token", "Jeton inattendu dans une déclaration."),
    ("E101", "expected-token", "Jeton attendu absent."),
    ("E102", "unclosed-brace", "Accolade, crochet ou multiplicité non fermé."),
    ("E103", "missing-semicolon", "Déclaration non terminée par `;` ni par un corps `{ }`."),
    ("E104", "doc-without-body", "`doc`, `comment` ou `rep` sans corps `/* ... */`."),
    ("E105", "nesting-too-deep", "Imbrication supérieure à 64 niveaux."),
    ("E200", "unresolved-name", "Nom référencé introuvable dans le modèle ni dans la bibliothèque."),
    ("E201", "duplicate-name", "Deux éléments portent le même nom dans une même portée."),
    ("E210", "def-typed-by-colon", "Une définition utilise `:` au lieu de `:>` (spécialisation)."),
    ("E212", "multiplicity-on-definition", "Multiplicité placée sur une définition au lieu d'un usage."),
    ("E213", "legacy-keyword", "Mot-clé SysML v1 / UML (`block`, `value`, `association`, ...)."),
    ("E214", "redefines-target-not-inherited", "`redefines` vise un élément absent des supertypes."),
    ("E215", "end-outside-connection", "`end` hors d'une `connection def` / `interface def`."),
    ("E216", "subject-outside-requirement", "`subject` hors d'une exigence, d'un `concern` ou d'un `case`."),
    ("E218", "invalid-multiplicity-range", "Bornes de multiplicité incohérentes ou vides."),
    ("E222", "variant-outside-variation", "`variant` hors d'un élément `variation`."),
    ("E225", "reserved-word-as-name", "Mot réservé utilisé comme nom sans échappement `'...'`."),
    ("E227", "package-inside-definition", "`package` déclaré dans une définition ou un usage."),
    ("E230", "satisfy-target-not-requirement", "`satisfy` vise un élément qui n'est pas une exigence."),
    ("E231", "actor-outside-requirement-or-case", "`actor` hors d'une exigence ou d'un cas."),
    ("E232", "stakeholder-outside-requirement", "`stakeholder` hors d'une exigence."),
    ("E233", "require-assume-outside-requirement", "`require` ou `assume` hors d'une exigence."),
    ("E234", "objective-outside-case", "`objective` hors d'un cas."),
    ("E235", "frame-outside-requirement", "`frame` hors d'une exigence."),
    ("E236", "verify-outside-requirement", "`verify` hors d'une exigence."),
    ("W200", "unresolved-name", "Nom non résolu dans un contexte tolérant (expression, import opaque)."),
    ("W301", "unimported-standard-type", "Type de la bibliothèque standard utilisé sans import."),
    ("W302", "empty-package", "Paquet déclaré avec un corps vide (--pedantic)."),
    ("W306", "naming-convention", "UpperCamelCase attendu pour les définitions, lowerCamelCase pour les usages (--pedantic)."),
    ("W307", "requirement-without-subject", "`requirement def` sans `subject` (--pedantic)."),
    ("W309", "untyped-usage", "Usage déclaré sans type (--pedantic)."),
    ("W310", "connection-without-ends", "`connection def` sans extrémité `end`."),
    ("W311", "non-standard-keyword", "Mot-clé absent de la grammaire SysML v2 (`readonly`, `composite`, `portion` employés seuls) (--pedantic)."),
    ("W312", "kerml-only-keyword", "Mot-clé KerML absent de la surface SysML v2 (`feature`, `namespace`, `specialization`, `subclassification`) (--pedantic)."),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnresolvedMode {
    Error,
    Warn,
    Off,
}

pub struct Options {
    pub pedantic: bool,
    pub unresolved: UnresolvedMode,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            pedantic: false,
            unresolved: UnresolvedMode::Error,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Res {
    Local(usize),
    /// La racine est un paquet de la bibliothèque standard.
    LibraryRoot,
    /// Nom simple connu de la bibliothèque standard mais non importé explicitement.
    LibraryLeaf,
    /// Impossible de conclure (navigation via un type inconnu, budget épuisé...).
    Opaque,
    /// La partie `usize` du nom qualifié n'a pas été trouvée.
    Unknown(usize),
}

// --------------------------------------------------------------------------
// Résolveur
// --------------------------------------------------------------------------

pub struct Resolver<'a> {
    m: &'a Model,
    budget: u32,
}

impl<'a> Resolver<'a> {
    pub fn new(m: &'a Model) -> Resolver<'a> {
        Resolver { m, budget: 4096 }
    }

    fn tick(&mut self) -> bool {
        if self.budget == 0 {
            false
        } else {
            self.budget -= 1;
            true
        }
    }

    /// Cherche `name` dans `scope` puis dans ses supertypes.
    fn lookup(&mut self, scope: usize, name: &str, visited: &mut Vec<usize>, depth: u32) -> Option<usize> {
        if depth > 8 || !self.tick() || visited.contains(&scope) {
            return None;
        }
        visited.push(scope);
        if let Some(c) = self.m.child_named(scope, name) {
            return Some(c);
        }
        let owner = self.m.syms[scope].parent.unwrap_or(0);
        let rels = self.m.syms[scope].rels.clone();
        for r in rels.iter() {
            if !matches!(
                r.kind,
                RelKind::Specializes | RelKind::Subsets | RelKind::TypedBy | RelKind::Redefines
            ) {
                continue;
            }
            if let Res::Local(t) = self.resolve_with(owner, &r.target, false) {
                if let Some(c) = self.lookup(t, name, visited, depth + 1) {
                    return Some(c);
                }
            }
        }
        None
    }

    fn lookup_via_imports(&mut self, scope: usize, name: &str) -> Option<usize> {
        let imports = self.m.syms[scope].imports.clone();
        let owner = self.m.syms[scope].parent.unwrap_or(0);
        for imp in imports.iter() {
            if !self.tick() {
                return None;
            }
            if imp.wildcard.is_some() {
                if let Res::Local(pkg) = self.resolve_with(owner, imp, false) {
                    let mut v = Vec::new();
                    if let Some(c) = self.lookup(pkg, name, &mut v, 0) {
                        return Some(c);
                    }
                }
            } else if imp.last() == name {
                if let Res::Local(t) = self.resolve_with(owner, imp, false) {
                    return Some(t);
                }
            }
        }
        None
    }

    /// Type effectif d'un élément (via `:`, sinon `:>`).
    fn type_of(&mut self, id: usize) -> Option<usize> {
        if !self.tick() {
            return None;
        }
        let owner = self.m.syms[id].parent.unwrap_or(0);
        let rels = self.m.syms[id].rels.clone();
        for r in rels.iter() {
            if matches!(
                r.kind,
                RelKind::TypedBy | RelKind::Subsets | RelKind::Specializes | RelKind::References
            ) {
                if let Res::Local(t) = self.resolve_with(owner, &r.target, true) {
                    return Some(t);
                }
            }
        }
        None
    }

    pub fn resolve(&mut self, from: usize, q: &QName) -> Res {
        self.budget = 4096;
        self.resolve_with(from, q, true)
    }

    fn resolve_with(&mut self, from: usize, q: &QName, use_imports: bool) -> Res {
        if q.parts.is_empty() {
            return Res::Opaque;
        }
        if !self.tick() {
            return Res::Opaque;
        }

        let first = q.parts[0].text.as_str();

        // Résolution lexicale : de la portée courante vers la racine.
        let mut scope = Some(from);
        let mut found: Option<usize> = None;
        while let Some(s) = scope {
            let mut v = Vec::new();
            if let Some(c) = self.lookup(s, first, &mut v, 0) {
                found = Some(c);
                break;
            }
            if use_imports {
                if let Some(c) = self.lookup_via_imports(s, first) {
                    found = Some(c);
                    break;
                }
            }
            scope = self.m.syms[s].parent;
        }

        let mut cur = match found {
            Some(c) => c,
            None => {
                if stdlib::is_library_root(first) {
                    return Res::LibraryRoot;
                }
                if q.parts.len() == 1 && stdlib::is_library_leaf(first) {
                    return Res::LibraryLeaf;
                }
                return Res::Unknown(0);
            }
        };

        for i in 1..q.parts.len() {
            if !self.tick() {
                return Res::Opaque;
            }
            let sep = q.seps.get(i - 1).copied().unwrap_or(':');
            let name = q.parts[i].text.as_str();
            let scope_id = if sep == '.' {
                match self.type_of(cur) {
                    Some(t) => t,
                    // Type non résolu localement : on ne peut rien affirmer.
                    None => return Res::Opaque,
                }
            } else {
                cur
            };
            let mut v = Vec::new();
            match self.lookup(scope_id, name, &mut v, 0) {
                Some(c) => cur = c,
                None => return Res::Unknown(i),
            }
        }

        Res::Local(cur)
    }
}

/// Résolution utilisée pour l'export de l'AST : renvoie le nom qualifié résolu.
pub fn resolve_for_emit(m: &Model, scope: usize, q: &QName) -> Option<String> {
    let mut r = Resolver::new(m);
    match r.resolve(scope, q) {
        Res::Local(id) => Some(m.display_name(id)),
        _ => None,
    }
}

// --------------------------------------------------------------------------
// Contexte global du fichier
// --------------------------------------------------------------------------

struct Ctx {
    /// Paquets importés (racines des cibles d'import).
    imported_roots: HashSet<String>,
    /// Vrai s'il existe un import joker vers un paquet inconnu du vérificateur.
    opaque_wildcard: bool,
}

fn build_ctx(m: &Model) -> Ctx {
    let mut imported_roots = HashSet::new();
    let mut opaque_wildcard = false;
    for s in m.syms.iter() {
        for imp in s.imports.iter() {
            let root = imp.root().to_string();
            imported_roots.insert(root.clone());
            for p in imp.parts.iter() {
                imported_roots.insert(p.text.clone());
            }
            if imp.wildcard.is_some() && !stdlib::is_library_root(&root) {
                let mut r = Resolver::new(m);
                if !matches!(r.resolve(0, imp), Res::Local(_)) {
                    opaque_wildcard = true;
                }
            }
        }
    }
    Ctx {
        imported_roots,
        opaque_wildcard,
    }
}

// --------------------------------------------------------------------------
// Point d'entrée
// --------------------------------------------------------------------------

pub fn check(m: &Model, opts: &Options) -> Vec<Diagnostic> {
    let mut out: Vec<Diagnostic> = Vec::new();
    let ctx = build_ctx(m);

    check_duplicates(m, &mut out);

    for id in 1..m.syms.len() {
        check_structure(m, id, opts, &mut out);
        check_references(m, id, opts, &ctx, &mut out);
        check_style(m, id, opts, &mut out);
    }

    out
}

// --------------------------------------------------------------------------
// Règles structurelles
// --------------------------------------------------------------------------

fn head_kw(kw: &str) -> &str {
    kw.split(' ').next().unwrap_or("")
}

/// Un membre `end` peut s'écrire `end port p : X;` (`end` en modificateur devant un
/// autre mot-clé) ou, forme la plus courante, `end nomDeLExtremite : X;` — auquel cas
/// le parseur n'a pas de second mot-clé à consommer et `end` devient le mot-clé du
/// membre lui-même plutôt qu'un simple préfixe. Les deux formes comptent comme `end`.
fn is_end_member(keyword: &str, prefixes: &[String]) -> bool {
    head_kw(keyword) == "end" || prefixes.iter().any(|p| p == "end")
}

/// `ConcernDefinition`/`ConcernUsage` et `ViewpointDefinition`/`ViewpointUsage`
/// spécialisent tous deux `RequirementDefinition`/`RequirementUsage` dans le
/// métamodèle SysML v2 (même corps `RequirementBody`) : tout ce qui n'est
/// légal que dans une exigence l'est donc transitivement aussi dans un
/// `concern` ou un `viewpoint`.
fn is_requirement_family(pkw: &str) -> bool {
    pkw.contains("requirement") || pkw.contains("concern") || pkw.contains("viewpoint")
}

/// `AnalysisCaseDefinition`/`VerificationCaseDefinition`/`UseCaseDefinition`
/// spécialisent tous `CaseDefinition` : même logique que ci-dessus.
fn is_case_family(pkw: &str) -> bool {
    pkw.contains("case") || pkw.contains("analysis") || pkw.contains("verification")
}

fn legacy_hint(kw: &str) -> &'static str {
    match kw {
        "block" => "remplace `block` par `part def` (ou `item def` pour un élément non structurel)",
        "value" => "remplace `value` par `attribute`",
        "class" => "remplace `class` par `item def` ou `part def`",
        "association" => "remplace `association` par `connection def`",
        "stereotype" => "remplace `stereotype` par `metadata def` (appliqué avec `#MonMetadata`)",
        "property" => "remplace `property` par `attribute`, `part` ou `ref`",
        "operation" => "remplace `operation` par `action def` (ou `calc def` si la fonction retourne une valeur)",
        _ => "ce mot-clé n'existe pas en SysML v2",
    }
}

fn check_structure(m: &Model, id: usize, _opts: &Options, out: &mut Vec<Diagnostic>) {
    let s = &m.syms[id];
    let parent = s.parent.unwrap_or(0);
    let pkw = m.syms[parent].keyword.clone();
    let head = head_kw(&s.keyword);

    // E213 — mot-clé SysML v1 / UML
    if is_legacy_kw(head) {
        out.push(
            Diagnostic::error(
                "E213",
                "legacy-keyword",
                s.span,
                format!("`{}` n'existe pas en SysML v2", head),
            )
            .hint(legacy_hint(head).to_string()),
        );
    }

    // E210 — une définition ne se type pas avec `:`
    if s.is_def {
        for r in s.rels.iter() {
            if r.kind == RelKind::TypedBy && r.token == ":" {
                let name = s.name.clone().unwrap_or_else(|| "X".to_string());
                out.push(
                    Diagnostic::error(
                        "E210",
                        "def-typed-by-colon",
                        r.op_span,
                        "une définition ne peut pas être typée avec `:`".to_string(),
                    )
                    .hint(format!(
                        "les définitions se spécialisent : écris `{} {} :> {}`",
                        s.keyword,
                        name,
                        r.target.text()
                    )),
                );
            }
        }
    }

    // E212 — multiplicité sur une définition
    if s.is_def {
        if let Some(mu) = &s.mult {
            out.push(
                Diagnostic::error(
                    "E212",
                    "multiplicity-on-definition",
                    mu.span,
                    "une définition ne porte pas de multiplicité".to_string(),
                )
                .hint(
                    "la multiplicité se déclare sur l'usage : `part roues : Roue[4];`".to_string(),
                ),
            );
        }
    }

    // E218 — bornes de multiplicité incohérentes
    if let Some(mu) = &s.mult {
        let up = mu.upper.text.trim().to_string();
        let lo = mu.lower.as_ref().map(|l| l.text.trim().to_string());
        if let (Some(lo_s), true) = (lo.clone(), up != "*") {
            if let (Ok(a), Ok(b)) = (lo_s.parse::<i64>(), up.parse::<i64>()) {
                if a > b {
                    out.push(
                        Diagnostic::error(
                            "E218",
                            "invalid-multiplicity-range",
                            mu.span,
                            format!("borne inférieure ({}) supérieure à la borne supérieure ({})", a, b),
                        )
                        .hint(format!("écris `[{}..{}]`", b, a)),
                    );
                }
            }
        }
        if mu.lower.is_none() && up.is_empty() {
            out.push(
                Diagnostic::error(
                    "E218",
                    "empty-multiplicity",
                    mu.span,
                    "multiplicité vide".to_string(),
                )
                .hint("écris par exemple `[1]`, `[0..1]` ou `[0..*]`".to_string()),
            );
        }
    }

    // E215 — `end` hors d'une connexion / interface
    if is_end_member(&s.keyword, &s.prefixes) {
        let ok = pkw.contains("connection")
            || pkw.contains("interface")
            || pkw.contains("allocation")
            || pkw.contains("flow");
        if !ok {
            out.push(
                Diagnostic::error(
                    "E215",
                    "end-outside-connection",
                    s.span,
                    "`end` n'est valide que dans une `connection def`, une `interface def` ou une `allocation def`".to_string(),
                )
                .hint("retire `end`, ou déplace ce membre dans une `connection def`".to_string()),
            );
        }
    }

    // E216 — `subject` hors exigence / cas
    if head == "subject" {
        let ok = is_requirement_family(&pkw) || is_case_family(&pkw);
        if !ok {
            out.push(
                Diagnostic::error(
                    "E216",
                    "subject-outside-requirement",
                    s.span,
                    "`subject` n'est valide que dans une exigence (`requirement`), un `concern` ou un `case`".to_string(),
                )
                .hint("utilise `ref part` si tu voulais simplement référencer un élément".to_string()),
            );
        }
    }

    // E231 — `actor` hors exigence / cas
    if head == "actor" {
        let ok = is_requirement_family(&pkw) || is_case_family(&pkw);
        if !ok {
            out.push(
                Diagnostic::error(
                    "E231",
                    "actor-outside-requirement-or-case",
                    s.span,
                    "`actor` n'est valide que dans une exigence (`requirement`) ou un cas (`case`)".to_string(),
                )
                .hint("déplace cet `actor` dans une `requirement def` ou une `case def`".to_string()),
            );
        }
    }

    // E232 — `stakeholder` hors exigence (contrairement à `actor`, pas dans un `case`)
    if head == "stakeholder" {
        let ok = is_requirement_family(&pkw);
        if !ok {
            out.push(
                Diagnostic::error(
                    "E232",
                    "stakeholder-outside-requirement",
                    s.span,
                    "`stakeholder` n'est valide que dans une exigence (`requirement`)".to_string(),
                )
                .hint("déplace ce `stakeholder` dans une `requirement def`, ou utilise `actor` si un `case` est concerné".to_string()),
            );
        }
    }

    // E233 — `require` / `assume` hors exigence
    if head == "require" || head == "assume" {
        let ok = is_requirement_family(&pkw);
        if !ok {
            out.push(
                Diagnostic::error(
                    "E233",
                    "require-assume-outside-requirement",
                    s.span,
                    format!("`{}` n'est valide que dans une exigence (`requirement`)", head),
                )
                .hint("déplace cette contrainte dans une `requirement def`".to_string()),
            );
        }
    }

    // E234 — `objective` hors cas (contrairement à `require`/`assume`, pas dans une exigence)
    if head == "objective" {
        let ok = is_case_family(&pkw);
        if !ok {
            out.push(
                Diagnostic::error(
                    "E234",
                    "objective-outside-case",
                    s.span,
                    "`objective` n'est valide que dans un cas (`case`)".to_string(),
                )
                .hint("déplace cet `objective` dans une `case def` (ou `analysis def` / `verification def` / `use case def`)".to_string()),
            );
        }
    }

    // E235 — `frame` hors exigence (`FramedConcernMember` n'est atteignable
    // que depuis le corps d'une exigence dans la grammaire réelle)
    if head == "frame" {
        let ok = is_requirement_family(&pkw);
        if !ok {
            out.push(
                Diagnostic::error(
                    "E235",
                    "frame-outside-requirement",
                    s.span,
                    "`frame` n'est valide que dans une exigence (`requirement`)".to_string(),
                )
                .hint("déplace ce `frame` dans une `requirement def` (ou un `concern`/`viewpoint`)".to_string()),
            );
        }
    }

    // E236 — `verify` hors exigence (forme approchée : la grammaire réelle
    // restreint plus finement `verify` à l'`objective` d'un cas de
    // vérification, mais ce rapprochement suit la même granularité que les
    // autres règles de portée ci-dessus, qui ne vérifient que le parent
    // immédiat)
    if head == "verify" {
        let ok = is_requirement_family(&pkw);
        if !ok {
            out.push(
                Diagnostic::error(
                    "E236",
                    "verify-outside-requirement",
                    s.span,
                    "`verify` n'est valide que dans une exigence (`requirement`)".to_string(),
                )
                .hint("déplace ce `verify` dans une `requirement def`".to_string()),
            );
        }
    }

    // E222 — `variant` hors d'une `variation`
    if s.prefixes.iter().any(|p| p == "variant") {
        let pv = m.syms[parent].prefixes.iter().any(|p| p == "variation")
            || m.syms[parent].keyword.contains("variation");
        if !pv {
            out.push(
                Diagnostic::error(
                    "E222",
                    "variant-outside-variation",
                    s.span,
                    "`variant` n'est valide que dans un élément déclaré `variation`".to_string(),
                )
                .hint("ajoute `variation` sur l'élément englobant : `variation part def Moteur { ... }`".to_string()),
            );
        }
    }

    // E227 — paquet dans une définition
    if s.kind == NodeKind::Package && parent != 0 {
        let pk = m.syms[parent].kind;
        if pk == NodeKind::Def || pk == NodeKind::Usage {
            out.push(
                Diagnostic::error(
                    "E227",
                    "package-inside-definition",
                    s.span,
                    "un `package` ne peut pas être déclaré dans une définition ou un usage".to_string(),
                )
                .hint("sors le paquet au niveau supérieur du fichier".to_string()),
            );
        }
    }
}

// --------------------------------------------------------------------------
// Doublons
// --------------------------------------------------------------------------

fn check_duplicates(m: &Model, out: &mut Vec<Diagnostic>) {
    for scope in 0..m.syms.len() {
        let kids = m.syms[scope].children.clone();
        let mut seen: HashMap<String, Span> = HashMap::new();
        for c in kids {
            let s = &m.syms[c];
            let name = match &s.name {
                Some(n) => n.clone(),
                None => continue,
            };
            if s.kind == NodeKind::Doc || s.kind == NodeKind::Comment {
                continue;
            }
            match seen.get(&name) {
                Some(first) => {
                    out.push(
                        Diagnostic::error(
                            "E201",
                            "duplicate-name",
                            s.name_span,
                            format!(
                                "`{}` est déjà déclaré dans cette portée (ligne {})",
                                name, first.line
                            ),
                        )
                        .hint("renomme l'un des deux éléments : les noms doivent être uniques dans une même portée".to_string()),
                    );
                }
                None => {
                    seen.insert(name, s.name_span);
                }
            }
        }
    }
}

// --------------------------------------------------------------------------
// Références
// --------------------------------------------------------------------------

fn ctx_label(ctx: RefCtx) -> &'static str {
    match ctx {
        RefCtx::ConnectEnd => "extrémité de connexion",
        RefCtx::FlowEnd => "extrémité de flux",
        RefCtx::BindEnd => "extrémité de liaison",
        RefCtx::TransitionSource => "état source de transition",
        RefCtx::TransitionTarget => "état cible de transition",
        RefCtx::SatisfyTarget => "exigence satisfaite",
        RefCtx::SatisfyBy => "élément satisfaisant l'exigence",
        RefCtx::AllocateSource => "source d'allocation",
        RefCtx::AllocateTarget => "cible d'allocation",
        RefCtx::About => "cible de commentaire",
        RefCtx::ImportTarget => "cible d'import",
        RefCtx::AliasTarget => "cible d'alias",
        RefCtx::Annotation => "métadonnée appliquée",
        RefCtx::Value => "expression de valeur",
        RefCtx::Other => "référence",
    }
}

/// Suggestion « vouliez-vous dire ? » par distance d'édition.
fn nearest_name(m: &Model, scope: usize, target: &str) -> Option<String> {
    let mut best: Option<(usize, String)> = None;
    let mut visit: Vec<usize> = vec![scope];
    let mut seen: HashSet<usize> = HashSet::new();
    let mut cur = Some(scope);
    while let Some(s) = cur {
        visit.push(s);
        cur = m.syms[s].parent;
    }
    let mut candidates: Vec<String> = Vec::new();
    for s in visit {
        if !seen.insert(s) {
            continue;
        }
        for &c in m.syms[s].children.iter() {
            if let Some(n) = &m.syms[c].name {
                candidates.push(n.clone());
            }
        }
    }
    for cand in candidates {
        if cand == target {
            continue;
        }
        let d = edit_distance(&cand.to_lowercase(), &target.to_lowercase());
        let limit = if target.chars().count() <= 4 { 1 } else { 2 };
        if d <= limit {
            let better = match &best {
                Some((bd, _)) => d < *bd,
                None => true,
            };
            if better {
                best = Some((d, cand));
            }
        }
    }
    best.map(|(_, n)| n)
}

fn edit_distance(a: &str, b: &str) -> usize {
    let av: Vec<char> = a.chars().collect();
    let bv: Vec<char> = b.chars().collect();
    let n = bv.len();
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut cur: Vec<usize> = vec![0; n + 1];
    for i in 1..=av.len() {
        cur[0] = i;
        for j in 1..=n {
            let cost = if av[i - 1] == bv[j - 1] { 0 } else { 1 };
            let mut best = prev[j] + 1;
            if cur[j - 1] + 1 < best {
                best = cur[j - 1] + 1;
            }
            if prev[j - 1] + cost < best {
                best = prev[j - 1] + cost;
            }
            cur[j] = best;
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[n]
}

fn check_references(m: &Model, id: usize, opts: &Options, ctx: &Ctx, out: &mut Vec<Diagnostic>) {
    if opts.unresolved == UnresolvedMode::Off {
        return;
    }
    let owner = m.syms[id].parent.unwrap_or(0);

    // Cibles de relations (typage, spécialisation, redéfinition...)
    let rels = m.syms[id].rels.clone();
    for r in rels.iter() {
        let mut res = Resolver::new(m);
        let out_res = res.resolve(owner, &r.target);
        report_resolution(
            m,
            owner,
            &r.target,
            out_res,
            RefCtx::Other,
            opts,
            ctx,
            out,
            Some(r.kind),
        );

        // E214 — cible de redéfinition absente des supertypes
        if r.kind == RelKind::Redefines && r.target.parts.len() == 1 {
            check_redefines(m, id, &r.target, out);
        }
    }

    // Autres références
    let refs = m.syms[id].refs.clone();
    for rf in refs.iter() {
        let mut res = Resolver::new(m);
        let scope = owner;
        let out_res = res.resolve(scope, &rf.qname);
        report_resolution(m, scope, &rf.qname, out_res, rf.ctx, opts, ctx, out, None);

        // E230 — `satisfy X` où X n'est pas une exigence
        if rf.ctx == RefCtx::SatisfyTarget {
            if let Res::Local(t) = out_res {
                if !m.syms[t].keyword.contains("requirement") {
                    out.push(
                        Diagnostic::error(
                            "E230",
                            "satisfy-target-not-requirement",
                            rf.qname.span,
                            format!(
                                "`{}` est un `{}`, pas une exigence : `satisfy` doit viser une `requirement`",
                                rf.qname.text(),
                                m.syms[t].keyword
                            ),
                        )
                        .hint("vise une `requirement def` / `requirement`, ou utilise `allocate ... to ...`".to_string()),
                    );
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn report_resolution(
    m: &Model,
    scope: usize,
    q: &QName,
    res: Res,
    rctx: RefCtx,
    opts: &Options,
    ctx: &Ctx,
    out: &mut Vec<Diagnostic>,
    rel: Option<RelKind>,
) {
    match res {
        Res::Local(_) | Res::LibraryRoot | Res::Opaque => {}
        Res::LibraryLeaf => {
            let name = q.root().to_string();
            if let Some(pkg) = stdlib::suggest_import_for(&name) {
                if !ctx.imported_roots.contains(pkg) {
                    out.push(
                        Diagnostic::warn(
                            "W301",
                            "unimported-standard-type",
                            q.span,
                            format!("`{}` vient de la bibliothèque standard mais n'est pas importé", name),
                        )
                        .hint(format!("ajoute `import {}::*;` en tête du paquet", pkg)),
                    );
                }
            }
        }
        Res::Unknown(idx) => {
            let part = q.parts.get(idx).cloned();
            let (bad, span) = match part {
                Some(p) => (p.text.clone(), p.span),
                None => (q.text(), q.span),
            };

            // Une valeur d'expression est bien plus sujette aux faux positifs :
            // on ne la signale qu'en avertissement.
            let soft = rctx == RefCtx::Value || ctx.opaque_wildcard;
            let severity_error = opts.unresolved == UnresolvedMode::Error && !soft;

            let label = match rel {
                Some(RelKind::TypedBy) => "type",
                Some(RelKind::Specializes) => "supertype",
                Some(RelKind::Subsets) => "élément sous-ensemblé",
                Some(RelKind::Redefines) => "élément redéfini",
                Some(RelKind::References) => "élément référencé",
                Some(RelKind::Conjugates) => "port conjugué",
                Some(RelKind::Crosses) => "extrémité croisée",
                None => ctx_label(rctx),
            };

            let mut msg = format!("{} inconnu : `{}`", label, bad);
            if idx > 0 {
                msg = format!(
                    "`{}` n'est pas un membre de `{}`",
                    bad,
                    q.parts[..idx]
                        .iter()
                        .map(|p| p.text.clone())
                        .collect::<Vec<_>>()
                        .join("::")
                );
            }

            let mut d = if severity_error {
                Diagnostic::error("E200", "unresolved-name", span, msg)
            } else {
                Diagnostic::warn("W200", "unresolved-name", span, msg)
            };

            if let Some(sugg) = nearest_name(m, scope, &bad) {
                d = d.hint(format!("vouliez-vous dire `{}` ?", sugg));
            } else if ctx.opaque_wildcard {
                d = d.hint(
                    "aucune déclaration correspondante dans les fichiers analysés ; \
                     ce nom vient peut-être d'un paquet importé non fourni"
                        .to_string(),
                );
            } else {
                d = d.hint(format!(
                    "déclare l'élément (ex. `part def {};`) ou ajoute l'import du paquet qui le contient",
                    bad
                ));
            }
            out.push(d);
        }
    }
}

fn check_redefines(m: &Model, id: usize, target: &QName, out: &mut Vec<Diagnostic>) {
    let owner = match m.syms[id].parent {
        Some(p) if p != 0 => p,
        _ => return,
    };
    let name = target.last().to_string();
    let rels = m.syms[owner].rels.clone();
    if rels.is_empty() {
        return;
    }

    let mut supertypes: Vec<usize> = Vec::new();
    let owner_scope = m.syms[owner].parent.unwrap_or(0);
    for r in rels.iter() {
        if !matches!(
            r.kind,
            RelKind::Specializes | RelKind::Subsets | RelKind::TypedBy
        ) {
            continue;
        }
        let mut res = Resolver::new(m);
        match res.resolve(owner_scope, &r.target) {
            Res::Local(t) => supertypes.push(t),
            // Un supertype non résolu ou issu de la bibliothèque : on s'abstient.
            _ => return,
        }
    }
    if supertypes.is_empty() {
        return;
    }

    for t in supertypes.iter() {
        let mut res = Resolver::new(m);
        let mut v = Vec::new();
        if res.lookup(*t, &name, &mut v, 0).is_some() {
            return;
        }
    }

    let owner_name = m.syms[owner]
        .name
        .clone()
        .unwrap_or_else(|| "cet élément".to_string());
    out.push(
        Diagnostic::error(
            "E214",
            "redefines-target-not-inherited",
            target.span,
            format!(
                "`{}` n'existe dans aucun supertype de `{}` : rien à redéfinir",
                name, owner_name
            ),
        )
        .hint(format!(
            "déclare `{}` dans le supertype, ou remplace la redéfinition par une simple déclaration",
            name
        )),
    );
}

// --------------------------------------------------------------------------
// Règles de style / qualité
// --------------------------------------------------------------------------

fn first_char_upper(s: &str) -> bool {
    s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn first_char_lower(s: &str) -> bool {
    s.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
}

fn check_style(m: &Model, id: usize, opts: &Options, out: &mut Vec<Diagnostic>) {
    let s = &m.syms[id];

    // W310 — connexion sans extrémités
    if s.is_def && s.keyword.contains("connection") && s.has_body && s.rels.is_empty() {
        let ends = s
            .children
            .iter()
            .filter(|&&c| is_end_member(&m.syms[c].keyword, &m.syms[c].prefixes))
            .count();
        if ends == 0 {
            out.push(
                Diagnostic::warn(
                    "W310",
                    "connection-without-ends",
                    s.name_span,
                    "cette `connection def` ne déclare aucune extrémité `end`".to_string(),
                )
                .hint("déclare les extrémités : `end source : PortA;` et `end cible : PortB;`".to_string()),
            );
        }
    }

    if !opts.pedantic {
        return;
    }

    // W307 — exigence sans sujet
    if s.is_def && s.keyword.contains("requirement") && s.has_body && s.rels.is_empty() {
        let has_subject = s
            .children
            .iter()
            .any(|&c| head_kw(&m.syms[c].keyword) == "subject");
        if !has_subject {
            out.push(
                Diagnostic::warn(
                    "W307",
                    "requirement-without-subject",
                    s.name_span,
                    "cette `requirement def` ne déclare pas de `subject`".to_string(),
                )
                .hint("ajoute par exemple `subject véhicule : Véhicule;` pour préciser sur quoi porte l'exigence".to_string()),
            );
        }
    }

    // W311 — mots-clés absents de la grammaire SysML v2 employés seuls
    for p in &s.prefixes {
        let hint = match p.as_str() {
            "readonly" => Some(
                "SysML v2 utilise `constant` (valeur immuable à la création) ou `derived` \
                 (valeur calculée) plutôt que `readonly`",
            ),
            "composite" => {
                Some("la composition est implicite en SysML v2 (`part`), retire ce modificateur")
            }
            "portion" => Some("SysML v2 utilise `snapshot` ou `timeslice` plutôt que `portion`"),
            _ => None,
        };
        if let Some(hint) = hint {
            out.push(
                Diagnostic::warn(
                    "W311",
                    "non-standard-keyword",
                    s.name_span,
                    format!("`{}` n'est pas un mot-clé de la grammaire SysML v2", p),
                )
                .hint(hint.to_string()),
            );
        }
    }

    // W312 — mots-clés KerML absents de la surface SysML v2 (`feature`,
    // `namespace`, `specialization`, `subclassification` n'apparaissent
    // jamais dans `SysML.xtext` : la vraie grammaire les rejette en tête de
    // fichier ou dans un corps de définition, mais on ne casse pas les
    // fichiers existants — on avertit seulement).
    {
        let hint = match head_kw(&s.keyword) {
            "feature" => Some(
                "SysML v2 n'expose pas `feature` en surface — utilise `attribute`/`part`/`item`/`ref` selon le cas",
            ),
            "namespace" => Some("remplace par `package`"),
            "specialization" => Some(
                "exprime la spécialisation via `:>` ou `specializes` sur la définition elle-même, pas comme relation autonome",
            ),
            "subclassification" => Some(
                "exprime la spécialisation via `:>` ou `specializes` sur la définition elle-même, pas comme relation autonome",
            ),
            _ => None,
        };
        if let Some(hint) = hint {
            out.push(
                Diagnostic::warn(
                    "W312",
                    "kerml-only-keyword",
                    s.name_span,
                    format!("`{}` n'existe qu'au niveau KerML, absent de la grammaire de surface SysML v2", head_kw(&s.keyword)),
                )
                .hint(hint.to_string()),
            );
        }
    }

    // W302 — paquet vide
    if s.kind == NodeKind::Package && s.has_body && s.children.is_empty() {
        out.push(Diagnostic::warn(
            "W302",
            "empty-package",
            s.name_span,
            "paquet vide".to_string(),
        ));
    }

    // W306 — conventions de nommage
    if let Some(n) = &s.name {
        if !s.name_quoted && !n.is_empty() {
            if s.is_def && !first_char_upper(n) {
                out.push(
                    Diagnostic::warn(
                        "W306",
                        "naming-convention",
                        s.name_span,
                        format!("le nom de définition `{}` devrait commencer par une majuscule", n),
                    )
                    .hint("convention SysML v2 : UpperCamelCase pour les définitions".to_string()),
                );
            }
            if !s.is_def && s.kind == NodeKind::Usage && first_char_upper(n) {
                out.push(
                    Diagnostic::warn(
                        "W306",
                        "naming-convention",
                        s.name_span,
                        format!("le nom d'usage `{}` devrait commencer par une minuscule", n),
                    )
                    .hint("convention SysML v2 : lowerCamelCase pour les usages".to_string()),
                );
            }
        }
    }

    // W309 — usage sans type
    if !s.is_def
        && s.kind == NodeKind::Usage
        && s.rels.is_empty()
        && s.name.is_some()
        && matches!(head_kw(&s.keyword), "part" | "item" | "attribute" | "port" | "ref")
    {
        out.push(
            Diagnostic::warn(
                "W309",
                "untyped-usage",
                s.name_span,
                format!("l'usage `{}` n'est pas typé", s.name.clone().unwrap_or_default()),
            )
            .hint("ajoute un type : `part moteur : Moteur;`".to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diag::Severity;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn analyze_opts(src: &str, opts: &Options) -> Vec<Diagnostic> {
        let (toks, mut diags) = Lexer::new(src, 0).tokenize();
        let mut p = Parser::new(toks, 500);
        let nodes = p.parse_unit();
        diags.extend(p.diags.clone());
        assert!(diags.is_empty(), "erreur de syntaxe inattendue : {:?}", diags.iter().map(|d| d.code).collect::<Vec<_>>());
        let model = Model::build(&nodes);
        diags.extend(check(&model, opts));
        diags
    }

    fn analyze(src: &str) -> Vec<Diagnostic> {
        analyze_opts(src, &Options::default())
    }

    fn codes(d: &[Diagnostic]) -> Vec<&'static str> {
        d.iter().map(|x| x.code).collect()
    }

    fn has(d: &[Diagnostic], code: &str) -> bool {
        codes(d).contains(&code)
    }

    fn errors(d: &[Diagnostic]) -> usize {
        d.iter().filter(|x| x.severity == Severity::Error).count()
    }

    // -- Cas nominal -------------------------------------------------------

    #[test]
    fn nominal_vehicle_like_model_has_no_errors() {
        let src = r#"
            package VehicleModel {
                import ISQ::*;
                part def Wheel { attribute diametre : ISQ::LengthValue; }
                part def Engine { attribute masse : ISQ::MassValue; }
                part def Vehicle {
                    part moteur : Engine[1];
                    part roues : Wheel[4];
                }
                connection def FuelLine {
                    end source : Engine;
                    end cible : Vehicle;
                }
                requirement def MasseMaximale {
                    subject vehicule : Vehicle;
                    attribute masseMax : ISQ::MassValue;
                    require constraint { vehicule.masseTotale <= masseMax }
                }
            }
        "#;
        let d = analyze(src);
        assert_eq!(errors(&d), 0, "{:?}", d);
    }

    #[test]
    fn empty_model_has_no_diagnostics() {
        assert!(analyze("").is_empty());
    }

    // -- Une règle par test --------------------------------------------------

    #[test]
    fn e201_duplicate_name_in_same_scope() {
        let d = analyze("package P { part def Robot; part def Robot; }");
        assert!(has(&d, "E201"));
    }

    #[test]
    fn e201_same_name_in_different_scopes_is_allowed() {
        let d = analyze("package P { part def A { attribute x : Integer; } part def B { attribute x : Integer; } }");
        assert!(!has(&d, "E201"), "{:?}", d);
    }

    #[test]
    fn e210_definition_typed_by_colon_instead_of_specializes() {
        let d = analyze("package P { part def Vehicle; part def Robot : Vehicle; }");
        assert!(has(&d, "E210"));
    }

    #[test]
    fn e210_does_not_fire_on_usage_typing() {
        let d = analyze("package P { part def Robot; part r : Robot; }");
        assert!(!has(&d, "E210"), "{:?}", d);
    }

    #[test]
    fn e212_multiplicity_on_definition() {
        let d = analyze("package P { part def Robot[1]; }");
        assert!(has(&d, "E212"));
    }

    #[test]
    fn e213_flags_every_legacy_keyword() {
        for kw in ["block", "value", "class", "association", "stereotype", "property", "operation"] {
            let src = format!("package P {{ {} X; }}", kw);
            let d = analyze(&src);
            assert!(has(&d, "E213"), "mot-clé {} non signalé : {:?}", kw, d);
        }
    }

    #[test]
    fn e213_does_not_flag_current_keywords() {
        let d = analyze("package P { part def Robot; item def Fuel; port def P1; }");
        assert!(!has(&d, "E213"), "{:?}", d);
    }

    #[test]
    fn e214_redefines_target_absent_from_supertypes() {
        let src = r#"
            package P {
                part def Base { attribute x : Integer; }
                part def Derived :> Base {
                    attribute z : Integer;
                    attribute y :>> z;
                }
            }
        "#;
        let d = analyze(src);
        assert!(has(&d, "E214"), "{:?}", d);
    }

    #[test]
    fn e214_does_not_fire_when_target_is_inherited() {
        let src = r#"
            package P {
                part def Base { attribute x : Integer; }
                part def Derived :> Base {
                    attribute y :>> x;
                }
            }
        "#;
        let d = analyze(src);
        assert!(!has(&d, "E214"), "{:?}", d);
    }

    #[test]
    fn e215_end_outside_connection_interface_or_allocation() {
        let d = analyze("package P { part def Robot { end e1 : Robot; } }");
        assert!(has(&d, "E215"));
    }

    #[test]
    fn e215_does_not_fire_inside_a_connection() {
        let d = analyze("package P { part def X; connection def C { end a : X; end b : X; } }");
        assert!(!has(&d, "E215"), "{:?}", d);
    }

    #[test]
    fn e216_subject_outside_requirement_case_or_concern() {
        let d = analyze("package P { part def Robot { subject r : Robot; } }");
        assert!(has(&d, "E216"));
    }

    #[test]
    fn e216_does_not_fire_inside_a_requirement() {
        let d = analyze("package P { part def Robot; requirement def R { subject r : Robot; } }");
        assert!(!has(&d, "E216"), "{:?}", d);
    }

    #[test]
    fn e216_subject_allowed_inside_a_concern() {
        // ConcernDefinition spécialise RequirementDefinition dans le métamodèle.
        let d = analyze("package P { part def Robot; concern def C { subject r : Robot; } }");
        assert!(!has(&d, "E216"), "{:?}", d);
    }

    #[test]
    fn e231_actor_outside_requirement_or_case() {
        let d = analyze("package P { part def Robot { actor a; } }");
        assert!(has(&d, "E231"));
    }

    #[test]
    fn e231_actor_allowed_inside_requirement_and_case() {
        let d = analyze("package P { requirement def R { actor a; } case def C { actor a; } }");
        assert!(!has(&d, "E231"), "{:?}", d);
    }

    #[test]
    fn e232_stakeholder_outside_requirement() {
        let d = analyze("package P { part def Robot { stakeholder s; } }");
        assert!(has(&d, "E232"));
    }

    #[test]
    fn e232_stakeholder_not_allowed_inside_a_case() {
        // Contrairement à `actor`, `stakeholder` n'est légal que dans une exigence.
        let d = analyze("package P { case def C { stakeholder s; } }");
        assert!(has(&d, "E232"), "{:?}", d);
    }

    #[test]
    fn e232_stakeholder_allowed_inside_requirement() {
        let d = analyze("package P { requirement def R { stakeholder s; } }");
        assert!(!has(&d, "E232"), "{:?}", d);
    }

    #[test]
    fn e233_require_outside_requirement() {
        let d = analyze("package P { part def Robot { require constraint { true } } }");
        assert!(has(&d, "E233"));
    }

    #[test]
    fn e233_assume_outside_requirement() {
        let d = analyze("package P { part def Robot { assume constraint { true } } }");
        assert!(has(&d, "E233"));
    }

    #[test]
    fn e233_require_allowed_inside_requirement() {
        let d = analyze("package P { requirement def R { require constraint { true } } }");
        assert!(!has(&d, "E233"), "{:?}", d);
    }

    #[test]
    fn e234_objective_outside_case() {
        let d = analyze("package P { requirement def R { objective o; } }");
        assert!(has(&d, "E234"), "{:?}", d);
    }

    #[test]
    fn e234_objective_allowed_inside_case() {
        let d = analyze("package P { case def C { objective o; } }");
        assert!(!has(&d, "E234"), "{:?}", d);
    }

    #[test]
    fn e235_frame_outside_requirement() {
        let d = analyze("package P { concern def C; part def Robot { frame concern c : C; } }");
        assert!(has(&d, "E235"), "{:?}", d);
    }

    #[test]
    fn e235_frame_allowed_inside_requirement() {
        let d = analyze("package P { concern def C; requirement def R { frame concern c : C; } }");
        assert!(!has(&d, "E235"), "{:?}", d);
    }

    #[test]
    fn e235_frame_allowed_inside_viewpoint() {
        let d = analyze("package P { concern def C; viewpoint def V { frame concern c : C; } }");
        assert!(!has(&d, "E235"), "{:?}", d);
    }

    #[test]
    fn e236_verify_outside_requirement() {
        let d = analyze("package P { part def Robot { verify; } }");
        assert!(has(&d, "E236"), "{:?}", d);
    }

    #[test]
    fn e236_verify_allowed_inside_requirement() {
        let d = analyze("package P { requirement def R { verify; } }");
        assert!(!has(&d, "E236"), "{:?}", d);
    }

    #[test]
    fn e216_subject_allowed_inside_viewpoint() {
        let d = analyze("package P { part def Robot; viewpoint def V { subject r : Robot; } }");
        assert!(!has(&d, "E216"), "{:?}", d);
    }

    #[test]
    fn e231_actor_allowed_inside_viewpoint() {
        let d = analyze("package P { viewpoint def V { actor a; } }");
        assert!(!has(&d, "E231"), "{:?}", d);
    }

    #[test]
    fn e232_stakeholder_allowed_inside_viewpoint() {
        let d = analyze("package P { viewpoint def V { stakeholder s; } }");
        assert!(!has(&d, "E232"), "{:?}", d);
    }

    #[test]
    fn e233_require_allowed_inside_viewpoint() {
        let d = analyze("package P { viewpoint def V { require constraint { true } } }");
        assert!(!has(&d, "E233"), "{:?}", d);
    }

    #[test]
    fn e218_lower_bound_greater_than_upper_bound() {
        let d = analyze("package P { part def S; part def R { part s : S[5..2]; } }");
        assert!(has(&d, "E218"));
    }

    #[test]
    fn e218_empty_multiplicity_brackets() {
        let d = analyze("package P { part def S; part def R { part s : S[]; } }");
        assert!(has(&d, "E218"));
    }

    #[test]
    fn e218_open_upper_bound_is_valid() {
        let d = analyze("package P { part def S; part def R { part s : S[0..*]; } }");
        assert!(!has(&d, "E218"), "{:?}", d);
    }

    #[test]
    fn e222_variant_outside_variation() {
        let d = analyze("package P { part def Robot { variant part v1 : Robot; } }");
        assert!(has(&d, "E222"));
    }

    #[test]
    fn e222_does_not_fire_inside_a_variation() {
        let d = analyze("package P { part def A; variation part def V { variant part v1 : A; } }");
        assert!(!has(&d, "E222"), "{:?}", d);
    }

    #[test]
    fn e227_package_inside_definition() {
        let d = analyze("package P { part def Robot { package Sub { } } }");
        assert!(has(&d, "E227"));
    }

    #[test]
    fn e227_does_not_fire_for_nested_packages_at_package_level() {
        let d = analyze("package P { package Sub { } }");
        assert!(!has(&d, "E227"), "{:?}", d);
    }

    #[test]
    fn e230_satisfy_target_not_a_requirement() {
        let src = r#"
            package P {
                part def Robot;
                part def Sensor;
                part r : Robot {
                    satisfy Sensor by r;
                }
            }
        "#;
        let d = analyze(src);
        assert!(has(&d, "E230"), "{:?}", d);
    }

    #[test]
    fn e230_does_not_fire_when_target_is_a_requirement() {
        let src = r#"
            package P {
                part def Robot;
                requirement def NeedsPower;
                part r : Robot {
                    satisfy requirement NeedsPower by r;
                }
            }
        "#;
        let d = analyze(src);
        assert!(!has(&d, "E230"), "{:?}", d);
    }

    #[test]
    fn e230_fires_through_assert_satisfy_wrapper() {
        // Régression : `assert satisfy` doit rester classé comme relation
        // `satisfy` (voir `ast::classify` / `parser::is_relationship_kw`),
        // pas comme un `assert` générique qui échapperait à E230.
        let src = r#"
            package P {
                part def Robot;
                part def Sensor;
                part r : Robot {
                    assert satisfy Sensor by r;
                }
            }
        "#;
        let d = analyze(src);
        assert!(has(&d, "E230"), "{:?}", d);
    }

    #[test]
    fn e230_fires_through_not_satisfy_wrapper() {
        let src = r#"
            package P {
                part def Robot;
                part def Sensor;
                part r : Robot {
                    not satisfy Sensor by r;
                }
            }
        "#;
        let d = analyze(src);
        assert!(has(&d, "E230"), "{:?}", d);
    }

    #[test]
    fn e230_fires_through_assert_not_satisfy_wrapper() {
        let src = r#"
            package P {
                part def Robot;
                part def Sensor;
                part r : Robot {
                    assert not satisfy Sensor by r;
                }
            }
        "#;
        let d = analyze(src);
        assert!(has(&d, "E230"), "{:?}", d);
    }

    #[test]
    fn e230_does_not_fire_through_assert_satisfy_wrapper_with_a_valid_target() {
        let src = r#"
            package P {
                requirement def NeedsPower;
                part def Robot;
                part r : Robot {
                    assert satisfy NeedsPower by r;
                }
            }
        "#;
        let d = analyze(src);
        assert!(!has(&d, "E230"), "{:?}", d);
    }

    #[test]
    fn e200_unresolved_type_is_an_error_by_default() {
        let d = analyze("package P { part def Robot { part s : Ghost; } }");
        assert!(has(&d, "E200"));
        assert_eq!(errors(&d), 1);
    }

    #[test]
    fn w200_unresolved_name_in_value_expression_is_downgraded_to_warning() {
        let d = analyze("package P { part def Robot { attribute x = someUnknownValue; } }");
        assert!(has(&d, "W200"));
        assert!(!has(&d, "E200"));
        assert_eq!(errors(&d), 0);
    }

    #[test]
    fn w301_standard_library_type_used_without_import() {
        let d = analyze("package P { part def Robot { attribute masse : MassValue; } }");
        assert!(has(&d, "W301"));
    }

    #[test]
    fn w301_does_not_fire_once_imported() {
        let d = analyze("package P { import ISQ::*; part def Robot { attribute masse : MassValue; } }");
        assert!(!has(&d, "W301"), "{:?}", d);
    }

    #[test]
    fn w307_requirement_def_without_subject() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts("package P { requirement def R { attribute x : Integer; } }", &o);
        assert!(has(&d, "W307"));
    }

    #[test]
    fn w307_does_not_fire_once_a_subject_is_declared() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts(
            "package P { part def Robot; requirement def R { subject r : Robot; } }",
            &o,
        );
        assert!(!has(&d, "W307"), "{:?}", d);
    }

    #[test]
    fn w307_requires_pedantic_to_fire() {
        let d = analyze("package P { requirement def R { attribute x : Integer; } }");
        assert!(!has(&d, "W307"), "{:?}", d);
    }

    #[test]
    fn w311_readonly_flags_non_standard_keyword() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts(
            "package P { part def Robot { readonly attribute x : Integer; } }",
            &o,
        );
        assert!(has(&d, "W311"), "{:?}", d);
    }

    #[test]
    fn w311_composite_flags_non_standard_keyword() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts(
            "package P { part def Robot; part def R { composite part r : Robot; } }",
            &o,
        );
        assert!(has(&d, "W311"), "{:?}", d);
    }

    #[test]
    fn w311_portion_flags_non_standard_keyword() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts(
            "package P { part def Robot { portion attribute x : Integer; } }",
            &o,
        );
        assert!(has(&d, "W311"), "{:?}", d);
    }

    #[test]
    fn w311_requires_pedantic_to_fire() {
        let d = analyze("package P { part def Robot { readonly attribute x : Integer; } }");
        assert!(!has(&d, "W311"), "{:?}", d);
    }

    #[test]
    fn w311_does_not_flag_standard_modifiers() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts(
            "package P { part def Robot { derived attribute x : Integer; } }",
            &o,
        );
        assert!(!has(&d, "W311"), "{:?}", d);
    }

    #[test]
    fn w312_flags_each_kerml_only_keyword() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts(
            "package P { feature x : Integer; namespace N { } specialization : Integer; subclassification : Integer; }",
            &o,
        );
        let count = d.iter().filter(|x| x.code == "W312").count();
        assert_eq!(count, 4, "{:?}", d);
    }

    #[test]
    fn w312_requires_pedantic_to_fire() {
        let d = analyze("package P { feature x : Integer; }");
        assert!(!has(&d, "W312"), "{:?}", d);
    }

    #[test]
    fn w312_does_not_flag_standard_keywords() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts("package P { part def Robot; }", &o);
        assert!(!has(&d, "W312"), "{:?}", d);
    }

    #[test]
    fn w310_connection_def_without_any_end() {
        let d = analyze("package P { connection def C { attribute x : Integer; } }");
        assert!(has(&d, "W310"));
    }

    #[test]
    fn w310_does_not_fire_once_ends_are_declared() {
        let d = analyze("package P { part def X; connection def C { end a : X; end b : X; } }");
        assert!(!has(&d, "W310"), "{:?}", d);
    }

    // -- Options CLI --------------------------------------------------------

    #[test]
    fn pedantic_rules_are_silent_by_default() {
        let src = "package Empty { }";
        assert!(!has(&analyze(src), "W302"));
    }

    #[test]
    fn pedantic_rules_activate_with_the_flag() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts("package Empty { }", &o);
        assert!(has(&d, "W302"));
    }

    #[test]
    fn pedantic_naming_convention_flags_lowercase_definitions_and_uppercase_usages() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts("package p { part def robot; part Robot : robot; }", &o);
        let count = d.iter().filter(|x| x.code == "W306").count();
        assert_eq!(count, 2, "{:?}", d);
    }

    #[test]
    fn pedantic_untyped_usage_flags_missing_type() {
        let mut o = Options::default();
        o.pedantic = true;
        let d = analyze_opts("package P { part def Robot { part sensor; } }", &o);
        assert!(has(&d, "W309"));
    }

    #[test]
    fn unresolved_mode_off_suppresses_both_error_and_warning_variants() {
        let mut o = Options::default();
        o.unresolved = UnresolvedMode::Off;
        let d = analyze_opts("package P { part def Robot { part s : Ghost; } }", &o);
        assert!(!has(&d, "E200"));
        assert!(!has(&d, "W200"));
    }

    #[test]
    fn unresolved_mode_warn_downgrades_error_to_warning() {
        let mut o = Options::default();
        o.unresolved = UnresolvedMode::Warn;
        let d = analyze_opts("package P { part def Robot { part s : Ghost; } }", &o);
        assert!(has(&d, "W200"));
        assert_eq!(errors(&d), 0);
    }

    // -- Résolution multi-fichiers -------------------------------------------

    #[test]
    fn cross_file_resolution_via_qualified_name() {
        let (toks_a, _) = Lexer::new("package Shared { part def Sensor; }", 0).tokenize();
        let mut pa = Parser::new(toks_a, 500);
        let nodes_a = pa.parse_unit();
        assert!(pa.diags.is_empty());

        let (toks_b, _) = Lexer::new("package Consumer { part def Robot { part s : Shared::Sensor; } }", 1).tokenize();
        let mut pb = Parser::new(toks_b, 500);
        let nodes_b = pb.parse_unit();
        assert!(pb.diags.is_empty());

        let mut all = nodes_a;
        all.extend(nodes_b);
        let model = Model::build(&all);
        let d = check(&model, &Options::default());
        assert!(d.is_empty(), "{:?}", d);
    }
}
