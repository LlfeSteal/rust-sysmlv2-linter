// Attendu : E236 (verify-outside-verification-objective) — `verify` n'est
// valide que dans l'`objective` d'un cas de vérification.
//
// `UsageUtil.isLegalVerification` exige que le propriétaire soit l'objectif
// d'un `VerificationCase{Definition,Usage}`. Une exigence ordinaire ne suffit
// donc pas, même si la grammaire (`RequirementBodyItem`) accepte `verify` là.
package P {
    requirement def SafetyMargin;

    requirement def Enclosing {
        verify SafetyMargin;
    }
}
