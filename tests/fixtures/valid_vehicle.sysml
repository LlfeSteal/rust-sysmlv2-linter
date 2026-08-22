package VehicleModel {

    import ScalarValues::*;
    import ISQ::*;
    import SI::*;

    doc /* Modèle de démonstration : structure, exigence et machine à états. */

    item def Fuel {
        attribute densite : Real;
    }

    item def SignalDemarrage;

    port def FuelPort {
        in item fuel : Fuel;
    }

    part def Wheel {
        attribute diametre : ISQ::LengthValue;
    }

    part def Engine {
        attribute masse : ISQ::MassValue;
        port alimentation : FuelPort;
    }

    part def Vehicle {
        attribute masseTotale : ISQ::MassValue;
        part moteur : Engine[1];
        part roues : Wheel[4];
        port reservoir : ~FuelPort;
    }

    connection def FuelLine {
        end source : FuelPort;
        end cible : FuelPort;
    }

    requirement def MasseMaximale {
        doc /* La masse totale ne doit pas dépasser la limite contractuelle. */
        subject vehicule : Vehicle;
        attribute masseMax : ISQ::MassValue;
        require constraint { vehicule.masseTotale <= masseMax }
    }

    action def Demarrer;

    state def EtatVehicule {
        state arret;
        state roulant;
        transition t1 first arret accept SignalDemarrage then roulant;
    }

    part vehiculeConcret : Vehicle {
        satisfy requirement MasseMaximale by vehiculeConcret;
    }
}
