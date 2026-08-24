#!/usr/bin/env python3
"""Génère `src/spec.rs` : la hiérarchie des métaclasses SysML v2.

    tools/gen-spec-table.py            emet src/spec.rs depuis spec/metamodel-supertypes.tsv
    tools/gen-spec-table.py --check    vérifie que src/spec.rs est à jour (CI)
    tools/gen-spec-table.py --fetch    reconstruit le TSV depuis SysML.json (OMG)

Pourquoi la hiérarchie ? Les règles de portée du validateur de référence sont
écrites en `instanceof` :

    // validateSubjectMembershipOwningType
    if (!(owningType instanceof RequirementDefinition || ... instanceof CaseUsage))

Un `ConcernDefinition` est un `RequirementDefinition`, un
`VerificationCaseDefinition` est un `CaseDefinition` : sans la clôture
transitive on ne peut pas reproduire ces règles, et on retombe sur des
heuristiques de sous-chaînes qui se trompent (`satisfy` ne contient ni
« requirement » ni « case », alors qu'un `SatisfyRequirementUsage` *est* une
`RequirementUsage`).

Le schéma encode le sous-typage par `anyOf: [ {soi}, {$ref: SousType}, ... ]`.

Sources :
  - métamodèle   https://www.omg.org/spec/SysML/20250201/SysML.json
  - règles       org.omg.sysml.xtext/.../validation/SysMLValidator.xtend
                 org.omg.sysml.logic/.../util/UsageUtil.java
"""

import argparse
import json
import os
import subprocess
import sys
import urllib.request

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPEC = os.path.join(ROOT, "spec")
TSV = os.path.join(SPEC, "metamodel-supertypes.tsv")
OUT_RS = os.path.join(ROOT, "src", "spec.rs")
URL = "https://www.omg.org/spec/SysML/20250201/SysML.json"


def fetch():
    with urllib.request.urlopen(URL, timeout=180) as r:
        defs = json.load(r)["$defs"]
    # anyOf: [ {soi}, {$ref: SousType}, ... ]  ->  arêtes parent -> sous-types
    subtypes = {}
    for name, node in defs.items():
        kids = [v["$ref"].rsplit("/", 1)[-1] for v in node.get("anyOf", []) if "$ref" in v]
        if kids:
            subtypes[name] = kids

    supers = {name: set() for name in defs}

    def walk(base, node, seen):
        for kid in subtypes.get(node, ()):
            if kid in seen:
                continue
            seen.add(kid)
            supers[kid].add(base)
            walk(base, kid, seen)

    for name in defs:
        walk(name, name, set())

    os.makedirs(SPEC, exist_ok=True)
    with open(TSV, "w", encoding="utf-8") as f:
        f.write("# métaclasse\tsuper-classes transitives (séparées par des virgules)\n")
        f.write("# Généré par tools/gen-spec-table.py --fetch ; ne pas éditer à la main.\n")
        f.write("# Source : %s\n" % URL)
        for name in sorted(defs):
            f.write("%s\t%s\n" % (name, ",".join(sorted(supers[name]))))
    print("%d métaclasses" % len(defs))


def load():
    rows = []
    with open(TSV, encoding="utf-8") as f:
        for line in f:
            if not line.strip() or line.startswith("#"):
                continue
            name, _, sup = line.rstrip("\n").partition("\t")
            rows.append((name, [s for s in sup.split(",") if s]))
    return sorted(rows)


