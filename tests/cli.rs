//! Tests d'intégration boîte noire : on invoque le binaire compilé (via la
//! variable `CARGO_BIN_EXE_<nom>` fournie par Cargo, sans dépendance externe)
//! sur les fixtures de `tests/fixtures/` et on vérifie le code de sortie et le
//! contenu JSON produit.
//!
//! Organisation des fixtures :
//! - `valid/`   : cas nominaux, aucun diagnostic d'erreur attendu.
//! - `invalid/` : un fichier par règle, qui doit déclencher exactement le
//!   code indiqué dans son nom (`e210_...` -> "E210").
//! - `edge/`    : cas limites (fichier vide, CRLF, Unicode, imports jokers
//!   profonds, multiplicités dégénérées, ...).

use std::process::{Command, Output};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_sysml-check")
}

fn run(args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .output()
        .expect("échec du lancement de sysml-check")
}

fn json(path: &str) -> (String, i32) {
    let out = run(&["--format", "json", path]);
    let code = out.status.code().unwrap_or(-1);
    (String::from_utf8_lossy(&out.stdout).into_owned(), code)
}

fn json_with(args: &[&str], path: &str) -> (String, i32) {
    let mut full: Vec<&str> = vec!["--format", "json"];
    full.extend_from_slice(args);
    full.push(path);
    let out = run(&full);
    let code = out.status.code().unwrap_or(-1);
    (String::from_utf8_lossy(&out.stdout).into_owned(), code)
}

fn has_code(out: &str, code: &str) -> bool {
    out.contains(&format!("\"code\": \"{code}\""))
}

fn assert_ok(path: &str) {
    let (out, code) = json(path);
    assert!(
        out.contains("\"ok\": true"),
        "attendu ok:true pour {path} — sortie :\n{out}"
    );
    assert_eq!(code, 0, "code de sortie attendu 0 pour {path}");
}

fn assert_has(path: &str, code: &str) {
    let (out, exit) = json(path);
    assert!(
        has_code(&out, code),
        "attendu {code} dans la sortie de {path} — sortie :\n{out}"
    );
    assert_eq!(
        exit, 1,
        "les diagnostics d'erreur donnent un code de sortie 1 ({path})"
    );
}

fn assert_has_pedantic(path: &str, code: &str) {
    let (out, _exit) = json_with(&["--pedantic"], path);
    assert!(
        has_code(&out, code),
        "attendu {code} (avec --pedantic) dans la sortie de {path} — sortie :\n{out}"
    );
}

fn assert_absent_without_pedantic(path: &str, code: &str) {
    let (out, _exit) = json(path);
    assert!(
        !has_code(&out, code),
        "{code} ne devrait apparaître qu'avec --pedantic pour {path} — sortie :\n{out}"
    );
}

const VALID_DIR: &str = "tests/fixtures/valid/";
const INVALID_DIR: &str = "tests/fixtures/invalid/";
const EDGE_DIR: &str = "tests/fixtures/edge/";

// --------------------------------------------------------------------------
// Cas nominaux : aucun diagnostic d'erreur
// --------------------------------------------------------------------------

#[test]
fn valid_basics_is_clean() {
    assert_ok(&format!("{VALID_DIR}basics.sysml"));
}

#[test]
fn valid_state_machine_is_clean() {
    assert_ok(&format!("{VALID_DIR}state_machine.sysml"));
}

#[test]
fn valid_requirements_and_verification_is_clean() {
    assert_ok(&format!("{VALID_DIR}requirements_and_verification.sysml"));
}

#[test]
fn valid_connection_allocation_flow_is_clean() {
    assert_ok(&format!("{VALID_DIR}connection_allocation_flow.sysml"));
}

#[test]
fn valid_variation_and_variants_is_clean() {
    assert_ok(&format!("{VALID_DIR}variation_and_variants.sysml"));
}

#[test]
fn valid_metadata_and_docs_is_clean() {
    assert_ok(&format!("{VALID_DIR}metadata_and_docs.sysml"));
}

#[test]
fn valid_quoted_names_and_alias_is_clean() {
    assert_ok(&format!("{VALID_DIR}quoted_names_and_alias.sysml"));
}

