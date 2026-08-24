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
use crate::renames;
use crate::spec;
use crate::stdlib;

/// D'où une règle tire son autorité.
///
/// La distinction est le cœur de la conformité : un `Spec` est opposable — on
/// peut pointer la métaclasse ou la règle du validateur de référence dont il
/// découle —, un `Style` ne l'est pas.
///
/// Axe indépendant de `--pedantic`, qui ne filtre que les règles *bruyantes* :
/// quelques `Style` peu bavards (E105, E218, W310) restent actifs par défaut.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Authority {
    /// Syntaxe abstraite ou règle de bonne formation : `SysML.json`,
    /// `SysMLValidator.xtend`, `UsageUtil.java`.
    Spec,
    /// Notation textuelle : `SysML.xtext` / `KerML.xtext`. Le métamodèle ne
    /// décrit pas la forme concrète, ces règles n'en découlent donc pas.
    Grammar,
    /// Convention maison, sans base normative.
    Style,
}

impl Authority {
    pub fn as_str(self) -> &'static str {
        match self {
            Authority::Spec => "spec",
            Authority::Grammar => "grammar",
            Authority::Style => "style",
        }
    }
}

/// Catalogue exposé par `--list-rules` : (code, règle, autorité, description).
pub const CATALOG: &[(&str, &str, Authority, &str)] = &[
    ("E001", "unterminated-block-comment", Authority::Grammar, "Commentaire `/* ... */` non fermé."),
    ("E002", "unterminated-string", Authority::Grammar, "Chaîne ou nom entre apostrophes non terminé."),
    ("E003", "unexpected-character", Authority::Grammar, "Caractère hors grammaire (souvent un stéréotype UML `«...»`)."),
    ("E100", "member-must-start-with-keyword", Authority::Grammar, "Un membre doit commencer par un mot-clé SysML v2 (ou par sa partie de spécialisation : `:>> x = ...;`)."),
    ("E100", "unexpected-token", Authority::Grammar, "Jeton inattendu dans une déclaration."),
    ("E101", "expected-token", Authority::Grammar, "Jeton attendu absent."),
    ("E102", "unclosed-brace", Authority::Grammar, "Accolade, crochet ou multiplicité non fermé."),
    ("E103", "missing-semicolon", Authority::Grammar, "Déclaration non terminée par `;` ni par un corps `{ }`."),
    ("E104", "doc-without-body", Authority::Grammar, "`doc`, `comment` ou `rep` sans corps `/* ... */`."),
    ("E105", "nesting-too-deep", Authority::Style, "Imbrication supérieure à 64 niveaux."),
    ("E200", "unresolved-name", Authority::Spec, "Nom référencé introuvable dans le modèle ni dans la bibliothèque."),
    ("E201", "duplicate-name", Authority::Spec, "Deux éléments portent le même nom dans une même portée."),
    ("E210", "def-typed-by-colon", Authority::Grammar, "Une définition utilise `:` au lieu de `:>` (spécialisation)."),
    ("E212", "multiplicity-on-definition", Authority::Grammar, "Multiplicité placée sur une définition au lieu d'un usage."),
    ("E213", "legacy-keyword", Authority::Grammar, "Mot-clé SysML v1 / UML (`block`, `value`, `association`, ...)."),
    ("E214", "redefines-target-not-inherited", Authority::Spec, "`redefines` vise un élément absent des supertypes."),
    ("E215", "end-outside-connection", Authority::Grammar, "`end` hors d'une `connection def` / `interface def`."),
    ("E216", "subject-outside-requirement", Authority::Spec, "`subject` hors d'une exigence ou d'un `case` (validateSubjectMembershipOwningType)."),
    ("E218", "invalid-multiplicity-range", Authority::Style, "Bornes de multiplicité incohérentes ou vides (le validateur de référence ne le signale pas)."),
    ("E222", "variant-outside-variation", Authority::Spec, "`variant` hors d'un élément `variation`."),
    ("E225", "reserved-word-as-name", Authority::Grammar, "Mot réservé utilisé comme nom sans échappement `'...'`."),
    ("E227", "package-inside-definition", Authority::Grammar, "`package` déclaré dans une définition ou un usage."),
    ("E230", "satisfy-target-not-requirement", Authority::Spec, "`satisfy` vise un élément qui n'est pas une exigence."),
    ("E231", "actor-outside-requirement-or-case", Authority::Spec, "`actor` hors d'une exigence ou d'un `case` (validateActorMembershipOwningType)."),
    ("E232", "stakeholder-outside-requirement", Authority::Spec, "`stakeholder` hors d'une exigence (validateStakeholderMembershipOwningType)."),
    ("E233", "require-assume-outside-requirement", Authority::Spec, "`require` ou `assume` hors d'une exigence (validateRequirementConstraintMembershipOwningType)."),
    ("E234", "objective-outside-case", Authority::Spec, "`objective` hors d'un `case` (validateObjectiveMembershipOwningType)."),
    ("E235", "frame-outside-requirement", Authority::Spec, "`frame` hors d'une exigence (FramedConcernMembership hérite de RequirementConstraintMembership)."),
    ("E236", "verify-outside-verification-objective", Authority::Spec, "`verify` hors de l'`objective` d'un cas de vérification (UsageUtil.isLegalVerification)."),
    ("W200", "unresolved-name", Authority::Spec, "Nom non résolu dans un contexte tolérant (expression, import opaque)."),
    ("W301", "unimported-standard-type", Authority::Spec, "Type de la bibliothèque standard utilisé sans import."),
    ("W302", "empty-package", Authority::Style, "Paquet déclaré avec un corps vide (--pedantic)."),
    ("W306", "naming-convention", Authority::Style, "UpperCamelCase attendu pour les définitions, lowerCamelCase pour les usages (--pedantic)."),
    ("W307", "requirement-without-subject", Authority::Style, "`requirement def` sans `subject` (--pedantic)."),
    ("W309", "untyped-usage", Authority::Style, "Usage déclaré sans type (--pedantic)."),
    ("W310", "connection-without-ends", Authority::Style, "`connection def` sans extrémité `end`."),
    ("W311", "non-standard-keyword", Authority::Style, "Mot-clé absent de la grammaire SysML v2 (`readonly`, `composite`, `portion` employés seuls) (--pedantic)."),
    ("W312", "kerml-only-keyword", Authority::Style, "Mot-clé KerML absent de la surface SysML v2 (`feature`, `namespace`, `specialization`, `subclassification`) (--pedantic)."),
    ("W313", "public-import-at-top-level", Authority::Style, "`import` sans paquet englobant marqué `public`/`protected` au lieu de `private` (--pedantic)."),
    ("W314", "legacy-library-name", Authority::Spec, "Nom retiré de la bibliothèque standard depuis la version visée (--library-version)."),
];
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnresolvedMode {
    Error,
    Warn,
    Off,
}

