// Cas nominal : exigence avec sujet et contrainte, satisfaction, cas de vérification.
package RequirementsAndVerification {
    import ISQ::*;

    part def Vehicle {
        attribute masseTotale : ISQ::MassValue;
    }

    requirement def MasseMaximale {
        doc /* La masse totale ne doit pas dépasser la limite contractuelle. */
        subject vehicule : Vehicle;
        attribute masseMax : ISQ::MassValue;
        require constraint { vehicule.masseTotale <= masseMax }
    }

    concern def ConcernMasse {
        subject vehicule : Vehicle;
        doc /* Préoccupation autour de la masse embarquée. */
    }

    verification case def VerifieMasse {
        subject vehicule : Vehicle;
        doc /* Vérifie que l'exigence MasseMaximale est satisfaite. */
    }

    part vehiculeConcret : Vehicle {
        satisfy requirement MasseMaximale by vehiculeConcret;
    }
}