#[test]
fn valid_library_usage_with_imports_is_clean() {
    assert_ok(&format!("{VALID_DIR}library_usage_with_imports.sysml"));
}

#[test]
fn valid_redefinition_is_clean() {
    assert_ok(&format!("{VALID_DIR}redefinition_valid.sysml"));
}

#[test]
fn valid_analysis_and_verification_bare_keywords_is_clean() {
    assert_ok(&format!(
        "{VALID_DIR}analysis_and_verification_bare_keywords.sysml"
    ));
}

#[test]
fn valid_crosses_and_satisfy_forms_is_clean() {
    assert_ok(&format!("{VALID_DIR}crosses_and_satisfy_forms.sysml"));
}

#[test]
fn valid_coverage_audit_fixes_is_clean() {
    assert_ok(&format!("{VALID_DIR}coverage_audit_fixes.sysml"));
}

#[test]
fn valid_coverage_audit_low_fixes_is_clean() {
    assert_ok(&format!("{VALID_DIR}coverage_audit_low_fixes.sysml"));
}

#[test]
fn valid_redefinition_member_shorthand_is_clean() {
    assert_ok(&format!("{VALID_DIR}redefinition_member_shorthand.sysml"));
}

#[test]
fn valid_verify_in_verification_objective_is_clean() {
    assert_ok(&format!(
        "{VALID_DIR}verify_in_verification_objective.sysml"
    ));
}

#[test]
fn valid_satisfy_with_subject_and_actor_is_clean() {
    assert_ok(&format!("{VALID_DIR}satisfy_with_subject_and_actor.sysml"));
}

#[test]
fn example_drone_is_clean() {
    assert_ok("examples/drone.sysml");
}

#[test]
fn shipped_vehicle_fixture_is_clean() {
    assert_ok("tests/fixtures/valid_vehicle.sysml");
}

/// La résolution de noms doit fonctionner à travers plusieurs fichiers passés
/// ensemble en argument : `multi_file_b.sysml` seul échoue (E200 sur
/// `Shared::Sensor`), mais réussit une fois combiné avec `multi_file_a.sysml`
/// qui déclare ce paquet.
#[test]
fn cross_file_name_resolution() {
    let a = format!("{VALID_DIR}multi_file_a.sysml");
    let b = format!("{VALID_DIR}multi_file_b.sysml");

    let (out_alone, exit_alone) = json(&b);
    assert!(has_code(&out_alone, "E200"), "{}", out_alone);
    assert_eq!(exit_alone, 1);

    let out = run(&["--format", "json", &a, &b]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"ok\": true"), "{}", stdout);
    assert!(stdout.contains("\"files\": 2"), "{}", stdout);
    assert_eq!(out.status.code(), Some(0));
}

// --------------------------------------------------------------------------
// Un test par règle du catalogue (voir tests/fixtures/invalid/)
// --------------------------------------------------------------------------

#[test]
fn e001_unterminated_block_comment() {
    assert_has(
        &format!("{INVALID_DIR}e001_unterminated_block_comment.sysml"),
        "E001",
    );
}

#[test]
fn e002_unterminated_string() {
    assert_has(
        &format!("{INVALID_DIR}e002_unterminated_string.sysml"),
        "E002",
    );
}

#[test]
fn e003_unexpected_character_stereotype() {
    assert_has(
        &format!("{INVALID_DIR}e003_unexpected_character.sysml"),
        "E003",
    );
}

#[test]
fn e100_member_must_start_with_keyword() {
    assert_has(
        &format!("{INVALID_DIR}e100_member_must_start_with_keyword.sysml"),
        "E100",
    );
}

#[test]
fn e100_unexpected_token() {
    assert_has(&format!("{INVALID_DIR}e100_unexpected_token.sysml"), "E100");
}

#[test]
fn e101_expected_token() {
    assert_has(&format!("{INVALID_DIR}e101_expected_token.sysml"), "E101");
}

#[test]
fn e102_unclosed_brace() {
    assert_has(&format!("{INVALID_DIR}e102_unclosed_brace.sysml"), "E102");
}

#[test]
fn e103_missing_semicolon() {
    assert_has(
        &format!("{INVALID_DIR}e103_missing_semicolon.sysml"),
        "E103",
    );
}

