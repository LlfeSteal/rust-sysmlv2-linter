// Cas nominal : metadata def, annotation `#Meta`, doc et comment `about`.
package MetadataAndDocs {
    metadata def Critique;

    #Critique
    part def Robot {
        doc /* Le robot mobile principal du système. */
    }

    comment MaRemarque about Robot
    /* Ce composant est en cours de revue de conception. */
}
