// Attendu : E230 (satisfy-target-not-requirement) — `Sensor` n'est pas une exigence
package P {
    part def Robot;
    part def Sensor;
    part r : Robot {
        satisfy Sensor by r;
    }
}
