// Attendu : E235 (frame-outside-requirement) — `frame` n'est valide que
// dans une exigence (`requirement`, `concern`, `viewpoint`).
package P {
    concern def C;
    part def Robot {
        frame concern c : C;
    }
}
