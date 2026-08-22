// Cas limites de multiplicité : bornée, ouverte, exacte, étoile seule.
package MultiplicityVariants {
    part def Sensor;
    part def Robot {
        part a : Sensor[0..1];
        part b : Sensor[1..*];
        part c : Sensor[3];
        part d : Sensor[*];
    }
}
