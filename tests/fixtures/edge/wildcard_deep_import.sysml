// Cas limite : import joker profond (`::**`) vers un paquet inconnu du vérificateur.
// Ne doit pas faire planter l'outil ; les noms non résolus du fichier deviennent tolérants.
package WildcardDeepImport {
    import ExternalCatalog::*::**;

    part def Robot {
        part gadget : SomeExternalType;
    }
}
