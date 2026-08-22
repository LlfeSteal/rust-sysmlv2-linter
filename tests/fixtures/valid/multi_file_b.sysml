// Cas nominal (multi-fichiers, 2/2) : référence croisée vers Shared::Sensor du fichier a.
package Consumer {
    part def Robot {
        part s : Shared::Sensor;
    }
}
