// Attendu : E214 (redefines-target-not-inherited) — `z` n'existe dans aucun supertype de Derived
package P {
    part def Base {
        attribute x : Integer;
    }
    part def Derived :> Base {
        attribute z : Integer;
        attribute y :>> z;
    }
}
