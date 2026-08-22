// Attendu : E201 (shadows-inherited-member) — `Derived` redéclare `x` sans
// `:>>` (redefines) alors que `Base` (sa supertype) déclare déjà `x`.
package P {
    part def Base {
        attribute x : Integer;
    }
    part def Derived :> Base {
        attribute x : Integer;
    }
}
