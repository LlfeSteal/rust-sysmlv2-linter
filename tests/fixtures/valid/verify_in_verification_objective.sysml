// Cas nominal : `verify` dans l'`objective` d'un cas de vérification, sous ses
// deux formes (définition et usage). La cible est une *référence* à une
// exigence existante, éventuellement qualifiée ou chaînée — et non la
// déclaration d'un nouvel élément.
package VerifyInObjective {
    requirement def SafetyMargin;

    use case uc {
        objective ucObjective;
    }

    verification def VerifySafetyMargin {
        objective {
            verify SafetyMargin;
        }
    }

    verification v : VerifySafetyMargin {
        objective {
            verify VerifyInObjective::SafetyMargin;
        }
    }

    verification def VerifyUseCase {
        objective {
            verify uc.ucObjective;
        }
    }
}
