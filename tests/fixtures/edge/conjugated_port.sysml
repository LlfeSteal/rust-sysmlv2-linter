// Cas limite : conjugaison de port (`~`) sur un usage.
package ConjugatedPort {
    port def FuelPort;
    part def Vehicle {
        port reservoir : ~FuelPort;
    }
}
