//! Table des symboles : arène plate d'éléments, reliés par indices.
//! L'indice 0 est une portée globale synthétique contenant les racines de tous
//! les fichiers analysés.

use crate::ast::{Mult, Node, NodeKind, QName, Rel, RefUse};
use crate::diag::Span;

#[derive(Clone, Debug)]
pub struct Sym {
    pub id: usize,
    pub parent: Option<usize>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub qname: String,
    pub keyword: String,
    pub kind: NodeKind,
    pub is_def: bool,
    pub prefixes: Vec<String>,
    pub rels: Vec<Rel>,
    pub refs: Vec<RefUse>,
    pub mult: Option<Mult>,
    /// Imports déclarés directement dans cette portée.
    pub imports: Vec<QName>,
    pub children: Vec<usize>,
    pub has_body: bool,
    pub has_doc: bool,
    pub span: Span,
    pub name_span: Span,
    pub name_quoted: bool,
}

pub struct Model {
    pub syms: Vec<Sym>,
}

impl Model {
    pub fn global(&self) -> usize {
        0
    }

    pub fn child_named(&self, scope: usize, name: &str) -> Option<usize> {
        for &c in &self.syms[scope].children {
            let s = &self.syms[c];
            if s.name.as_deref() == Some(name) || s.short_name.as_deref() == Some(name) {
                return Some(c);
            }
        }
        None
    }

    pub fn display_name(&self, id: usize) -> String {
        let s = &self.syms[id];
        if !s.qname.is_empty() {
            s.qname.clone()
        } else {
            s.keyword.clone()
        }
    }

    pub fn build(roots: &[Node]) -> Model {
        let global = Sym {
            id: 0,
            parent: None,
            name: None,
            short_name: None,
            qname: String::new(),
            keyword: "<global>".to_string(),
            kind: NodeKind::Package,
            is_def: false,
            prefixes: Vec::new(),
            rels: Vec::new(),
            refs: Vec::new(),
            mult: None,
            imports: Vec::new(),
            children: Vec::new(),
            has_body: true,
            has_doc: false,
            span: Span::dummy(),
            name_span: Span::dummy(),
            name_quoted: false,
        };
        let mut model = Model { syms: vec![global] };
        let mut kids = Vec::new();
        for n in roots {
            let imported = collect_import(n);
            if let Some(q) = imported {
                model.syms[0].imports.push(q);
            }
            kids.push(add_node(&mut model, n, 0));
        }
        model.syms[0].children = kids;
        model
    }
}

fn collect_import(node: &Node) -> Option<QName> {
    if node.kind == NodeKind::Import {
        node.import_target.clone()
    } else {
        None
    }
}

fn add_node(model: &mut Model, node: &Node, parent: usize) -> usize {
    let id = model.syms.len();
    let name = node.name.as_ref().map(|n| n.text.clone());
    let short = node.short_name.as_ref().map(|n| n.text.clone());

    let parent_q = model.syms[parent].qname.clone();
    let own = name.clone().or_else(|| short.clone());
    let qname = match (&own, parent_q.is_empty()) {
        (Some(n), true) => n.clone(),
        (Some(n), false) => format!("{}::{}", parent_q, n),
        (None, _) => String::new(),
    };

    let has_doc = node.doc.is_some()
        || node
            .children
            .iter()
            .any(|c| c.kind == NodeKind::Doc && c.doc.is_some());

    let sym = Sym {
        id,
        parent: Some(parent),
        name,
        short_name: short,
        qname,
        keyword: node.keyword.clone(),
        kind: node.kind,
        is_def: node.is_def,
        prefixes: node.prefixes.iter().map(|p| p.text.clone()).collect(),
        rels: node.rels.clone(),
        refs: node.refs.clone(),
        mult: node.mult.clone(),
        imports: Vec::new(),
        children: Vec::new(),
        has_body: node.has_body,
        has_doc,
        span: node.span,
        name_span: node.name_span(),
        name_quoted: node.name_quoted,
    };
    model.syms.push(sym);

    let mut imports = Vec::new();
    for c in &node.children {
        if let Some(q) = collect_import(c) {
            imports.push(q);
        }
    }
    model.syms[id].imports = imports;

    let mut kids = Vec::new();
    for c in &node.children {
        kids.push(add_node(model, c, id));
    }
    model.syms[id].children = kids;

    id
}
