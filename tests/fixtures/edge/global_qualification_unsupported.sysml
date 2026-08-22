// Documente une limitation connue : la grammaire SysML v2 admet `$::Nom`
// pour forcer une résolution depuis la racine globale (`GlobalNamespace`).
// `sysml-check` ne reconnaît pas `$` comme caractère de nom et rapporte
// E003 (unexpected-character) au lieu d'accepter la construction — voir la
// section « hors périmètre » du plan de mise en conformité SysML v2.
package P {
    part def Robot;
    part r : $::P::Robot;
}
