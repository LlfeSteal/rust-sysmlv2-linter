// Cas nominal : `redefines` vers un membre réellement hérité du supertype.
package RedefinitionValid {
    part def Base {
        attribute x : Integer;
    }
    part def Derived :> Base {
        attribute y :>> x;
    }
}
