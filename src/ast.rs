//! AST « uniforme » : SysML v2 étant très régulier (définitions / usages / relations),
//! un seul type de nœud suffit, la spécialisation passant par `keyword`.

use crate::diag::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Package,
    Def,
    Usage,
    Import,
    Alias,
    Doc,
    Comment,
    Relationship,
    Unknown,
}

impl NodeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            NodeKind::Package => "package",
            NodeKind::Def => "definition",
            NodeKind::Usage => "usage",
            NodeKind::Import => "import",
            NodeKind::Alias => "alias",
            NodeKind::Doc => "doc",
            NodeKind::Comment => "comment",
            NodeKind::Relationship => "relationship",
            NodeKind::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Spanned {
    pub text: String,
    pub span: Span,
}

impl Spanned {
    pub fn new(text: String, span: Span) -> Spanned {
        Spanned { text, span }
    }
}

/// Nom qualifié : `A::B::c.d`, avec séparateurs mémorisés (`::` vs `.`).
///
/// Simplification connue : la grammaire réelle réserve `::` à la
/// qualification d'espace de noms et `.` au chaînage de fonctionnalités
/// (feature chaining) dans les expressions — deux rôles distincts que ce
/// type traite comme interchangeables (voir `tests/fixtures/edge/dot_and_double_colon_are_conflated.sysml`).
#[derive(Clone, Debug)]
pub struct QName {
    pub parts: Vec<Spanned>,
    /// Séparateur précédant `parts[i+1]` : ':' pour `::`, '.' pour `.`
    pub seps: Vec<char>,
    /// `Some("*")` ou `Some("*::**")` pour les imports avec joker.
    pub wildcard: Option<String>,
    pub span: Span,
}

impl QName {
    pub fn text(&self) -> String {
        let mut s = String::new();
        for (i, p) in self.parts.iter().enumerate() {
            if i > 0 {
                let sep = self.seps.get(i - 1).copied().unwrap_or(':');
                s.push_str(if sep == '.' { "." } else { "::" });
            }
            s.push_str(&p.text);
        }
        if let Some(w) = &self.wildcard {
            s.push_str("::");
            s.push_str(w);
        }
        s
    }

    pub fn root(&self) -> &str {
        self.parts.first().map(|p| p.text.as_str()).unwrap_or("")
    }

