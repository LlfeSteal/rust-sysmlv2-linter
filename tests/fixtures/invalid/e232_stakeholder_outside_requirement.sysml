// Attendu : E232 (stakeholder-outside-requirement) — `stakeholder` n'est
// valide que dans une `requirement def` (contrairement à `actor`, pas dans
// un `case def`).
package P {
    case def C {
        stakeholder client;
    }
}
