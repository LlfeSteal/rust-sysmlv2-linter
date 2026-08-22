// `analysis def` et `verification def` sans le suffixe `case` sont la forme
// canonique de la grammaire SysML v2 (`case` est optionnel après ces deux
// mots-clés) : `analysis case def` / `verification case def` restent aussi
// acceptés, voir `requirements_and_verification.sysml`.
package P {
    part def Robot;

    analysis def PerformanceStudy {
        subject robot : Robot;
        objective assessPerformance;
    }

    verification def PowerCheck {
        subject robot : Robot;
        objective checkPower;
    }
}
