// Attendu (--pedantic uniquement) : W311 (non-standard-keyword) — `readonly`,
// `composite` et `portion` n'existent pas dans la grammaire SysML v2.
package P {
    part def Robot {
        readonly attribute masse : Integer;
    }
    part def Vehicle {
        composite part moteur : Robot;
    }
    part def Snapshot {
        portion attribute etat : Integer;
    }
}
