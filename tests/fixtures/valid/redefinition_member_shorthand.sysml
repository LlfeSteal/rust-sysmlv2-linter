// Cas nominal : un membre peut commencer directement par sa partie de
// spécialisation, sans mot-clé — une `FeatureUsage` anonyme. C'est la forme
// courante pour affecter une valeur à une caractéristique héritée, notamment
// dans le corps d'une métadonnée.
package RedefinitionShorthand {
    private import ScalarValues::*;

    metadata def SourceArtifact {
        attribute path : String;
        attribute kind : String;
    }

    part def Base {
        attribute x : Integer;
        attribute y : Integer;
    }

    part def Derived :> Base {
        @SourceArtifact {
            :>> path = "backlog/NAV-042.md";
            :>> kind = "customer-need";
        }

        :>> x = 1;
        ::> y = 2;
        redefines y;
        subsets x;
        :> Base;
    }
}
