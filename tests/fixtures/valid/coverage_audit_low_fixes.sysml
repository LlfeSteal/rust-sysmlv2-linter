// Couvre les correctifs Low restants de l'audit de conformité SysML v2 :
// `locale` sur `doc`/`comment`, `succession flow`, `expose`, et un import
// avec condition de filtre `import P[cond];` (déjà géré correctement par
// le squelette de multiplicité, qui accepte n'importe quel contenu entre
// crochets — ce fichier le prouve désormais explicitement).
package Sensors {
    part def Sensor;
}

package P {
    import Sensors[isCompatible];

    doc D locale "en" /* Robot documentation. */;

    part def A;
    part def Link {
        port pa : A;
        port pb : A;
        succession flow pa to pb;

        comment about pa locale "fr" /* Point de départ du flux. */;
    }

    expose Sensors::Sensor;
}
