// Attendu (--pedantic uniquement) : W313 (public-import-at-top-level) —
// un `import` sans paquet englobant doit être `private` dans la grammaire
// réelle (`checkImport`), pas `public`/`protected`.
package Q {
    part def X;
}

public import Q::X;
