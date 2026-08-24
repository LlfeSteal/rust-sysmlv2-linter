//! Éléments de bibliothèque renommés ou supprimés d'une version du standard à
//! l'autre.
//!
//! **Écrit à la main**, contrairement à `spec.rs` et `stdlib.rs` : un diff entre
//! deux versions dit ce qui a *disparu*, jamais ce qui le *remplace*. La
//! correspondance ci-dessous se lit dans les chaînes de spécialisation, pas
//! dans un diff :
//!
//! ```text
//! 2024-11  MessageConnection        :> BinaryConnection, Transfer, Action
//!          FlowConnection           :> MessageConnection, FlowTransfer
//!          SuccessionFlowConnection :> FlowConnection, FlowTransferBefore
//! 2025-02  Message                  :> MessageAction, Transfer
//!          Flow                     :> Message, FlowTransfer
//!          SuccessionFlow           :> Flow, FlowTransferBefore
//! ```
//!
//! Le diff de la *Systems Library* entre les étiquettes `2024-11` et `2025-02`
//! de `Systems-Modeling/SysML-v2-Release` donne exactement quatre retraits :
//! ceux listés ici. La table sert uniquement à *expliquer* un nom non résolu —
//! ces noms restent absents de `stdlib.rs`, sans quoi ils se résoudraient
//! silencieusement.

/// Un élément retiré de la bibliothèque standard à une version donnée.
pub struct Rename {
    pub old: &'static str,
    /// Paquet qui le définissait avant le retrait.
    pub old_pkg: &'static str,
    /// `(paquet, nom)` de remplacement ; `None` si supprimé sans équivalent.
    pub new: Option<(&'static str, &'static str)>,
    /// Version du standard où le retrait prend effet.
    pub removed_in: &'static str,
}

const RENAMES: &[Rename] = &[
    Rename {
        old: "FlowConnection",
        old_pkg: "Connections",
        new: Some(("Flows", "Flow")),
        removed_in: "2025-02",
    },
    Rename {
        old: "MessageConnection",
        old_pkg: "Connections",
        new: Some(("Flows", "Message")),
        removed_in: "2025-02",
    },
    Rename {
        old: "SuccessionFlowConnection",
        old_pkg: "Connections",
        new: Some(("Flows", "SuccessionFlow")),
        removed_in: "2025-02",
    },
    Rename {
        // Fusionné dans `ForLoopAction`, qui existait déjà : pas de
        // correspondance 1 pour 1 à proposer.
        old: "ForLoopActionBase",
        old_pkg: "Actions",
        new: None,
        removed_in: "2025-02",
    },
];

/// L'entrée décrivant `name`, si ce nom a été retiré de la bibliothèque.
pub fn lookup(name: &str) -> Option<&'static Rename> {
    RENAMES.iter().find(|r| r.old == name)
}

/// Version de bibliothèque la plus ancienne connue de cette table — celle où
/// tous les noms ci-dessus étaient encore valides.
pub const LEGACY_VERSION: &str = "2024-11";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib;

    /// Garde-fou contre la dérive : `stdlib.rs` est régénéré depuis la
    /// bibliothèque publiée, cette table-ci ne l'est pas. Si une régénération
    /// réintroduisait un ancien nom ou déplaçait un remplacement, ce test le
    /// signale au lieu de laisser les deux diverger en silence.
    #[test]
    fn table_agrees_with_the_current_library() {
        for r in RENAMES {
            assert!(
                !stdlib::is_library_leaf(r.old),
                "`{}` est de nouveau dans la bibliothèque : la table est périmée",
                r.old
            );
            if let Some((pkg, name)) = r.new {
                assert_eq!(
                    stdlib::suggest_import_for(name),
                    Some(pkg),
                    "le remplacement `{pkg}::{name}` ne correspond plus à la bibliothèque"
                );
            }
        }
    }

    #[test]
    fn lookup_finds_renamed_and_removed_names() {
        let r = lookup("FlowConnection").expect("FlowConnection");
        assert_eq!(r.old_pkg, "Connections");
        assert_eq!(r.new, Some(("Flows", "Flow")));
        assert_eq!(r.removed_in, "2025-02");

        assert!(matches!(lookup("ForLoopActionBase"), Some(r) if r.new.is_none()));
        assert!(lookup("Flow").is_none());
        assert!(lookup("PasUnNom").is_none());
    }
}
