// Attendu : E200 par défaut, W314 (legacy-library-name) avec
// `--library-version 2024-11`.
//
// `Connections::FlowConnection` était le type de flux standard jusqu'à la
// version 2024-11 du standard ; il a été renommé `Flows::Flow` en 2025-02.
// Un modèle écrit pour l'ancienne version n'est donc pas fautif — il vise une
// autre version de la bibliothèque.
package LegacyFlowType {
    private import Connections::*;

    item def Position;

    action def Produce { out item p : Position; }
    action def Consume { in item p : Position; }

    part def System {
        action producer : Produce;
        action consumer : Consume;

        flow f : FlowConnection from producer.p to consumer.p;
    }
}