#[test]
fn e104_doc_without_body() {
    assert_has(&format!("{INVALID_DIR}e104_doc_without_body.sysml"), "E104");
}

#[test]
fn e200_unresolved_name() {
    assert_has(&format!("{INVALID_DIR}e200_unresolved_name.sysml"), "E200");
}

#[test]
fn e201_duplicate_name() {
    assert_has(&format!("{INVALID_DIR}e201_duplicate_name.sysml"), "E201");
}

#[test]
fn e201_shadows_inherited_member() {
    assert_has(
        &format!("{INVALID_DIR}e201_shadows_inherited_member.sysml"),
        "E201",
    );
}

#[test]
fn e210_def_typed_by_colon() {
    assert_has(
        &format!("{INVALID_DIR}e210_def_typed_by_colon.sysml"),
        "E210",
    );
}

#[test]
fn e212_multiplicity_on_definition() {
    assert_has(
        &format!("{INVALID_DIR}e212_multiplicity_on_definition.sysml"),
        "E212",
    );
}

#[test]
fn e213_legacy_keyword() {
    let path = format!("{INVALID_DIR}e213_legacy_keyword.sysml");
    let (out, _exit) = json(&path);
    // Les sept mots-clés hérités du fichier doivent chacun être signalés.
    let count = out.matches("\"code\": \"E213\"").count();
    assert_eq!(count, 7, "sortie :\n{out}");
}

#[test]
fn e214_redefines_target_not_inherited() {
    assert_has(
        &format!("{INVALID_DIR}e214_redefines_target_not_inherited.sysml"),
        "E214",
    );
}

#[test]
fn e215_end_outside_connection() {
    assert_has(
        &format!("{INVALID_DIR}e215_end_outside_connection.sysml"),
        "E215",
    );
}

#[test]
fn e216_subject_outside_requirement() {
    assert_has(
        &format!("{INVALID_DIR}e216_subject_outside_requirement.sysml"),
        "E216",
    );
}

#[test]
fn e218_invalid_multiplicity_range() {
    assert_has(
        &format!("{INVALID_DIR}e218_invalid_multiplicity_range.sysml"),
        "E218",
    );
}

#[test]
fn e222_variant_outside_variation() {
    assert_has(
        &format!("{INVALID_DIR}e222_variant_outside_variation.sysml"),
        "E222",
    );
}

#[test]
fn e225_reserved_word_as_name() {
    assert_has(
        &format!("{INVALID_DIR}e225_reserved_word_as_name.sysml"),
        "E225",
    );
}

#[test]
fn e227_package_inside_definition() {
    assert_has(
        &format!("{INVALID_DIR}e227_package_inside_definition.sysml"),
        "E227",
    );
}

#[test]
fn e230_satisfy_target_not_requirement() {
    assert_has(
        &format!("{INVALID_DIR}e230_satisfy_target_not_requirement.sysml"),
        "E230",
    );
}

#[test]
fn e231_actor_outside_requirement_or_case() {
    assert_has(
        &format!("{INVALID_DIR}e231_actor_outside_requirement_or_case.sysml"),
        "E231",
    );
}

#[test]
fn e232_stakeholder_outside_requirement() {
    assert_has(
        &format!("{INVALID_DIR}e232_stakeholder_outside_requirement.sysml"),
        "E232",
    );
}

#[test]
fn e233_require_assume_outside_requirement() {
    assert_has(
        &format!("{INVALID_DIR}e233_require_assume_outside_requirement.sysml"),
        "E233",
    );
}

#[test]
fn e234_objective_outside_case() {
    assert_has(
        &format!("{INVALID_DIR}e234_objective_outside_case.sysml"),
        "E234",
    );
}

#[test]
fn e235_frame_outside_requirement() {
    assert_has(
        &format!("{INVALID_DIR}e235_frame_outside_requirement.sysml"),
        "E235",
    );
}

#[test]
fn e236_verify_outside_verification_objective() {
    assert_has(
        &format!("{INVALID_DIR}e236_verify_outside_verification_objective.sysml"),
        "E236",
    );
}

