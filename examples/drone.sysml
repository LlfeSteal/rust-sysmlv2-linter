// Exemple de modèle SysML v2 (notation textuelle) à passer à sysml-check :
//
//     cargo run -- examples/drone.sysml
//     cargo run -- --format json examples/drone.sysml
//     cargo run -- --pedantic examples/drone.sysml
//
// Ce fichier est volontairement propre (0 diagnostic attendu en mode par
// défaut) : il sert de démonstration des principales constructions de la
// notation, pas de cas d'erreur. Pour voir le linter réagir, regarde plutôt
// les fixtures sous tests/fixtures/invalid/ et tests/fixtures/edge/, ou
// essaie --pedantic sur ce fichier (il déclenchera quelques avertissements
// de style, par ex. W309 sur un usage non typé si tu en ajoutes un).

package DroneDeLivraison {
    import ISQ::*;
    import ScalarValues::*;

    doc /* Modèle de démonstration d'un drone de livraison autonome :
           structure, ports, comportement, exigence et vérification. */

    // -- Types de valeur et éléments d'échange -----------------------------

    item def Electricite {
        attribute tension : ISQ::VoltageValue;
    }

    item def ColisPayload {
        attribute masse : ISQ::MassValue;
    }

    port def PortAlimentation {
        in item energie : Electricite;
    }

    port def PortSoute {
        inout item colis : ColisPayload;
    }

    // -- Structure -----------------------------------------------------------

    part def Batterie {
        attribute capacite : ISQ::EnergyValue;
        port sortie : PortAlimentation;
    }

    part def Moteur {
        attribute puissance : ISQ::PowerValue;
        port alimentation : ~PortAlimentation;
    }

    variation part def ChoixMoteur {
        variant part moteurStandard : Moteur;
        variant part moteurLonguePortee : Moteur;
    }

    part def Soute {
        port acces : PortSoute;
    }

    part def Drone {
        attribute masseAVide : ISQ::MassValue;
        part batterie : Batterie[1];
        part moteurs : Moteur[4];
        part soute : Soute[1];

        connect batterie.sortie to moteurs.alimentation;
    }

    // -- Comportement ---------------------------------------------------------

    item def SignalDecollage;
    item def SignalAtterrissage;

    action def Decoller;
    action def Atterrir;
    action def Livrer;

    state def EtatVol {
        entry action a1 : Decoller;
        state auSol;
        state enVol;
        state enLivraison;

        transition t1 first auSol accept SignalDecollage then enVol;
        transition t2 first enVol accept SignalAtterrissage then auSol;
        succession enVol then enLivraison;
    }

    // -- Exigences et vérification --------------------------------------------

    requirement def AutonomieMinimale {
        doc /* Le drone doit pouvoir parcourir au moins 20 km avec un colis
               de 2 kg sans recharge. */
        subject drone : Drone;
        attribute distanceMin : ISQ::LengthValue;
    }

    concern def ConcernAutonomie {
        subject drone : Drone;
        doc /* Préoccupation : autonomie réelle en conditions de vent. */
    }

    verification case def VerifieAutonomie {
        subject drone : Drone;
        doc /* Vérifie que l'exigence AutonomieMinimale est satisfaite en vol réel. */
    }

    part droneProduction : Drone {
        satisfy requirement AutonomieMinimale by droneProduction;
    }

    // -- Métadonnées ------------------------------------------------------------

    metadata def Critique;

    #Critique
    part def SystemeNavigation {
        doc /* Sous-système sûreté-critique : toute modification exige une revue. */
    }

    comment RevueRequise about SystemeNavigation
    /* Nécessite une revue de sûreté avant toute modification. */
}
