#!/usr/bin/env python3
"""Génère `src/stdlib.rs` à partir de la bibliothèque standard SysML v2.

Deux modes :

    tools/gen-stdlib.py            emet src/stdlib.rs depuis spec/stdlib-*.{tsv,txt}
    tools/gen-stdlib.py --check    vérifie que src/stdlib.rs est à jour (CI)
    tools/gen-stdlib.py --fetch    reconstruit spec/stdlib-*.{tsv,txt} depuis GitHub

L'index vendu (`spec/`) rend la génération reproductible hors ligne ; seul
`--fetch` a besoin du réseau. Le binaire, lui, reste sans dépendance : tout
est figé dans un tableau statique.

Source : https://github.com/Systems-Modeling/SysML-v2-Release, `sysml.library/`.
Les fichiers `KerML.kerml` et `SysML.sysml` sont exclus de l'index des noms :
ils décrivent le métamodèle (les métaclasses) et non les éléments de
bibliothèque qu'un modèle utilisateur référence.
"""

import argparse
import glob
import json
import os
import re
import subprocess
import sys
import urllib.parse
import urllib.request

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPEC = os.path.join(ROOT, "spec")
INDEX_TSV = os.path.join(SPEC, "stdlib-index.tsv")
PKGS_TXT = os.path.join(SPEC, "stdlib-packages.txt")
REEXPORT_TSV = os.path.join(SPEC, "stdlib-reexports.tsv")
OUT_RS = os.path.join(ROOT, "src", "stdlib.rs")

REPO = "https://github.com/Systems-Modeling/SysML-v2-Release"
RAW = "https://raw.githubusercontent.com/Systems-Modeling/SysML-v2-Release/master/"
TREE = "https://api.github.com/repos/Systems-Modeling/SysML-v2-Release/git/trees/master?recursive=1"
META_FILES = ("KerML.kerml", "SysML.sysml")

DEF_KWS = (
    r"(?:def|datatype|class|struct|assoc(?:iation)?|type|function"
    r"|predicate|classifier|behavior|interaction|metaclass|enum)"
)
PKG_PAT = re.compile(
    r"\b(?:standard\s+)?(?:library\s+)?package\s+(?:<([A-Za-z_]\w*)>\s*)?([A-Za-z_]\w*)"
)
DEF_PAT = re.compile(r"\b" + DEF_KWS + r"\s+(?:<([A-Za-z_]\w*)>\s*)?([A-Za-z_]\w*)")
# `public import Q::*;` : Q est ré-exporté, donc importer P suffit à voir Q.
REEXPORT_PAT = re.compile(r"\bpublic\s+import\s+([A-Za-z_]\w*)\s*::\s*\*")


def strip_noise(src):
    """Retire commentaires et chaînes : leurs accolades fausseraient l'imbrication."""
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if c == "/" and i + 1 < n and src[i + 1] == "*":
            j = src.find("*/", i + 2)
            i = n if j < 0 else j + 2
            out.append(" ")
        elif c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            i = n if j < 0 else j
            out.append(" ")
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            i = j + 1
            out.append(" ")
        else:
            out.append(c)
            i += 1
    return "".join(out)


def scan(text, is_meta, index, packages, reexports):
    """Associe chaque définition au paquet qui la contient, par suivi d'accolades."""
    events = [(m.start(), "pkg", m.group(2), m.group(1)) for m in PKG_PAT.finditer(text)]
    events += [(m.start(), "def", m.group(2), m.group(1)) for m in DEF_PAT.finditer(text)]
    events += [(m.start(), "reexport", m.group(1), None) for m in REEXPORT_PAT.finditer(text)]
    events.sort()
    stack, depth, pos = [], 0, 0
    for off, kind, name, short in events:
        depth += text.count("{", pos, off) - text.count("}", pos, off)
        pos = off
        while stack and stack[-1][0] >= depth:
            stack.pop()
        if kind == "pkg":
            packages.add(name)
            if short:
                packages.add(short)
            stack.append((depth, name))
        elif kind == "reexport":
            if stack and not is_meta:
                reexports.add((stack[-1][1], name))
        elif stack and not is_meta:
            owner = stack[-1][1]
            index.setdefault(name, owner)
            if short:
                index.setdefault(short, owner)


