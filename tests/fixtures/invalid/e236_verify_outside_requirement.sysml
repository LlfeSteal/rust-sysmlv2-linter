// Attendu : E236 (verify-outside-requirement) — `verify` n'est valide que
// dans une exigence (`requirement`).
package P {
    requirement def SafetyMargin;

    part def Robot {
        verify SafetyMargin;
    }
}
