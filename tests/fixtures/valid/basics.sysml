// Cas nominal : paquet, imports, définitions et usages typés, multiplicités, ports.
package Basics {
    import ISQ::*;

    doc /* Modèle minimal couvrant les constructions de base de SysML v2. */

    item def Fuel {
        attribute densite : ISQ::MassValue;
    }

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
        part moteur : Engine[1];
        part roues : Wheel[4];
    }
}
