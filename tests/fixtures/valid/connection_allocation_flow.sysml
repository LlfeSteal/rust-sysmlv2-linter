// Cas nominal : connection def avec extrémités, allocation, flux entre parts.
package ConnectionAllocationFlow {
    port def FuelPort;

    part def Engine {
        port alimentation : FuelPort;
    }

    part def Tank {
        port sortie : FuelPort;
    }

    connection def FuelLine {
        end source : FuelPort;
        end cible : FuelPort;
    }

    part def Vehicle {
        part moteur : Engine;
        part reservoir : Tank;
        connect reservoir.sortie to moteur.alimentation;
        flow reservoir.sortie to moteur.alimentation;
    }

    action def Consommer;
    action def Fournir;

    allocation def AllocationEnergie {
        allocate Fournir to Consommer;
    }
}
