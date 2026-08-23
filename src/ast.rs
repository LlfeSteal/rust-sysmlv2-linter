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