#[test]
fn w200_unresolved_name_in_value_expression_is_a_warning_not_an_error() {
    let path = format!("{INVALID_DIR}w200_unresolved_name_in_value_expression.sysml");
    let (out, exit) = json(&path);
    assert!(has_code(&out, "W200"), "{}", out);
    assert!(out.contains("\"ok\": true"), "{}", out);
    assert_eq!(exit, 0, "un avertissement seul ne bloque pas la sortie");
}

#[test]
fn w301_unimported_standard_type() {
    let path = format!("{INVALID_DIR}w301_unimported_standard_type.sysml");
    let (out, exit) = json(&path);
    assert!(has_code(&out, "W301"), "{}", out);
    assert_eq!(exit, 0);
}

#[test]
fn w302_empty_package_requires_pedantic() {
    let path = format!("{INVALID_DIR}w302_empty_package.sysml");
    assert_absent_without_pedantic(&path, "W302");
    assert_has_pedantic(&path, "W302");
}

#[test]
fn w306_naming_convention_requires_pedantic() {
    let path = format!("{INVALID_DIR}w306_naming_convention.sysml");
    assert_absent_without_pedantic(&path, "W306");
    let (out, _exit) = json_with(&["--pedantic"], &path);
    let count = out.matches("\"code\": \"W306\"").count();
    assert_eq!(
        count, 2,
        "une définition en minuscule + un usage en majuscule : {out}"
    );
}

#[test]
fn w307_requirement_without_subject_requires_pedantic() {
    let path = format!("{INVALID_DIR}w307_requirement_without_subject.sysml");
    assert_absent_without_pedantic(&path, "W307");
    assert_has_pedantic(&path, "W307");
}

#[test]
fn w309_untyped_usage_requires_pedantic() {
    let path = format!("{INVALID_DIR}w309_untyped_usage.sysml");
    assert_absent_without_pedantic(&path, "W309");
    assert_has_pedantic(&path, "W309");
}

#[test]
fn w310_connection_without_ends() {
    let path = format!("{INVALID_DIR}w310_connection_without_ends.sysml");
    let (out, exit) = json(&path);
    assert!(has_code(&out, "W310"), "{}", out);
    assert_eq!(exit, 0);
}

#[test]
fn w311_non_standard_keyword_requires_pedantic() {
    let path = format!("{INVALID_DIR}w311_non_standard_keyword.sysml");
    assert_absent_without_pedantic(&path, "W311");
    let (out, _exit) = json_with(&["--pedantic"], &path);
    let count = out.matches("\"code\": \"W311\"").count();
    assert_eq!(count, 3, "readonly + composite + portion : {out}");
}

#[test]
fn w312_kerml_only_keyword_requires_pedantic() {
    let path = format!("{INVALID_DIR}w312_kerml_only_keyword.sysml");
    assert_absent_without_pedantic(&path, "W312");
    let (out, _exit) = json_with(&["--pedantic"], &path);
    let count = out.matches("\"code\": \"W312\"").count();
    assert_eq!(
        count, 4,
        "feature + namespace + specialization + subclassification : {out}"
    );
}

#[test]
fn w313_public_import_at_top_level_requires_pedantic() {
    let path = format!("{INVALID_DIR}w313_public_import_at_top_level.sysml");
    assert_absent_without_pedantic(&path, "W313");
    assert_has_pedantic(&path, "W313");
}

// --------------------------------------------------------------------------
// Cas limites
// --------------------------------------------------------------------------

#[test]
fn edge_empty_file_has_no_diagnostics() {
    assert_ok(&format!("{EDGE_DIR}empty_file.sysml"));
}

#[test]
fn edge_only_comments_has_no_diagnostics() {
    assert_ok(&format!("{EDGE_DIR}only_comments.sysml"));
}

#[test]
fn edge_unicode_identifiers_are_accepted() {
    assert_ok(&format!("{EDGE_DIR}unicode_identifiers.sysml"));
}

#[test]
fn edge_tabs_indentation_is_accepted() {
    assert_ok(&format!("{EDGE_DIR}tabs_indentation.sysml"));
}

#[test]
fn edge_crlf_line_endings_are_accepted() {
    assert_ok(&format!("{EDGE_DIR}crlf_line_endings.sysml"));
}

#[test]
fn edge_multiplicity_variants_are_all_valid() {
    assert_ok(&format!("{EDGE_DIR}multiplicity_variants.sysml"));
}

