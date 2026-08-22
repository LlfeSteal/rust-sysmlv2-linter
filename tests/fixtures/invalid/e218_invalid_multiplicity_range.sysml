// Attendu : E218 (invalid-multiplicity-range) — borne inférieure > borne supérieure
package P {
    part def Sensor;
    part def Robot {
        part sensors : Sensor[5..2];
    }
}
