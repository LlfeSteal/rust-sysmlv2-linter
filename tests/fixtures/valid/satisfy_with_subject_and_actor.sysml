// Cas nominal : un `SatisfyRequirementUsage` *est* une `RequirementUsage`
// (métamodèle), il peut donc porter `subject`, `actor`, `stakeholder`,
// `require` et `frame` — ce qu'une heuristique sur le mot-clé « satisfy »
// (qui ne contient ni « requirement » ni « case ») manquait.
package SatisfyScope {
    part def Sys;
    part def Operator;
    concern def Safety;
    requirement def R;

    part sys : Sys;

    satisfy R by sys {
        subject s : Sys;
        actor a : Operator;
        stakeholder st : Operator;
        frame concern c : Safety;
    }
}
