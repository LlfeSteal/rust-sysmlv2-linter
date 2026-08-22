// Cas nominal : point de variation et variantes associées.
package VariationAndVariants {
    part def Battery;
    part def SmallBattery :> Battery;
    part def LargeBattery :> Battery;

    variation part def BatteryChoice {
        variant part small : SmallBattery;
        variant part large : LargeBattery;
    }
}