#[test]
fn edge_empty_multiplicity_reports_e218() {
    assert_has(&format!("{EDGE_DIR}multiplicity_empty.sysml"), "E218");
}

#[test]
fn edge_conjugated_port_is_valid() {
    assert_ok(&format!("{EDGE_DIR}conjugated_port.sysml"));
}

#[test]
fn edge_quoted_reserved_word_name_is_valid() {
    assert_ok(&format!("{EDGE_DIR}quoted_reserved_word_name.sysml"));
}

#[test]
fn edge_dot_and_double_colon_are_conflated_is_valid() {
    assert_ok(&format!(
        "{EDGE_DIR}dot_and_double_colon_are_conflated.sysml"
    ));
}

/// `$::` (qualification depuis la racine globale) n'est pas reconnue par le
/// lexeur : `$` déclenche E003 au lieu d'être traité comme un caractère de
/// nom valide. Limitation documentée, pas un bug à corriger dans ce lot.
#[test]
fn edge_global_qualification_unsupported_reports_e003() {
    assert_has(
        &format!("{EDGE_DIR}global_qualification_unsupported.sysml"),
        "E003",
    );
}

/// Un import joker profond vers un paquet totalement inconnu du vérificateur
/// ne doit pas faire planter l'outil : tous les noms non résolus du fichier
/// deviennent des avertissements tolérants (W200), pas des erreurs.
#[test]
fn edge_wildcard_deep_import_downgrades_unresolved_to_warnings() {
    let path = format!("{EDGE_DIR}wildcard_deep_import.sysml");
    let (out, exit) = json(&path);
    assert!(out.contains("\"ok\": true"), "{}", out);
    assert_eq!(exit, 0);
    assert!(has_code(&out, "W200"));
    assert!(!has_code(&out, "E200"));
}

// --------------------------------------------------------------------------
// Options de la ligne de commande
// --------------------------------------------------------------------------