    pub fn last(&self) -> &str {
        self.parts.last().map(|p| p.text.as_str()).unwrap_or("")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelKind {
    /// `:` — typage d'un usage
    TypedBy,
    /// `:>` sur une définition, ou `specializes`
    Specializes,
    /// `:>` sur un usage, ou `subsets`
    Subsets,
    /// `:>>` ou `redefines`
    Redefines,
    /// `::>` ou `references`
    References,
    /// `~` — conjugaison de port
    Conjugates,
    /// `=>` ou `crosses` — croisement d'extrémité d'association
    Crosses,
}

impl RelKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RelKind::TypedBy => "typedBy",
            RelKind::Specializes => "specializes",
            RelKind::Subsets => "subsets",
            RelKind::Redefines => "redefines",
            RelKind::References => "references",
            RelKind::Conjugates => "conjugates",
            RelKind::Crosses => "crosses",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rel {
    pub kind: RelKind,
    /// Le lexème réellement écrit (`:`, `:>`, `subsets`, ...), utile pour les messages.
    pub token: String,
    pub op_span: Span,
    pub target: QName,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefCtx {
    Value,
    ConnectEnd,
    FlowEnd,
    BindEnd,
    TransitionSource,
    TransitionTarget,
    SatisfyTarget,
    SatisfyBy,
    VerifyTarget,
    AllocateSource,
    AllocateTarget,
    About,
    ImportTarget,
    AliasTarget,
    ExposeTarget,
    Annotation,
    Other,
}

impl RefCtx {
    pub fn as_str(self) -> &'static str {
        match self {
            RefCtx::Value => "value",
            RefCtx::ConnectEnd => "connectEnd",
            RefCtx::FlowEnd => "flowEnd",
            RefCtx::BindEnd => "bindEnd",
            RefCtx::TransitionSource => "transitionSource",
            RefCtx::TransitionTarget => "transitionTarget",
            RefCtx::SatisfyTarget => "satisfyTarget",
            RefCtx::SatisfyBy => "satisfyBy",
            RefCtx::VerifyTarget => "verifyTarget",
            RefCtx::AllocateSource => "allocateSource",
            RefCtx::AllocateTarget => "allocateTarget",
            RefCtx::About => "about",
            RefCtx::ImportTarget => "importTarget",
            RefCtx::AliasTarget => "aliasTarget",
            RefCtx::ExposeTarget => "exposeTarget",
            RefCtx::Annotation => "annotation",
            RefCtx::Other => "other",
        }
    }
}

#[derive(Clone, Debug)]
pub struct RefUse {
    pub qname: QName,
    pub ctx: RefCtx,
}

#[derive(Clone, Debug)]
pub struct Mult {
    pub lower: Option<Spanned>,
    pub upper: Spanned,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub keyword: String,
    pub keyword_span: Span,
    pub is_def: bool,
    pub prefixes: Vec<Spanned>,
    pub name: Option<Spanned>,
    pub name_quoted: bool,
    pub short_name: Option<Spanned>,
    pub rels: Vec<Rel>,
    pub mult: Option<Mult>,
    pub refs: Vec<RefUse>,
    pub value: Option<Spanned>,
    pub doc: Option<String>,
    pub import_target: Option<QName>,
    pub children: Vec<Node>,
    pub has_body: bool,
    pub span: Span,
}

impl Node {
    pub fn new(span: Span) -> Node {
        Node {
            kind: NodeKind::Unknown,
            keyword: String::new(),
            keyword_span: span,
            is_def: false,
            prefixes: Vec::new(),
            name: None,
            name_quoted: false,
            short_name: None,
            rels: Vec::new(),
            mult: None,
            refs: Vec::new(),
            value: None,
            doc: None,
            import_target: None,
            children: Vec::new(),
            has_body: false,
            span,
        }
    }

    pub fn has_prefix(&self, p: &str) -> bool {
        self.prefixes.iter().any(|x| x.text == p)
    }

    pub fn name_span(&self) -> Span {
        match &self.name {
            Some(n) => n.span,
            None => self.keyword_span,
        }
    }
}

/// Métaclasse SysML v2 correspondant à un mot-clé de déclaration.
///
/// **Écrit à la main** — c'est le seul pont que le métamodèle ne peut pas
/// fournir : `SysML.json` décrit la syntaxe *abstraite* (les métaclasses),
/// jamais la notation textuelle. Les mots-clés viennent donc de la grammaire
/// de référence, `SysML.xtext` (`UseCaseKeyword: 'use' 'case';`,
/// `VerificationCaseDefKeyword: 'verification' 'def';`, ...).
///
/// `None` signifie « ce mot-clé ne déclare pas un type porteur de membres »
/// (`doc`, `import`, `alias`, une relation, ...) — pas « mot-clé invalide ».
/// Les règles de portée s'appuient sur cette distinction : un parent sans
/// métaclasse ne déclenche aucun diagnostic, il faut donc que la table couvre
/// réellement tous les contextes déclaratifs.
///
/// Chaque nom produit ici est vérifié contre le métamodèle par le test
/// `every_metaclass_exists` (`src/spec.rs` fait foi).
pub fn metaclass_for(kw: &str, is_def: bool) -> Option<&'static str> {
    // `assert satisfy` / `not satisfy` restent des `satisfy` : c'est le dernier
    // mot qui porte le sens (même convention que `classify` ci-dessous).
    let last = kw.split(' ').next_back().unwrap_or("");
    let base = kw.strip_suffix(" def").unwrap_or(kw);

    // Racines « Definition / Usage » : même mot-clé, le suffixe `def` décide.
    let stem = match base {
        "part" => "Part",
        "item" => "Item",
        "attribute" => "Attribute",
        "port" => "Port",
        "action" => "Action",
        "state" => "State",
        "constraint" => "Constraint",
        "calc" => "Calculation",
        "connection" => "Connection",
        "interface" => "Interface",
        "allocation" => "Allocation",
        "flow" => "Flow",
        "occurrence" => "Occurrence",
        "metadata" => "Metadata",
        "enum" => "Enumeration",
        "view" => "View",
        "rendering" => "Rendering",
        "requirement" => "Requirement",
        "concern" => "Concern",
        "viewpoint" => "Viewpoint",
        "case" => "Case",
        "use case" => "UseCase",
        // `analysis def` / `verification def` : la grammaire n'écrit pas
        // « case » (`VerificationCaseDefKeyword: 'verification' 'def';`), mais
        // la métaclasse, elle, s'appelle bien `VerificationCaseDefinition`.
        "analysis" | "analysis case" => "AnalysisCase",
        "verification" | "verification case" => "VerificationCase",
        _ => {
            return match last {
                // Un `SatisfyRequirementUsage` *est* une `RequirementUsage` :
                // il peut donc porter subject/actor/stakeholder/require/...
                "satisfy" => Some("SatisfyRequirementUsage"),
                // Le corps d'un `objective` *est* une `RequirementUsage`
                // (`ObjectiveMembership::ownedObjectiveRequirement`), ce qui
                // rend `verify` légal à l'intérieur — et lui seul.
                "objective" => Some("RequirementUsage"),
                "ref" => Some("ReferenceUsage"),
                "include" => Some("IncludeUseCaseUsage"),
                "perform" => Some("PerformActionUsage"),
                "exhibit" => Some("ExhibitStateUsage"),
                "event" => Some("EventOccurrenceUsage"),
                "transition" => Some("TransitionUsage"),
                _ => None,
            };
        }
    };

    // `Definition` et `Usage` existent pour chacune des racines ci-dessus.
    Some(match (stem, is_def) {
        ("Part", true) => "PartDefinition",
        ("Part", false) => "PartUsage",
        ("Item", true) => "ItemDefinition",
        ("Item", false) => "ItemUsage",
        ("Attribute", true) => "AttributeDefinition",
        ("Attribute", false) => "AttributeUsage",
        ("Port", true) => "PortDefinition",
        ("Port", false) => "PortUsage",
        ("Action", true) => "ActionDefinition",
        ("Action", false) => "ActionUsage",
        ("State", true) => "StateDefinition",
        ("State", false) => "StateUsage",
        ("Constraint", true) => "ConstraintDefinition",
        ("Constraint", false) => "ConstraintUsage",
        ("Calculation", true) => "CalculationDefinition",
        ("Calculation", false) => "CalculationUsage",
        ("Connection", true) => "ConnectionDefinition",
        ("Connection", false) => "ConnectionUsage",
        ("Interface", true) => "InterfaceDefinition",
        ("Interface", false) => "InterfaceUsage",
        ("Allocation", true) => "AllocationDefinition",
        ("Allocation", false) => "AllocationUsage",
        ("Flow", true) => "FlowDefinition",
        ("Flow", false) => "FlowUsage",
        ("Occurrence", true) => "OccurrenceDefinition",
        ("Occurrence", false) => "OccurrenceUsage",
        ("Metadata", true) => "MetadataDefinition",
        ("Metadata", false) => "MetadataUsage",
        ("Enumeration", true) => "EnumerationDefinition",
        ("Enumeration", false) => "EnumerationUsage",
        ("View", true) => "ViewDefinition",
        ("View", false) => "ViewUsage",
        ("Rendering", true) => "RenderingDefinition",
        ("Rendering", false) => "RenderingUsage",
        ("Requirement", true) => "RequirementDefinition",
        ("Requirement", false) => "RequirementUsage",
        ("Concern", true) => "ConcernDefinition",
        ("Concern", false) => "ConcernUsage",
        ("Viewpoint", true) => "ViewpointDefinition",
        ("Viewpoint", false) => "ViewpointUsage",
        ("Case", true) => "CaseDefinition",
        ("Case", false) => "CaseUsage",
        ("UseCase", true) => "UseCaseDefinition",
        ("UseCase", false) => "UseCaseUsage",
        ("AnalysisCase", true) => "AnalysisCaseDefinition",
        ("AnalysisCase", false) => "AnalysisCaseUsage",
        ("VerificationCase", true) => "VerificationCaseDefinition",
        ("VerificationCase", false) => "VerificationCaseUsage",
        _ => unreachable!("racine de métaclasse non couverte : {stem}"),
    })
}

pub fn classify(kw: &str, is_def: bool) -> NodeKind {
    if kw == "package" || kw == "library package" {
        return NodeKind::Package;
    }
    match kw {
        "import" => return NodeKind::Import,
        "alias" => return NodeKind::Alias,
        "doc" => return NodeKind::Doc,
        "rep" => return NodeKind::Doc,
        "comment" => return NodeKind::Comment,
        _ => {}
    }
    if is_def {
        return NodeKind::Def;
    }
    // Le dernier mot : `assert satisfy` doit rester une relation `satisfy`,
    // pas un `assert` générique (voir `parser::is_relationship_kw`).
    let last = kw.split(' ').next_back().unwrap_or("");
    match last {
        "connect" | "bind" | "binding" | "flow" | "succession" | "transition" | "satisfy"
        | "allocate" | "dependency" => NodeKind::Relationship,
        _ => NodeKind::Usage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::MEMBER_KWS;
    use crate::spec;

    /// Aucun nom de métaclasse inventé : tout ce que `metaclass_for` renvoie
    /// doit exister dans le métamodèle vendu (`spec/metamodel-supertypes.tsv`).
    #[test]
    fn every_metaclass_exists() {
        for kw in MEMBER_KWS {
            for is_def in [false, true] {
                if let Some(mc) = metaclass_for(kw, is_def) {
                    assert!(
                        !spec::supertypes(mc).is_empty() || mc == "Element",
                        "`{kw}` (is_def={is_def}) donne `{mc}`, absent du métamodèle"
                    );
                }
            }
        }
        for kw in [
            "use case",
            "analysis case",
            "verification case",
            "assert satisfy",
        ] {
            for is_def in [false, true] {
                if let Some(mc) = metaclass_for(kw, is_def) {
                    assert!(
                        !spec::supertypes(mc).is_empty(),
                        "`{kw}` donne `{mc}`, absent du métamodèle"
                    );
                }
            }
        }
    }

    /// Les contextes dont dépendent les règles de portée doivent être reconnus :
    /// un trou ici rend une règle silencieuse au lieu de fausse.
    #[test]
    fn scope_bearing_keywords_are_mapped() {
        for kw in [
            "part",
            "item",
            "action",
            "state",
            "requirement",
            "concern",
            "viewpoint",
            "case",
            "use case",
            "analysis",
            "verification",
            "constraint",
            "port",
            "attribute",
        ] {
            for is_def in [false, true] {
                assert!(
                    metaclass_for(kw, is_def).is_some(),
                    "`{kw}` (is_def={is_def}) n'a pas de métaclasse"
                );
            }
        }
        assert_eq!(
            metaclass_for("satisfy", false),
            Some("SatisfyRequirementUsage")
        );
        assert_eq!(
            metaclass_for("assert satisfy", false),
            Some("SatisfyRequirementUsage")
        );
        assert_eq!(metaclass_for("objective", false), Some("RequirementUsage"));
    }

    /// `doc`, `import`, ... ne portent pas de membres : pas de métaclasse, donc
    /// pas de diagnostic de portée inventé à leur sujet.
    #[test]
    fn non_declarative_keywords_have_no_metaclass() {
        for kw in ["doc", "import", "alias", "comment", "package"] {
            assert_eq!(metaclass_for(kw, false), None, "`{kw}`");
        }
    }
}