def fetch():
    with urllib.request.urlopen(TREE, timeout=120) as r:
        tree = json.load(r)["tree"]
    paths = [
        e["path"]
        for e in tree
        if e["path"].startswith("sysml.library/")
        and e["path"].endswith((".sysml", ".kerml"))
    ]
    if not paths:
        sys.exit("aucun fichier de bibliothèque trouvé — l'arborescence du dépôt a changé ?")
    index, packages, reexports = {}, set(), set()
    for p in sorted(paths):
        with urllib.request.urlopen(RAW + urllib.parse.quote(p), timeout=120) as r:
            src = r.read().decode("utf-8")
        scan(strip_noise(src), os.path.basename(p) in META_FILES, index, packages, reexports)
    os.makedirs(SPEC, exist_ok=True)
    with open(INDEX_TSV, "w", encoding="utf-8") as f:
        f.write("# nom\tpaquet — index de la bibliothèque standard SysML v2.\n")
        f.write("# Généré par tools/gen-stdlib.py --fetch ; ne pas éditer à la main.\n")
        f.write("# Source : %s (sysml.library/, master)\n" % REPO)
        for name in sorted(index):
            f.write("%s\t%s\n" % (name, index[name]))
    with open(PKGS_TXT, "w", encoding="utf-8") as f:
        f.write("# Paquets de la bibliothèque standard SysML v2 / KerML.\n")
        f.write("# Généré par tools/gen-stdlib.py --fetch ; ne pas éditer à la main.\n")
        for p in sorted(packages):
            f.write(p + "\n")
    with open(REEXPORT_TSV, "w", encoding="utf-8") as f:
        f.write("# paquet\tpaquet ré-exporté (`public import X::*;`).\n")
        f.write("# Généré par tools/gen-stdlib.py --fetch ; ne pas éditer à la main.\n")
        for a, b in sorted(reexports):
            f.write("%s\t%s\n" % (a, b))
    print(
        "%d fichiers, %d noms, %d paquets, %d ré-exports"
        % (len(paths), len(index), len(packages), len(reexports))
    )


def load():
    def lines(path):
        with open(path, encoding="utf-8") as f:
            return [l.rstrip("\n") for l in f if l.strip() and not l.startswith("#")]

    defining = dict(tuple(l.split("\t", 1)) for l in lines(INDEX_TSV))
    packages = sorted(lines(PKGS_TXT))
    edges = [tuple(l.split("\t", 1)) for l in lines(REEXPORT_TSV)]

    # `public import Q::*;` dans P rend visible depuis P tout ce que Q expose —
    # transitivement. Sans cette fermeture, `import ISQ::*;` ne « couvrirait »
    # pas `MassValue`, qui est défini dans `ISQBase` et seulement ré-exporté
    # par `ISQ` : W301 se déclencherait alors sur un modèle pourtant correct.
    reexports = {}
    for a, b in edges:
        reexports.setdefault(a, set()).add(b)

    def exposed_pkgs(pkg, seen):
        if pkg in seen:
            return set()
        seen.add(pkg)
        out = {pkg}
        for nxt in reexports.get(pkg, ()):
            out |= exposed_pkgs(nxt, seen)
        return out

    # paquet -> tous les paquets qu'il expose (lui compris)
    closure = {p: exposed_pkgs(p, set()) for p in set(reexports) | set(defining.values())}
    # nom -> paquets depuis lesquels il est visible, paquet définissant en tête
    exposers = {}
    for name, home in defining.items():
        others = sorted(p for p, seen in closure.items() if home in seen and p != home)
        exposers[name] = [home] + others
    index = sorted(exposers.items())
    return index, packages