#[test]
fn cli_deny_warnings_turns_warnings_into_failure() {
    let path = format!("{INVALID_DIR}w307_requirement_without_subject.sysml");
    let out = run(&["--deny-warnings", "--pedantic", &path]);
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn cli_unresolved_off_suppresses_unresolved_diagnostics() {
    let path = format!("{INVALID_DIR}e200_unresolved_name.sysml");
    let (out, exit) = json_with(&["--unresolved", "off"], &path);
    assert!(!has_code(&out, "E200"));
    assert!(!has_code(&out, "W200"));
    assert_eq!(exit, 0);
}

#[test]
fn cli_unresolved_warn_downgrades_error_to_warning() {
    let path = format!("{INVALID_DIR}e200_unresolved_name.sysml");
    let (out, exit) = json_with(&["--unresolved", "warn"], &path);
    assert!(has_code(&out, "W200"), "{}", out);
    assert_eq!(exit, 0);
}

#[test]
fn cli_stdin_is_read_when_flag_set() {
    // On utilise une fixture qui produit un diagnostic afin de vérifier que
    // le nom donné via --name apparaît bien comme fichier du diagnostic.
    let path = format!("{INVALID_DIR}e200_unresolved_name.sysml");
    let src = std::fs::read_to_string(&path).unwrap();
    let mut child = Command::new(bin())
        .args([
            "--format",
            "json",
            "--stdin",
            "--name",
            "depuis_stdin.sysml",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(src.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(has_code(&stdout, "E200"), "{}", stdout);
    assert!(stdout.contains("depuis_stdin.sysml"), "{}", stdout);
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn cli_gitlab_format_produces_code_quality_json() {
    let path = format!("{INVALID_DIR}e201_duplicate_name.sysml");
    let out = run(&["--format", "gitlab", &path]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.trim_start().starts_with('['));
    assert!(stdout.contains("\"check_name\": \"E201\""));
    assert!(stdout.contains("\"fingerprint\""));
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn cli_human_format_is_readable_and_mentions_rule() {
    let path = format!("{INVALID_DIR}e201_duplicate_name.sysml");
    let out = run(&[&path]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("E201"));
    assert!(stdout.contains("duplicate-name"));
}

#[test]
fn cli_quiet_suppresses_per_diagnostic_output_but_keeps_summary() {
    let path = format!("{INVALID_DIR}e201_duplicate_name.sysml");
    let out = run(&["--quiet", &path]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!stdout.contains("E201"));
    assert!(stdout.contains("erreur"));
}

#[test]
fn cli_emit_ast_includes_ast_and_omits_diagnostics_array_content() {
    let path = format!("{VALID_DIR}basics.sysml");
    let (out, _exit) = json_with(&["--emit", "ast"], &path);
    assert!(out.contains("\"ast\": ["));
    assert!(out.contains("\"qualifiedName\""));
}

#[test]
fn cli_max_diags_caps_diagnostic_count() {
    let path = format!("{INVALID_DIR}e213_legacy_keyword.sysml");
    let (out, _exit) = json_with(&["--max-diags", "2"], &path);
    let count = out.matches("\"code\":").count();
    assert_eq!(count, 2, "sortie :\n{out}");
}

#[test]
fn cli_list_rules_lists_every_catalog_entry() {
    let out = run(&["--list-rules"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"E210\""));
    assert!(stdout.contains("\"W310\""));
    assert_eq!(out.status.code(), Some(0));
}

/// Chaque règle déclare d'où elle tire son autorité — c'est ce qui distingue
/// une exigence de la spécification d'une convention maison.
#[test]
fn cli_list_rules_declares_an_authority_per_rule() {
    let out = run(&["--list-rules"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let codes = stdout.matches("\"code\":").count();
    let authorities = stdout.matches("\"authority\":").count();
    assert_eq!(codes, authorities, "sortie :\n{stdout}");
    for value in ["\"spec\"", "\"grammar\"", "\"style\""] {
        assert!(
            stdout.contains(value),
            "autorité `{value}` absente :\n{stdout}"
        );
    }
    // Les règles adossées au validateur de référence.
    assert!(stdout.contains("\"verify-outside-verification-objective\""));
    assert!(!stdout.contains("\"verify-outside-requirement\""));
}

/// Un nom retiré de la bibliothèque standard est une erreur pour la version
/// courante, et seulement un avertissement pour la version qui le définissait.
#[test]
fn w314_legacy_library_name_depends_on_the_targeted_version() {
    let path = format!("{INVALID_DIR}w314_legacy_library_name.sysml");

    let (out, exit) = json(&path);
    assert!(has_code(&out, "E200"), "sortie :\n{out}");
    assert!(!has_code(&out, "W314"), "sortie :\n{out}");
    assert_eq!(exit, 1);

    let (out, exit) = json_with(&["--library-version", "2024-11"], &path);
    assert!(has_code(&out, "W314"), "sortie :\n{out}");
    assert!(!has_code(&out, "E200"), "sortie :\n{out}");
    assert_eq!(exit, 0, "sortie :\n{out}");
}

/// Le message par défaut doit nommer le remplacement *et* le drapeau : c'est
/// par là qu'on découvre qu'une autre version du standard existe.
#[test]
fn w314_default_message_points_at_the_replacement_and_the_flag() {
    let out = run(&[&format!("{INVALID_DIR}w314_legacy_library_name.sysml")]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Flows::Flow"), "sortie :\n{stdout}");
    assert!(stdout.contains("--library-version"), "sortie :\n{stdout}");
}

#[test]
fn cli_library_version_rejects_an_unknown_value() {
    let out = run(&[
        "--library-version",
        "1999-01",
        &format!("{VALID_DIR}basics.sysml"),
    ]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("--library-version"));
}

#[test]
fn cli_help_flag_exits_zero() {
    let out = run(&["--help"]);
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).contains("USAGE"));
}

#[test]
fn cli_version_flag_exits_zero() {
    let out = run(&["--version"]);
    assert_eq!(out.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&out.stdout).contains("sysml-check"));
}

#[test]
fn cli_unknown_option_is_a_usage_error() {
    let out = run(&["--nope-not-a-flag"]);
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn cli_no_files_is_a_usage_error() {
    let out = run(&[]);
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn cli_missing_file_is_an_io_error() {
    let out = run(&["tests/fixtures/does_not_exist.sysml"]);
    assert_eq!(out.status.code(), Some(2));
}
