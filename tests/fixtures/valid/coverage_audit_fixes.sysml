// Couvre les correctifs High/Medium de l'audit de conformité SysML v2 :
// `constant`, l'usage de métadonnée autonome `@Meta;`, le modificateur
// d'état orthogonal `parallel`, le `subject`/`actor`/`stakeholder`/`require`
// désormais acceptés dans un `viewpoint def`, `rep` (TextualRepresentation)
// et l'import membre récursif sans joker `A::B::**`.
package P {
    import ISQ::**;
    import ScalarValues::*;

    metadata def Deprecated;

    part def Robot {
        constant attribute pi : Real = 3.14159;
        @Deprecated;

        rep behaviorDoc language "OCL" /* self.pi > 0 */;
    }

    state def Modes parallel {
        state idle;
        state active;
    }

    requirement def NeedsPower;

    viewpoint def SafetyViewpoint {
        subject robot : Robot;
        actor operator;
        stakeholder client;
        require constraint { true }
    }
}