def render(index, packages):
    q = lambda s: '"%s"' % s
    out = []
    w = out.append
    w("//! Bibliothèque standard SysML v2 / KerML — **fichier généré**.")
    w("//!")
    w("//! Ne pas éditer à la main : régénérer avec `tools/gen-stdlib.py`")
    w("//! (et `--fetch` pour rafraîchir l'index vendu dans `spec/`).")
    w("//!")
    w("//! Le vérificateur ne charge pas la bibliothèque ; il sait seulement quels")
    w("//! noms lui appartiennent et dans quel paquet, afin de ne pas signaler comme")
    w("//! « inconnu » ce qui en vient légitimement — et, à l'inverse, de repérer un")
    w("//! nom qui *ressemble* à un type standard sans en être un.")
    w("//!")
    w("//! Source : %s" % REPO)
    w("")
    w("/// Paquets de la bibliothèque standard (racines d'import valides).")
    w("pub const LIBRARY_ROOTS: &[&str] = &[")
    for p in packages:
        w("    %s," % q(p))
    w("];")
    w("")
    w("/// `(nom, paquets)` pour chaque élément de la bibliothèque, **trié par nom**")
    w("/// (invariant requis par la recherche dichotomique ci-dessous).")
    w("///")
    w("/// Les paquets sont ceux depuis lesquels le nom est visible : celui qui le")
    w("/// définit en tête, puis ceux qui le ré-exportent via `public import`.")
    w("/// `MassValue` est ainsi joignable par `ISQBase` (sa définition) comme par")
    w("/// `ISQ` (qui ré-exporte `ISQBase`).")
    w("const LIBRARY_INDEX: &[(&str, &[&str])] = &[")
    for name, pkgs in index:
        w("    (%s, &[%s])," % (q(name), ", ".join(q(p) for p in pkgs)))
    w("];")
    w("")
    w("pub fn is_library_root(name: &str) -> bool {")
    w("    LIBRARY_ROOTS.binary_search(&name).is_ok()")
    w("}")
    w("")
    w("pub fn is_library_leaf(name: &str) -> bool {")
    w("    !exposing_packages(name).is_empty()")
    w("}")
    w("")
    w("/// Paquet standard qui **définit** `name`, s'il en existe un.")
    w("pub fn suggest_import_for(name: &str) -> Option<&'static str> {")
    w("    exposing_packages(name).first().copied()")
    w("}")
    w("")
    w("/// Tous les noms de la bibliothèque, triés — pour les suggestions par")
    w("/// proximité orthographique.")
    w("pub fn all_names() -> impl Iterator<Item = &'static &'static str> {")
    w("    LIBRARY_INDEX.iter().map(|(n, _)| n)")
    w("}")
    w("")
    w("/// Tous les paquets depuis lesquels `name` est visible : celui qui le définit,")
    w("/// puis ses ré-exportateurs. Vide si `name` n'appartient pas à la bibliothèque.")
    w("pub fn exposing_packages(name: &str) -> &'static [&'static str] {")
    w("    LIBRARY_INDEX")
    w("        .binary_search_by(|(n, _)| (*n).cmp(name))")
    w("        .map(|i| LIBRARY_INDEX[i].1)")
    w("        .unwrap_or(&[])")
    w("}")
    w("")
    w("#[cfg(test)]")
    w("mod tests {")
    w("    use super::*;")
    w("")
    w("    #[test]")
    w("    fn tables_are_sorted() {")
    w("        assert!(LIBRARY_ROOTS.windows(2).all(|w| w[0] < w[1]));")
    w("        assert!(LIBRARY_INDEX.windows(2).all(|w| w[0].0 < w[1].0));")
    w("    }")
    w("")
    w("    #[test]")
    w("    fn known_names_resolve_to_their_package() {")
    w('        assert_eq!(suggest_import_for("String"), Some("ScalarValues"));')
    w('        assert_eq!(suggest_import_for("Flow"), Some("Flows"));')
    w('        assert_eq!(suggest_import_for("Connection"), Some("Connections"));')
    w("    }")
    w("")
    w("    #[test]")
    w("    fn reexports_are_followed() {")
    w("        // `MassValue` est défini dans `ISQBase` et ré-exporté par `ISQ` :")
    w("        // les deux imports doivent le rendre visible.")
    w('        let pkgs = exposing_packages("MassValue");')
    w('        assert_eq!(pkgs.first().copied(), Some("ISQBase"));')
    w('        assert!(pkgs.contains(&"ISQ"), "{pkgs:?}");')
    w("    }")

    w("")
    w("    #[test]")
    w("    fn roots_are_recognised() {")
    w('        assert!(is_library_root("Connections"));')
    w('        assert!(is_library_root("ScalarValues"));')
    w("    }")
    w("")
    w("    #[test]")
    w("    fn flow_connection_is_not_a_standard_type() {")
    w("        // `FlowConnection` n'existe nulle part dans la bibliothèque : le type")
    w("        // de flux standard est `Flows::Flow`. Régression pour un faux négatif")
    w("        // plausible — le nom *semble* standard mais ne l'est pas.")
    w('        assert!(!is_library_leaf("FlowConnection"));')
    w('        assert!(is_library_leaf("Flow"));')
    w("    }")
    w("}")
    return "\n".join(out) + "\n"



def rustfmt(text):
    """Passe la sortie à `rustfmt` : `cargo fmt` ne doit pas rendre le fichier
    généré « périmé » au regard de `--check`."""
    try:
        p = subprocess.run(
            ["rustfmt", "--edition", "2021", "--emit", "stdout", "--quiet"],
            input=text,
            capture_output=True,
            text=True,
            check=True,
        )
    except FileNotFoundError:
        sys.exit("rustfmt introuvable : installe-le avec `rustup component add rustfmt`")
    except subprocess.CalledProcessError as e:
        sys.exit("rustfmt a échoué :\n%s" % e.stderr)
    return p.stdout


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--fetch", action="store_true", help="reconstruit l'index depuis GitHub")
    ap.add_argument("--check", action="store_true", help="échoue si src/stdlib.rs est périmé")
    args = ap.parse_args()
    if args.fetch:
        fetch()
        return
    text = rustfmt(render(*load()))
    if args.check:
        current = open(OUT_RS, encoding="utf-8").read() if os.path.exists(OUT_RS) else ""
        if current != text:
            sys.exit("src/stdlib.rs est périmé : relancer tools/gen-stdlib.py")
        print("src/stdlib.rs est à jour")
        return
    with open(OUT_RS, "w", encoding="utf-8") as f:
        f.write(text)
    print("écrit %s" % os.path.relpath(OUT_RS, ROOT))


if __name__ == "__main__":
    main()