def render(rows):
    q = lambda s: '"%s"' % s
    out = []
    w = out.append
    w("//! Hiérarchie des métaclasses SysML v2 — **fichier généré**.")
    w("//!")
    w("//! Ne pas éditer à la main : régénérer avec `tools/gen-spec-table.py`")
    w("//! (et `--fetch` pour rafraîchir l'index vendu dans `spec/`).")
    w("//!")
    w("//! Sert à reproduire fidèlement les règles de portée du validateur de")
    w("//! référence, qui sont écrites en `instanceof` et dépendent donc du")
    w("//! sous-typage : un `ConcernDefinition` *est* un `RequirementDefinition`,")
    w("//! un `SatisfyRequirementUsage` *est* une `RequirementUsage`.")
    w("//!")
    w("//! Source : %s" % URL)
    w("")
    w("/// `(métaclasse, super-classes transitives)`, **trié par nom**")
    w("/// (invariant requis par la recherche dichotomique ci-dessous).")
    w("const SUPERTYPES: &[(&str, &[&str])] = &[")
    for name, sup in rows:
        w("    (%s, &[%s])," % (q(name), ", ".join(q(s) for s in sup)))
    w("];")
    w("")
    w("/// Vrai si `mc` est `base` ou l'un de ses sous-types — l'équivalent du")
    w("/// `instanceof` sur lequel reposent les règles du validateur de référence.")
    w("pub fn is_kind_of(mc: &str, base: &str) -> bool {")
    w("    mc == base || supertypes(mc).contains(&base)")
    w("}")
    w("")
    w("/// Vrai si `mc` est (un sous-type de) l'une des `bases`.")
    w("pub fn is_any_kind_of(mc: &str, bases: &[&str]) -> bool {")
    w("    bases.iter().any(|b| is_kind_of(mc, b))")
    w("}")
    w("")
    w("/// Super-classes transitives de `mc` ; vide si la métaclasse est inconnue.")
    w("pub fn supertypes(mc: &str) -> &'static [&'static str] {")
    w("    SUPERTYPES")
    w("        .binary_search_by(|(n, _)| (*n).cmp(mc))")
    w("        .map(|i| SUPERTYPES[i].1)")
    w("        .unwrap_or(&[])")
    w("}")
    w("")
    w("#[cfg(test)]")
    w("mod tests {")
    w("    use super::*;")
    w("")
    w("    #[test]")
    w("    fn table_is_sorted() {")
    w("        assert!(SUPERTYPES.windows(2).all(|w| w[0].0 < w[1].0));")
    w("    }")
    w("")
    w("    #[test]")
    w("    fn instanceof_follows_the_metamodel() {")
    w("        // Les quatre cas que les anciennes heuristiques de sous-chaînes")
    w("        // manquaient ou inventaient.")
    w('        assert!(is_kind_of("ConcernDefinition", "RequirementDefinition"));')
    w('        assert!(is_kind_of("ViewpointDefinition", "RequirementDefinition"));')
    w('        assert!(is_kind_of("SatisfyRequirementUsage", "RequirementUsage"));')
    w('        assert!(is_kind_of("VerificationCaseDefinition", "CaseDefinition"));')
    w('        assert!(is_kind_of("IncludeUseCaseUsage", "CaseUsage"));')
    w("    }")
    w("")
    w("    #[test]")
    w("    fn unrelated_types_are_not_confused() {")
    w('        assert!(!is_kind_of("PartDefinition", "RequirementDefinition"));')
    w('        assert!(!is_kind_of("RequirementUsage", "CaseUsage"));')
    w('        assert!(!is_kind_of("ActionDefinition", "CaseDefinition"));')
    w('        assert!(supertypes("PasUneMétaclasse").is_empty());')
    w("    }")
    w("")
    w("    #[test]")
    w("    fn a_type_is_an_instance_of_itself() {")
    w('        assert!(is_kind_of("CaseUsage", "CaseUsage"));')
    w('        assert!(is_any_kind_of("ConcernUsage", &["CaseUsage", "RequirementUsage"]));')
    w('        assert!(!is_any_kind_of("PartUsage", &["CaseUsage", "RequirementUsage"]));')
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
    ap.add_argument("--fetch", action="store_true", help="reconstruit le TSV depuis l'OMG")
    ap.add_argument("--check", action="store_true", help="échoue si src/spec.rs est périmé")
    args = ap.parse_args()
    if args.fetch:
        fetch()
        return
    text = rustfmt(render(load()))
    if args.check:
        current = open(OUT_RS, encoding="utf-8").read() if os.path.exists(OUT_RS) else ""
        if current != text:
            sys.exit("src/spec.rs est périmé : relancer tools/gen-spec-table.py")
        print("src/spec.rs est à jour")
        return
    with open(OUT_RS, "w", encoding="utf-8") as f:
        f.write(text)
    print("écrit %s" % os.path.relpath(OUT_RS, ROOT))


if __name__ == "__main__":
    main()