/// Version de la bibliothèque standard que le modèle est censé viser.
///
/// **Déclaration, pas validation** : la résolution s'appuie toujours sur les
/// tables de `stdlib.rs` (2025-02). Choisir une version antérieure reclasse
/// seulement les noms connus de `renames.rs` en avertissement, au lieu de les
/// signaler comme introuvables.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LibraryVersion {
    V2024_11,
    V2025_02,
}

pub struct Options {
    pub pedantic: bool,
    pub unresolved: UnresolvedMode,
    pub library: LibraryVersion,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            pedantic: false,
            unresolved: UnresolvedMode::Error,
            library: LibraryVersion::V2025_02,
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
    fn lookup(
        &mut self,
        scope: usize,
        name: &str,
        visited: &mut Vec<usize>,
        depth: u32,
    ) -> Option<usize> {
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

impl Ctx {
    /// Liste des paquets standard importés, si `name` n'appartient à aucun
    /// paquet de la bibliothèque **et** qu'aucun import opaque ne pourrait
    /// l'apporter. `None` dès qu'on ne peut rien affirmer.
    fn known_library_imports(&self, name: &str) -> Option<String> {
        if self.opaque_wildcard || stdlib::is_library_leaf(name) {
            return None;
        }
        let mut pkgs: Vec<&str> = self
            .imported_roots
            .iter()
            .filter(|r| stdlib::is_library_root(r))
            .map(|r| r.as_str())
            .collect();
        if pkgs.is_empty() {
            return None;
        }
        pkgs.sort_unstable();
        Some(match pkgs.split_last() {
            Some((last, [])) => format!("`{last}` ne le définit pas"),
            Some((last, rest)) => {
                format!("ni `{}` ni `{last}` ne le définissent", rest.join("`, `"))
            }
            None => unreachable!(),
        })
    }
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
                // Résoudre depuis la portée *de l'import*, pas depuis la racine :
                // `package A { private import B::**; }` vise le frère `B`, que
                // le contexte racine ne voit pas. Résoudre en 0 le déclarait
                // opaque, et un seul import mal classé suffisait à faire
                // basculer tous les E200 du fichier en W200.
                let mut r = Resolver::new(m);
                if !matches!(r.resolve(s.id, imp), Res::Local(_)) {
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
    check_inherited_shadowing(m, &mut out);

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
/// Métaclasse du parent immédiat, pour les règles de portée ci-dessous.
///
/// Les règles du validateur de référence sont écrites en `instanceof` sur le
/// type *propriétaire* de l'appartenance (`SysMLValidator.xtend`), d'où le
/// passage par `spec::is_any_kind_of` plutôt que par des sous-chaînes de
/// mots-clés.
fn owner_metaclass(m: &Model, parent: usize) -> Option<&'static str> {
    let p = &m.syms[parent];
    crate::ast::metaclass_for(&p.keyword, p.is_def)
}

/// Contextes admis par `validateSubjectMembershipOwningType` et
/// `validateActorMembershipOwningType` : « Only requirements and cases can
/// have subjects/actors. »
const REQUIREMENT_OR_CASE: &[&str] = &[
    "RequirementDefinition",
    "RequirementUsage",
    "CaseDefinition",
    "CaseUsage",
];

/// Contextes admis par `validateStakeholderMembershipOwningType` et
/// `validateRequirementConstraintMembershipOwningType` (dont hérite
/// `FramedConcernMembership`) : « Only requirements can have ... ».
const REQUIREMENT_ONLY: &[&str] = &["RequirementDefinition", "RequirementUsage"];

/// Contextes admis par `validateObjectiveMembershipOwningType` :
/// « Only cases can have objectives. »
const CASE_ONLY: &[&str] = &["CaseDefinition", "CaseUsage"];

/// Émet `code` si le parent immédiat n'est pas (un sous-type d')`allowed`.
///
/// Un parent dont la métaclasse est inconnue est laissé passer : la table de
/// mots-clés ne couvre que les contextes pertinents, et il vaut mieux taire un
/// diagnostic que d'en inventer un.
#[allow(clippy::too_many_arguments)]
fn check_owner(
    m: &Model,
    parent: usize,
    allowed: &[&str],
    code: &'static str,
    rule: &'static str,
    span: Span,
    msg: &str,
    hint: &str,
    out: &mut Vec<Diagnostic>,
) {
    let Some(mc) = owner_metaclass(m, parent) else {
        return;
    };
    if !spec::is_any_kind_of(mc, allowed) {
        out.push(Diagnostic::error(code, rule, span, msg.to_string()).hint(hint.to_string()));
    }
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
                format!("`{head}` n'existe pas en SysML v2"),
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
                            format!(
                                "borne inférieure ({a}) supérieure à la borne supérieure ({b})"
                            ),
                        )
                        .hint(format!("écris `[{b}..{a}]`")),
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
    // validateSubjectMembershipOwningType : « Only requirements and cases can
    // have subjects. » `concern`, `viewpoint` et `satisfy` en héritent.
    if head == "subject" {
        check_owner(
            m,
            parent,
            REQUIREMENT_OR_CASE,
            "E216",
            "subject-outside-requirement",
            s.span,
            "`subject` n'est valide que dans une exigence (`requirement`, `concern`, `viewpoint`, `satisfy`) ou un cas (`case`)",
            "utilise `ref part` si tu voulais simplement référencer un élément",
            out,
        );
    }

    // E231 — `actor` hors exigence / cas
    // validateActorMembershipOwningType : « Only requirements and cases can
    // have actors. » Même ensemble que `subject`.
    if head == "actor" {
        check_owner(
            m,
            parent,
            REQUIREMENT_OR_CASE,
            "E231",
            "actor-outside-requirement-or-case",
            s.span,
            "`actor` n'est valide que dans une exigence (`requirement`, `concern`, `viewpoint`, `satisfy`) ou un cas (`case`)",
            "déplace cet `actor` dans une `requirement def` ou une `case def`",
            out,
        );
    }

    // E232 — `stakeholder` hors exigence
    // validateStakeholderMembershipOwningType : « Only requirements can have
    // stakeholders. » Contrairement à `actor`, les cas sont exclus.
    if head == "stakeholder" {
        check_owner(
            m,
            parent,
            REQUIREMENT_ONLY,
            "E232",
            "stakeholder-outside-requirement",
            s.span,
            "`stakeholder` n'est valide que dans une exigence (`requirement`, `concern`, `viewpoint`)",
            "déplace ce `stakeholder` dans une `requirement def`, ou utilise `actor` si un `case` est concerné",
            out,
        );
    }

    // E233 — `require` / `assume` hors exigence
    // validateRequirementConstraintMembershipOwningType : « Only requirements
    // can have assumed or required constraints. »
    if head == "require" || head == "assume" {
        check_owner(
            m,
            parent,
            REQUIREMENT_ONLY,
            "E233",
            "require-assume-outside-requirement",
            s.span,
            &format!("`{head}` n'est valide que dans une exigence (`requirement`, `concern`, `viewpoint`)"),
            "déplace cette contrainte dans une `requirement def`",
            out,
        );
    }

    // E234 — `objective` hors cas
    // validateObjectiveMembershipOwningType : « Only cases can have
    // objectives. » `use case`, `analysis` et `verification` en héritent.
    if head == "objective" {
        check_owner(
            m,
            parent,
            CASE_ONLY,
            "E234",
            "objective-outside-case",
            s.span,
            "`objective` n'est valide que dans un cas (`case`)",
            "déplace cet `objective` dans une `case def` (ou `analysis def` / `verification def` / `use case def`)",
            out,
        );
    }

    // E235 — `frame` hors exigence
    // `FramedConcernMembership` spécialise `RequirementConstraintMembership` :
    // il hérite donc de validateRequirementConstraintMembershipOwningType.
    if head == "frame" {
        check_owner(
            m,
            parent,
            REQUIREMENT_ONLY,
            "E235",
            "frame-outside-requirement",
            s.span,
            "`frame` n'est valide que dans une exigence (`requirement`, `concern`, `viewpoint`)",
            "déplace ce `frame` dans une `requirement def` (ou un `concern`/`viewpoint`)",
            out,
        );
    }

    // E236 — `verify` hors de l'objectif d'un cas de vérification
    //
    // La règle de référence n'est pas une simple contrainte sur le parent
    // immédiat (`UsageUtil.isLegalVerification`) :
    //
    //     owningType instanceof RequirementUsage && isObjective(owningType)
    //     && owningType.owningType instanceof VerificationCase{Definition,Usage}
    //
    // soit, en notation textuelle, `verify` doit se trouver dans le corps d'un
    // `objective` lui-même porté par un `verification def` / `verification`.
    // C'est le seul contrôle de portée du lot qui regarde le grand-parent.
    if head == "verify" {
        let in_objective = head_kw(&m.syms[parent].keyword) == "objective";
        let ok = in_objective
            && m.syms[parent]
                .parent
                .and_then(|gp| owner_metaclass(m, gp))
                .map(|mc| {
                    spec::is_kind_of(mc, "VerificationCaseDefinition")
                        || spec::is_kind_of(mc, "VerificationCaseUsage")
                })
                .unwrap_or(false);
        if !ok {
            out.push(
                Diagnostic::error(
                    "E236",
                    "verify-outside-verification-objective",
                    s.span,
                    "`verify` n'est valide que dans l'`objective` d'un cas de vérification"
                        .to_string(),
                )
                .hint(
                    "encadre-le ainsi : `verification def V { objective { verify MonExigence; } }`"
                        .to_string(),
                ),
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
                    "un `package` ne peut pas être déclaré dans une définition ou un usage"
                        .to_string(),
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

/// E201 (variante `shadows-inherited-member`) : un nouveau membre masque un
/// membre hérité d'un supertype sans le redéfinir explicitement (`:>>` /
/// `references`) — même mécanisme de résolution de supertype que
/// `check_redefines` (E214), dans l'autre sens : E214 vérifie qu'une
/// `redefines` cible bien un membre hérité existant, ceci vérifie qu'un
/// membre qui n'est *pas* une `redefines` n'entre pas en collision avec un
/// membre hérité.
fn check_inherited_shadowing(m: &Model, out: &mut Vec<Diagnostic>) {
    for scope in 1..m.syms.len() {
        let rels = m.syms[scope].rels.clone();
        if rels.is_empty() {
            continue;
        }
        let owner_scope = m.syms[scope].parent.unwrap_or(0);
        let mut supertypes: Vec<usize> = Vec::new();
        for r in rels.iter() {
            if !matches!(r.kind, RelKind::Specializes | RelKind::Subsets) {
                continue;
            }
            let mut res = Resolver::new(m);
            if let Res::Local(t) = res.resolve(owner_scope, &r.target) {
                supertypes.push(t);
            }
        }
        if supertypes.is_empty() {
            continue;
        }

        for &c in &m.syms[scope].children.clone() {
            let cs = &m.syms[c];
            let name = match &cs.name {
                Some(n) => n.clone(),
                None => continue,
            };
            if cs.kind == NodeKind::Doc || cs.kind == NodeKind::Comment {
                continue;
            }
            let overrides = cs
                .rels
                .iter()
                .any(|r| matches!(r.kind, RelKind::Redefines | RelKind::References));
            if overrides {
                continue;
            }
            for &t in &supertypes {
                let mut res = Resolver::new(m);
                let mut visited = Vec::new();
                if res.lookup(t, &name, &mut visited, 0).is_some() {
                    out.push(
                        Diagnostic::error(
                            "E201",
                            "shadows-inherited-member",
                            cs.name_span,
                            format!(
                                "`{name}` masque un membre hérité sans le redéfinir explicitement"
                            ),
                        )
                        .hint("utilise `:>>` (redefines) pour redéfinir explicitement le membre hérité, ou renomme ce membre".to_string()),
                    );
                    break;
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
        RefCtx::VerifyTarget => "élément vérifié",
        RefCtx::AllocateSource => "source d'allocation",
        RefCtx::AllocateTarget => "cible d'allocation",
        RefCtx::About => "cible de commentaire",
        RefCtx::ImportTarget => "cible d'import",
        RefCtx::AliasTarget => "cible d'alias",
        RefCtx::ExposeTarget => "cible d'exposition",
        RefCtx::Annotation => "métadonnée appliquée",
        RefCtx::Value => "expression de valeur",
        RefCtx::Other => "référence",
    }
}

/// Conseil de migration pour un nom retiré de la bibliothèque standard.
fn rename_migration_hint(r: &renames::Rename) -> String {
    match r.new {
        Some((pkg, name)) => format!(
            "pour viser la version courante : remplace `{}::{}` par `{pkg}::{name}`",
            r.old_pkg, r.old
        ),
        None => format!(
            "`{}::{}` n'a pas d'équivalent dans les versions ultérieures",
            r.old_pkg, r.old
        ),
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

/// Nom de la bibliothèque standard proche de `target`, avec son paquet.
///
/// Complète `nearest_name`, qui ne regarde que le modèle analysé : attrape les
/// fautes de frappe sur un type standard (`Strng` → `ScalarValues::String`).
/// Volontairement limité à la proximité orthographique — un nom composé comme
/// `FlowConnection` est trop loin de `Flow` pour être deviné ainsi, et c'est le
/// diagnostic « aucun paquet importé ne le fournit » qui le prend en charge.
fn nearest_library_name(target: &str) -> Option<(&'static str, &'static str)> {
    let lower = target.to_lowercase();
    // Même tolérance que `nearest_name` : au-delà, on suggère du bruit
    // (`Shared` → `Sphere`) plutôt qu'une faute de frappe.
    let limit = if target.chars().count() <= 4 { 1 } else { 2 };
    let mut best: Option<(usize, &'static str, &'static str)> = None;
    for name in stdlib::all_names() {
        if *name == target {
            continue;
        }
        // Le calcul est borné aux noms de longueur voisine : inutile de
        // comparer `Flow` à `AccelerationValue`.
        let (a, b) = (name.chars().count(), target.chars().count());
        if a.abs_diff(b) > limit {
            continue;
        }
        let d = edit_distance(&name.to_lowercase(), &lower);
        let better = match &best {
            Some((bd, _, _)) => d < *bd,
            None => true,
        };
        if d <= limit && better {
            let pkg = stdlib::suggest_import_for(name)?;
            best = Some((d, name, pkg));
        }
    }
    best.map(|(_, n, p)| (n, p))
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
                // Un nom peut être visible depuis plusieurs paquets : celui qui
                // le définit et tous ceux qui le ré-exportent. `MassValue` vient
                // d'`ISQBase`, mais `import ISQ::*;` suffit à l'atteindre — il
                // ne faut donc pas réclamer un import de plus.
                let exposing = stdlib::exposing_packages(&name);
                if !exposing.iter().any(|p| ctx.imported_roots.contains(*p)) {
                    out.push(
                        Diagnostic::warn(
                            "W301",
                            "unimported-standard-type",
                            q.span,
                            format!(
                                "`{name}` vient de la bibliothèque standard mais n'est pas importé"
                            ),
                        )
                        .hint(format!("ajoute `import {pkg}::*;` en tête du paquet")),
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

            // Nom retiré de la bibliothèque standard entre deux versions. Si le
            // modèle déclare viser la version antérieure, il a raison de
            // l'employer : on le signale, sans le rejeter.
            let rename = renames::lookup(&bad);
            if let Some(r) = rename {
                if opts.library == LibraryVersion::V2024_11 {
                    out.push(
                        Diagnostic::warn(
                            "W314",
                            "legacy-library-name",
                            span,
                            format!(
                                "`{}` appartient à la bibliothèque SysML v2 {} ; {}",
                                r.old,
                                renames::LEGACY_VERSION,
                                match r.new {
                                    Some((pkg, name)) =>
                                        format!("renommé `{pkg}::{name}` en {}", r.removed_in),
                                    None => format!("supprimé en {}", r.removed_in),
                                }
                            ),
                        )
                        .hint(rename_migration_hint(r)),
                    );
                    return;
                }
            }

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

            let mut msg = format!("{label} inconnu : `{bad}`");
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

            if let Some(r) = rename {
                // Correspondance exacte : elle prime sur les suggestions par
                // proximité orthographique qui suivent, qui ne sont que des
                // conjectures.
                d = d.hint(format!(
                    "`{}` (`{}`) a été {} dans la version {} du standard ; si ce modèle vise une version antérieure, utilise `--library-version {}`",
                    r.old,
                    r.old_pkg,
                    match r.new {
                        Some((pkg, name)) => format!("renommé `{pkg}::{name}`"),
                        None => "supprimé, sans remplacement direct,".to_string(),
                    },
                    r.removed_in,
                    renames::LEGACY_VERSION,
                ));
            } else if let Some(sugg) = nearest_name(m, scope, &bad) {
                d = d.hint(format!("vouliez-vous dire `{sugg}` ?"));
            } else if let Some((sugg, pkg)) = nearest_library_name(&bad) {
                d = d.hint(format!(
                    "`{bad}` n'existe pas dans la bibliothèque standard ; vouliez-vous dire `{pkg}::{sugg}` ?"
                ));
            } else if let Some(pkgs) = ctx.known_library_imports(&bad) {
                // Tous les imports du fichier sont des paquets standard dont on
                // connaît le contenu exact : on peut affirmer que le nom n'y est
                // pas, plutôt que de supposer un paquet non fourni.
                d = d.hint(format!(
                    "`{bad}` n'appartient à aucun paquet standard ; {pkgs}"
                ));
            } else if ctx.opaque_wildcard {
                d = d.hint(
                    "aucune déclaration correspondante dans les fichiers analysés ; \
                     ce nom vient peut-être d'un paquet importé non fourni"
                        .to_string(),
                );
            } else {
                d = d.hint(format!(
                    "déclare l'élément (ex. `part def {bad};`) ou ajoute l'import du paquet qui le contient"
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
                "`{name}` n'existe dans aucun supertype de `{owner_name}` : rien à redéfinir"
            ),
        )
        .hint(format!(
            "déclare `{name}` dans le supertype, ou remplace la redéfinition par une simple déclaration"
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
                .hint(
                    "déclare les extrémités : `end source : PortA;` et `end cible : PortB;`"
                        .to_string(),
                ),
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
                    format!("`{p}` n'est pas un mot-clé de la grammaire SysML v2"),
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

    // W313 — un import sans paquet englobant (racine du fichier) doit être
    // `private` dans la grammaire réelle (`checkImport` côté KerML).
    if s.kind == NodeKind::Import && s.parent == Some(0) {
        if let Some(vis) = s
            .prefixes
            .iter()
            .find(|p| matches!(p.as_str(), "public" | "protected"))
        {
            out.push(
                Diagnostic::warn(
                    "W313",
                    "public-import-at-top-level",
                    s.name_span,
                    format!(
                        "un `import` sans paquet englobant devrait être `private` (trouvé `{vis}`)"
                    ),
                )
                .hint(
                    "retire ce modificateur de visibilité, ou déplace l'import dans un `package`"
                        .to_string(),
                ),
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
                        format!("le nom de définition `{n}` devrait commencer par une majuscule"),
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
                        format!("le nom d'usage `{n}` devrait commencer par une minuscule"),
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
        && matches!(
            head_kw(&s.keyword),
            "part" | "item" | "attribute" | "port" | "ref"
        )
    {
        out.push(
            Diagnostic::warn(
                "W309",
                "untyped-usage",
                s.name_span,
                format!(
                    "l'usage `{}` n'est pas typé",
                    s.name.clone().unwrap_or_default()
                ),
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
        assert!(
            diags.is_empty(),
            "erreur de syntaxe inattendue : {:?}",
            diags.iter().map(|d| d.code).collect::<Vec<_>>()
        );
        let model = Model::build(&nodes);
        diags.extend(check(&model, opts));
        diags
    }

    fn analyze(src: &str) -> Vec<Diagnostic> {
        analyze_opts(src, &Options::default())
    }

    /// Options par défaut, mais en mode `--pedantic` (le seul réglage que la
    /// grande majorité des tests de règles a besoin de changer).
    fn pedantic() -> Options {
        Options {
            pedantic: true,
            ..Default::default()
        }
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
        assert_eq!(errors(&d), 0, "{d:?}");
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
        assert!(!has(&d, "E201"), "{d:?}");
    }

    #[test]
    fn e210_definition_typed_by_colon_instead_of_specializes() {
        let d = analyze("package P { part def Vehicle; part def Robot : Vehicle; }");
        assert!(has(&d, "E210"));
    }

    #[test]
    fn e210_does_not_fire_on_usage_typing() {
        let d = analyze("package P { part def Robot; part r : Robot; }");
        assert!(!has(&d, "E210"), "{d:?}");
    }

    #[test]
    fn e212_multiplicity_on_definition() {
        let d = analyze("package P { part def Robot[1]; }");
        assert!(has(&d, "E212"));
    }

    #[test]
    fn e213_flags_every_legacy_keyword() {
        for kw in [
            "block",
            "value",
            "class",
            "association",
            "stereotype",
            "property",
            "operation",
        ] {
            let src = format!("package P {{ {kw} X; }}");
            let d = analyze(&src);
            assert!(has(&d, "E213"), "mot-clé {kw} non signalé : {d:?}");
        }
    }

    #[test]
    fn e213_does_not_flag_current_keywords() {
        let d = analyze("package P { part def Robot; item def Fuel; port def P1; }");
        assert!(!has(&d, "E213"), "{d:?}");
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
        assert!(has(&d, "E214"), "{d:?}");
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
        assert!(!has(&d, "E214"), "{d:?}");
    }

    #[test]
    fn e215_end_outside_connection_interface_or_allocation() {
        let d = analyze("package P { part def Robot { end e1 : Robot; } }");
        assert!(has(&d, "E215"));
    }

    #[test]
    fn e215_does_not_fire_inside_a_connection() {
        let d = analyze("package P { part def X; connection def C { end a : X; end b : X; } }");
        assert!(!has(&d, "E215"), "{d:?}");
    }

    #[test]
    fn e216_subject_outside_requirement_case_or_concern() {
        let d = analyze("package P { part def Robot { subject r : Robot; } }");
        assert!(has(&d, "E216"));
    }

    #[test]
    fn e216_does_not_fire_inside_a_requirement() {
        let d = analyze("package P { part def Robot; requirement def R { subject r : Robot; } }");
        assert!(!has(&d, "E216"), "{d:?}");
    }

    #[test]
    fn e216_subject_allowed_inside_a_concern() {
        // ConcernDefinition spécialise RequirementDefinition dans le métamodèle.
        let d = analyze("package P { part def Robot; concern def C { subject r : Robot; } }");
        assert!(!has(&d, "E216"), "{d:?}");
    }

    #[test]
    fn e231_actor_outside_requirement_or_case() {
        let d = analyze("package P { part def Robot { actor a; } }");
        assert!(has(&d, "E231"));
    }

    #[test]
    fn e231_actor_allowed_inside_requirement_and_case() {
        let d = analyze("package P { requirement def R { actor a; } case def C { actor a; } }");
        assert!(!has(&d, "E231"), "{d:?}");
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
        assert!(has(&d, "E232"), "{d:?}");
    }

    #[test]
    fn e232_stakeholder_allowed_inside_requirement() {
        let d = analyze("package P { requirement def R { stakeholder s; } }");
        assert!(!has(&d, "E232"), "{d:?}");
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
        assert!(!has(&d, "E233"), "{d:?}");
    }

    #[test]
    fn e234_objective_outside_case() {
        let d = analyze("package P { requirement def R { objective o; } }");
        assert!(has(&d, "E234"), "{d:?}");
    }

    #[test]
    fn e234_objective_allowed_inside_case() {
        let d = analyze("package P { case def C { objective o; } }");
        assert!(!has(&d, "E234"), "{d:?}");
    }

    #[test]
    fn e235_frame_outside_requirement() {
        let d = analyze("package P { concern def C; part def Robot { frame concern c : C; } }");
        assert!(has(&d, "E235"), "{d:?}");
    }

    #[test]
    fn e235_frame_allowed_inside_requirement() {
        let d = analyze("package P { concern def C; requirement def R { frame concern c : C; } }");
        assert!(!has(&d, "E235"), "{d:?}");
    }

    #[test]
    fn e235_frame_allowed_inside_viewpoint() {
        let d = analyze("package P { concern def C; viewpoint def V { frame concern c : C; } }");
        assert!(!has(&d, "E235"), "{d:?}");
    }

    #[test]
    fn e236_verify_outside_any_case() {
        let d = analyze("package P { requirement def R; part def Robot { verify R; } }");
        assert!(has(&d, "E236"), "{d:?}");
    }

    #[test]
    fn e236_verify_inside_a_plain_requirement_is_not_enough() {
        // `UsageUtil.isLegalVerification` exige l'`objective` d'un cas de
        // vérification : une exigence ordinaire ne suffit pas, alors même que
        // la grammaire (`RequirementBodyItem`) accepte `verify` ici.
        let d = analyze("package P { requirement def R; requirement def Q { verify R; } }");
        assert!(has(&d, "E236"), "{d:?}");
    }

    #[test]
    fn e236_verify_inside_a_non_verification_objective_is_not_enough() {
        let d =
            analyze("package P { requirement def R; use case def U { objective { verify R; } } }");
        assert!(has(&d, "E236"), "{d:?}");
    }

    #[test]
    fn e236_verify_allowed_in_a_verification_case_objective() {
        let d = analyze(
            "package P { requirement def R; verification def V { objective { verify R; } } }",
        );
        assert!(!has(&d, "E236"), "{d:?}");
    }

    #[test]
    fn e236_verify_allowed_in_a_verification_usage_objective() {
        let d = analyze(
            "package P { requirement def R; verification def V; \
             verification v : V { objective { verify R; } } }",
        );
        assert!(!has(&d, "E236"), "{d:?}");
    }

    #[test]
    fn verify_target_is_resolved_not_declared() {
        // `verify X` référence une exigence : une cible inexistante doit être
        // signalée, et ne doit surtout pas déclarer un élément nommé `X`.
        let d = analyze(
            "package P { requirement def R; verification def V { objective { verify Absente; } } }",
        );
        assert!(has(&d, "E200"), "{d:?}");
    }

    #[test]
    fn verify_target_can_be_qualified_or_chained() {
        let d = analyze(
            "package P { use case U { objective obj; } \
             verification def V { objective { verify U.obj; } } }",
        );
        assert!(!has(&d, "E100"), "{d:?}");
        assert!(!has(&d, "E200"), "{d:?}");
    }

    #[test]
    fn e216_subject_allowed_inside_viewpoint() {
        let d = analyze("package P { part def Robot; viewpoint def V { subject r : Robot; } }");
        assert!(!has(&d, "E216"), "{d:?}");
    }

    #[test]
    fn e231_actor_allowed_inside_viewpoint() {
        let d = analyze("package P { viewpoint def V { actor a; } }");
        assert!(!has(&d, "E231"), "{d:?}");
    }

    #[test]
    fn e232_stakeholder_allowed_inside_viewpoint() {
        let d = analyze("package P { viewpoint def V { stakeholder s; } }");
        assert!(!has(&d, "E232"), "{d:?}");
    }

    #[test]
    fn e233_require_allowed_inside_viewpoint() {
        let d = analyze("package P { viewpoint def V { require constraint { true } } }");
        assert!(!has(&d, "E233"), "{d:?}");
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
        assert!(!has(&d, "E218"), "{d:?}");
    }

    #[test]
    fn e222_variant_outside_variation() {
        let d = analyze("package P { part def Robot { variant part v1 : Robot; } }");
        assert!(has(&d, "E222"));
    }

    #[test]
    fn e222_does_not_fire_inside_a_variation() {
        let d = analyze("package P { part def A; variation part def V { variant part v1 : A; } }");
        assert!(!has(&d, "E222"), "{d:?}");
    }

    #[test]
    fn e227_package_inside_definition() {
        let d = analyze("package P { part def Robot { package Sub { } } }");
        assert!(has(&d, "E227"));
    }

    #[test]
    fn e227_does_not_fire_for_nested_packages_at_package_level() {
        let d = analyze("package P { package Sub { } }");
        assert!(!has(&d, "E227"), "{d:?}");
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
        assert!(has(&d, "E230"), "{d:?}");
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
        assert!(!has(&d, "E230"), "{d:?}");
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
        assert!(has(&d, "E230"), "{d:?}");
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
        assert!(has(&d, "E230"), "{d:?}");
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
        assert!(has(&d, "E230"), "{d:?}");
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
        assert!(!has(&d, "E230"), "{d:?}");
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
    fn sibling_wildcard_import_does_not_soften_the_whole_file() {
        // `package A { import B::**; }` vise le paquet frère `B`. Résoudre cet
        // import depuis la racine le déclarait opaque, ce qui faisait basculer
        // *tous* les noms non résolus du fichier de E200 en W200.
        let d = analyze(
            "package Top { package A { private import B::**; part def X : Absent; } \
             package B { part def Y; } }",
        );
        assert!(has(&d, "E200"), "{d:?}");
        assert!(!has(&d, "W200"), "{d:?}");
    }

    #[test]
    fn unresolved_name_suggests_the_standard_library_spelling() {
        let d = analyze(
            "package P { private import ScalarValues::*; part def A { attribute s : Strng; } }",
        );
        let hint = d
            .iter()
            .find(|x| x.code == "E200")
            .and_then(|x| x.hint.clone())
            .unwrap_or_default();
        assert!(hint.contains("ScalarValues::String"), "{hint}");
    }

    #[test]
    fn renamed_library_name_is_an_error_that_names_its_replacement() {
        let d = analyze(
            "package P { private import Connections::*; part def A { attribute f : FlowConnection; } }",
        );
        assert!(has(&d, "E200"), "{d:?}");
        assert!(!has(&d, "W314"), "{d:?}");
        let hint = d
            .iter()
            .find(|x| x.code == "E200")
            .and_then(|x| x.hint.clone())
            .unwrap_or_default();
        assert!(hint.contains("Flows::Flow"), "{hint}");
        assert!(hint.contains("2025-02"), "{hint}");
        assert!(hint.contains("--library-version"), "{hint}");
    }

    #[test]
    fn renamed_library_name_is_only_a_warning_for_the_older_library() {
        let o = Options {
            library: LibraryVersion::V2024_11,
            ..Default::default()
        };
        let d = analyze_opts(
            "package P { private import Connections::*; part def A { attribute f : FlowConnection; } }",
            &o,
        );
        assert!(has(&d, "W314"), "{d:?}");
        assert!(!has(&d, "E200"), "{d:?}");
    }

    #[test]
    fn library_version_does_not_excuse_names_that_were_never_standard() {
        // Le drapeau ne reclasse que les noms connus de `renames.rs` : il ne
        // doit pas transformer toute faute de frappe en avertissement.
        let o = Options {
            library: LibraryVersion::V2024_11,
            ..Default::default()
        };
        let d = analyze_opts("package P { part def A { attribute f : PasUnType; } }", &o);
        assert!(has(&d, "E200"), "{d:?}");
        assert!(!has(&d, "W314"), "{d:?}");
    }

    #[test]
    fn unresolved_off_also_suppresses_the_legacy_warning() {
        let o = Options {
            library: LibraryVersion::V2024_11,
            unresolved: UnresolvedMode::Off,
            ..Default::default()
        };
        let d = analyze_opts(
            "package P { private import Connections::*; part def A { attribute f : FlowConnection; } }",
            &o,
        );
        assert!(!has(&d, "W314"), "{d:?}");
    }

    #[test]
    fn unresolved_name_states_that_known_imports_do_not_provide_it() {
        // Nom absent de la bibliothèque, trop loin de tout nom réel pour une
        // suggestion et hors table de renommages : comme tous les imports sont
        // des paquets standard au contenu connu, on peut l'affirmer.
        // (Ne pas prendre `FlowConnection` ici : il a désormais son propre
        // message, plus précis — voir `renamed_library_name_*`.)
        let d = analyze(
            "package P { private import Connections::*; part def A { attribute f : ZzzWidgetBus; } }",
        );
        let hint = d
            .iter()
            .find(|x| x.code == "E200")
            .and_then(|x| x.hint.clone())
            .unwrap_or_default();
        assert!(hint.contains("Connections"), "{hint}");
        assert!(hint.contains("aucun paquet standard"), "{hint}");
    }

    #[test]
    fn w301_does_not_fire_once_imported() {
        let d =
            analyze("package P { import ISQ::*; part def Robot { attribute masse : MassValue; } }");
        assert!(!has(&d, "W301"), "{d:?}");
    }

    #[test]
    fn w307_requirement_def_without_subject() {
        let o = pedantic();
        let d = analyze_opts(
            "package P { requirement def R { attribute x : Integer; } }",
            &o,
        );
        assert!(has(&d, "W307"));
    }

    #[test]
    fn w307_does_not_fire_once_a_subject_is_declared() {
        let o = pedantic();
        let d = analyze_opts(
            "package P { part def Robot; requirement def R { subject r : Robot; } }",
            &o,
        );
        assert!(!has(&d, "W307"), "{d:?}");
    }

    #[test]
    fn w307_requires_pedantic_to_fire() {
        let d = analyze("package P { requirement def R { attribute x : Integer; } }");
        assert!(!has(&d, "W307"), "{d:?}");
    }

    #[test]
    fn w311_readonly_flags_non_standard_keyword() {
        let o = pedantic();
        let d = analyze_opts(
            "package P { part def Robot { readonly attribute x : Integer; } }",
            &o,
        );
        assert!(has(&d, "W311"), "{d:?}");
    }

    #[test]
    fn w311_composite_flags_non_standard_keyword() {
        let o = pedantic();
        let d = analyze_opts(
            "package P { part def Robot; part def R { composite part r : Robot; } }",
            &o,
        );
        assert!(has(&d, "W311"), "{d:?}");
    }

    #[test]
    fn w311_portion_flags_non_standard_keyword() {
        let o = pedantic();
        let d = analyze_opts(
            "package P { part def Robot { portion attribute x : Integer; } }",
            &o,
        );
        assert!(has(&d, "W311"), "{d:?}");
    }

    #[test]
    fn w311_requires_pedantic_to_fire() {
        let d = analyze("package P { part def Robot { readonly attribute x : Integer; } }");
        assert!(!has(&d, "W311"), "{d:?}");
    }

    #[test]
    fn w311_does_not_flag_standard_modifiers() {
        let o = pedantic();
        let d = analyze_opts(
            "package P { part def Robot { derived attribute x : Integer; } }",
            &o,
        );
        assert!(!has(&d, "W311"), "{d:?}");
    }

    #[test]
    fn w312_flags_each_kerml_only_keyword() {
        let o = pedantic();
        let d = analyze_opts(
            "package P { feature x : Integer; namespace N { } specialization : Integer; subclassification : Integer; }",
            &o,
        );
        let count = d.iter().filter(|x| x.code == "W312").count();
        assert_eq!(count, 4, "{d:?}");
    }

    #[test]
    fn w312_requires_pedantic_to_fire() {
        let d = analyze("package P { feature x : Integer; }");
        assert!(!has(&d, "W312"), "{d:?}");
    }

    #[test]
    fn w312_does_not_flag_standard_keywords() {
        let o = pedantic();
        let d = analyze_opts("package P { part def Robot; }", &o);
        assert!(!has(&d, "W312"), "{d:?}");
    }

    #[test]
    fn w313_public_import_at_top_level() {
        let o = pedantic();
        let d = analyze_opts("package Q { part def X; } public import Q::X;", &o);
        assert!(has(&d, "W313"), "{d:?}");
    }

    #[test]
    fn w313_does_not_fire_on_private_top_level_import() {
        let o = pedantic();
        let d = analyze_opts("package Q { part def X; } private import Q::X;", &o);
        assert!(!has(&d, "W313"), "{d:?}");
    }

    #[test]
    fn w313_does_not_fire_on_nested_import() {
        let o = pedantic();
        let d = analyze_opts(
            "package Q { part def X; } package P { public import Q::X; }",
            &o,
        );
        assert!(!has(&d, "W313"), "{d:?}");
    }

    #[test]
    fn w313_requires_pedantic_to_fire() {
        let d = analyze("package Q { part def X; } public import Q::X;");
        assert!(!has(&d, "W313"), "{d:?}");
    }

    #[test]
    fn e201_shadows_inherited_member_without_redefines() {
        let src = r#"
            package P {
                part def Base { attribute x : Integer; }
                part def Derived :> Base { attribute x : Integer; }
            }
        "#;
        let d = analyze(src);
        assert!(has(&d, "E201"), "{d:?}");
    }

    #[test]
    fn e201_does_not_fire_when_member_uses_redefines() {
        let src = r#"
            package P {
                part def Base { attribute x : Integer; }
                part def Derived :> Base { attribute y :>> x; }
            }
        "#;
        let d = analyze(src);
        assert!(!has(&d, "E201"), "{d:?}");
    }

    #[test]
    fn e201_does_not_fire_when_no_name_collision() {
        let src = r#"
            package P {
                part def Base { attribute x : Integer; }
                part def Derived :> Base { attribute y : Integer; }
            }
        "#;
        let d = analyze(src);
        assert!(!has(&d, "E201"), "{d:?}");
    }

    #[test]
    fn w310_connection_def_without_any_end() {
        let d = analyze("package P { connection def C { attribute x : Integer; } }");
        assert!(has(&d, "W310"));
    }

    #[test]
    fn w310_does_not_fire_once_ends_are_declared() {
        let d = analyze("package P { part def X; connection def C { end a : X; end b : X; } }");
        assert!(!has(&d, "W310"), "{d:?}");
    }

    // -- Options CLI --------------------------------------------------------

    #[test]
    fn pedantic_rules_are_silent_by_default() {
        let src = "package Empty { }";
        assert!(!has(&analyze(src), "W302"));
    }

    #[test]
    fn pedantic_rules_activate_with_the_flag() {
        let o = pedantic();
        let d = analyze_opts("package Empty { }", &o);
        assert!(has(&d, "W302"));
    }

    #[test]
    fn pedantic_naming_convention_flags_lowercase_definitions_and_uppercase_usages() {
        let o = pedantic();
        let d = analyze_opts("package p { part def robot; part Robot : robot; }", &o);
        let count = d.iter().filter(|x| x.code == "W306").count();
        assert_eq!(count, 2, "{d:?}");
    }

    #[test]
    fn pedantic_untyped_usage_flags_missing_type() {
        let o = pedantic();
        let d = analyze_opts("package P { part def Robot { part sensor; } }", &o);
        assert!(has(&d, "W309"));
    }

    #[test]
    fn unresolved_mode_off_suppresses_both_error_and_warning_variants() {
        let o = Options {
            unresolved: UnresolvedMode::Off,
            ..Default::default()
        };
        let d = analyze_opts("package P { part def Robot { part s : Ghost; } }", &o);
        assert!(!has(&d, "E200"));
        assert!(!has(&d, "W200"));
    }

    #[test]
    fn unresolved_mode_warn_downgrades_error_to_warning() {
        let o = Options {
            unresolved: UnresolvedMode::Warn,
            ..Default::default()
        };
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

        let (toks_b, _) = Lexer::new(
            "package Consumer { part def Robot { part s : Shared::Sensor; } }",
            1,
        )
        .tokenize();
        let mut pb = Parser::new(toks_b, 500);
        let nodes_b = pb.parse_unit();
        assert!(pb.diags.is_empty());

        let mut all = nodes_a;
        all.extend(nodes_b);
        let model = Model::build(&all);
        let d = check(&model, &Options::default());
        assert!(d.is_empty(), "{d:?}");
    }
}
