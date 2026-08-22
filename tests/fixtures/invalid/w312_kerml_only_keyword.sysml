// Attendu (--pedantic uniquement) : W312 (kerml-only-keyword) — `feature`,
// `namespace`, `specialization` et `subclassification` n'apparaissent
// jamais dans la grammaire de surface SysML v2 (`SysML.xtext`) : ce sont
// des productions KerML-only.
package P {
    feature x : Integer;
    namespace N {
    }
    specialization : Integer;
    subclassification : Integer;
}
