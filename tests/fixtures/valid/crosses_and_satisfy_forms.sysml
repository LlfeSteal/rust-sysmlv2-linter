// Couvre `crosses` / `=>` (croisement d'extrémité d'association) ainsi que
// les trois formes normatives de `[assert] [not] satisfy` :
// `assert satisfy`, `not satisfy` et `assert not satisfy`.
package P {
    part def A;
    part def Link {
        part x : A;
        attribute crossedByOperator => x;
        attribute crossedByKeyword crosses x;
    }

    requirement def NeedsPower;
    requirement def NeedsCooling;
    requirement def NeedsShielding;
    part def Robot;

    part r : Robot {
        assert satisfy NeedsPower by r;
        not satisfy NeedsCooling by r;
        assert not satisfy NeedsShielding by r;
    }
}
