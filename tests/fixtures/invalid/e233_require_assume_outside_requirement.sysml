// Attendu : E233 (require-assume-outside-requirement) — `require` et
// `assume` ne sont valides que dans une `requirement def`.
package P {
    part def Robot {
        require constraint { true }
    }
}
