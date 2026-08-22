// Cas nominal : types de bibliothèque standard correctement importés (pas de W301).
package LibraryUsageWithImports {
    import ISQ::*;
    import ScalarValues::*;

    part def Engine {
        attribute masse : MassValue;
        attribute puissance : PowerValue;
        attribute nomModele : String;
    }
}
