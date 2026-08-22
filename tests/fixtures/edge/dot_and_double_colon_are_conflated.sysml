// Documente une simplification connue : la grammaire SysML v2 réserve `::`
// à la qualification d'espace de noms et `.` au chaînage de fonctionnalités
// (feature chaining) dans les expressions — deux rôles distincts. `QName`
// (voir `ast.rs`) traite les deux comme des séparateurs interchangeables :
// `A::B.Robot` et `A.B::Robot` résolvent donc tous deux ici, ce qui est plus
// permissif que la grammaire réelle. Voir la section « hors périmètre » du
// plan de mise en conformité SysML v2.
package P {
    package A {
        package B {
            part def Robot;
        }
    }
    part r1 : A::B.Robot;
    part r2 : A.B::Robot;
}
