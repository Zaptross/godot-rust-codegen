use std::{fs, io::Write, path::Path};

use crate::{
    projectgodot::ProjectGodot,
    utils::{make_path_if_not_exists, pascal_to_snake_case},
};

const MOD_CONSTS: &str = "consts";
const MOD_INVOCATIONS: &str = "invocations";

fn mod_name(t: &str) -> String {
    format!("actions_{}", t)
}

pub fn generate_actions(
    output_dir: &str,
    output_consts: bool,
    output_invocations: bool,
    godot_project: &ProjectGodot,
) -> Vec<String> {
    if godot_project.input.is_none() || godot_project.input.as_ref().unwrap().inputs.len() == 0 {
        println!(
            "cargo::warning=No input actions found in project.godot, skipping actions.rs generation"
        );
    }

    let inputs = godot_project.input.as_ref().unwrap();
    let mut actions = inputs
        .inputs
        .iter()
        .map(|(name, input)| {
            (
                name.as_str(),
                input
                    .events
                    .iter()
                    .map(|e| e.get_key_string())
                    .filter_map(|k| k)
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<Vec<(&str, Vec<String>)>>();
    actions.sort();

    godot_project
        .input
        .as_ref()
        .unwrap()
        .inputs
        .iter()
        .for_each(|(_, input)| {
            if input.events.len() == 0 {
                println!(
                    "cargo::warning=Input action '{}' has no events, skipping",
                    input.name
                );
            } else {
                for event in input.events.iter() {
                    println!(
                        "cargo::warning=Input action '{}' has event: {} = {} ({})",
                        input.name,
                        event.event_type,
                        event.get_key_string().unwrap_or("unknown".to_string()),
                        event.int_properties.get("unicode").unwrap_or(&0)
                    );
                }
            }
        });

    let mut output_mods: Vec<String> = vec![];

    if output_consts {
        let input_actions = actions
            .iter()
            .map(|(action, events)| {
                format_action_to_const(action, &get_action_keystroke_doc_comment(events))
            })
            .collect::<Vec<String>>()
            .join("\n");

        make_path_if_not_exists(get_action_mod_file(output_dir, MOD_CONSTS).as_str());

        let mut file = fs::File::create(get_action_mod_file(output_dir, MOD_CONSTS)).unwrap();
        file.write_all(get_consts_file_content(input_actions.as_str()).as_bytes())
            .unwrap();

        output_mods.push(mod_name(MOD_CONSTS));
    }

    if output_invocations {
        let trait_defs = actions
            .iter()
            .map(|(action, events)| format_action_to_invocation_trait(action, events))
            .collect::<Vec<String>>()
            .join("\n\n");
        let impl_defs = actions
            .iter()
            .map(|(action, _)| format_action_to_invocation_impl(action))
            .collect::<Vec<String>>()
            .join("\n\n");

        make_path_if_not_exists(&get_action_mod_file(output_dir, MOD_CONSTS).as_str());

        let mut file = fs::File::create(get_action_mod_file(output_dir, MOD_INVOCATIONS)).unwrap();
        file.write_all(get_invocations_file_content(&trait_defs, &impl_defs).as_bytes())
            .unwrap();

        output_mods.push(mod_name(MOD_INVOCATIONS));
    }

    output_mods
}

fn get_action_keystroke_doc_comment(keystrokes: &Vec<String>) -> String {
    format!(
        "/// Maps to: `{}`",
        keystrokes
            .iter()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>()
            .join("` or `")
    )
}
#[test]
fn test_get_action_keystroke_doc_comment() {
    assert_eq!(
        get_action_keystroke_doc_comment(&vec!["Ctrl+A".into()]),
        "/// Maps to: `Ctrl+A`"
    );
    assert_eq!(
        get_action_keystroke_doc_comment(&vec!["left_click".into(), "mouse_left".into()]),
        "/// Maps to: `left_click` or `mouse_left`"
    );
}

fn get_action_mod_file(output_dir: &str, name: &str) -> String {
    Path::new(output_dir)
        .join(mod_name(name) + ".rs")
        .to_string_lossy()
        .replace("/", "\\")
        .to_string()
}
#[test]
fn test_get_action_mod_file() {
    assert_eq!(
        get_action_mod_file("src/generated", "consts"),
        "src\\generated\\actions_consts.rs"
    );
    assert_eq!(
        get_action_mod_file("src/generated", "invocations"),
        "src\\generated\\actions_invocations.rs"
    );
}

fn get_consts_file_content(consts: &str) -> String {
    format!(
        "#![allow(dead_code)]\n#![allow(non_snake_case)]\nuse godot::builtin::StringName;\n\n{}",
        consts
    )
}
#[test]
fn test_get_consts_file_content() {
    assert_eq!(
        get_consts_file_content(
            "/// Maps to: `Ctrl+A`\npub fn CTRL_A() -> StringName { StringName::from(\"Ctrl+A\") }"
        ),
        "#![allow(dead_code)]\n#![allow(non_snake_case)]\nuse godot::builtin::StringName;\n\n/// Maps to: `Ctrl+A`\npub fn CTRL_A() -> StringName { StringName::from(\"Ctrl+A\") }"
    );
}

fn format_action_to_const(action: &str, doc_comment: &str) -> String {
    format!(
        "{}\npub fn {}() -> StringName {{ StringName::from(\"{}\") }}",
        doc_comment,
        pascal_to_snake_case(action).to_ascii_uppercase(),
        action
    )
}
#[test]
fn test_format_action_to_const() {
    assert_eq!(
        format_action_to_const("Fire", "/// Maps to: `left_click`"),
        "/// Maps to: `left_click`\npub fn FIRE() -> StringName { StringName::from(\"Fire\") }"
    );
}

fn get_invocations_file_content(trait_defs: &str, impl_defs: &str) -> String {
    format!(
        "#![allow(dead_code)]\nuse godot::classes::Input;\n\npub trait InputActionInvocations {{\n{}\n}}\n\nimpl InputActionInvocations for Input {{\n{}\n}}",
        trait_defs, impl_defs
    )
}
#[test]
fn test_get_invocations_file_content() {
    assert_eq!(
        get_invocations_file_content(
            "    /// Returns true while left_click is pressed\n    fn is_fire_pressed(&self) -> bool;",
            "    fn is_fire_pressed(&self) -> bool { self.is_action_pressed(\"Fire\") }"
        ),
        "#![allow(dead_code)]\nuse godot::classes::Input;\n\npub trait InputActionInvocations {\n    /// Returns true while left_click is pressed\n    fn is_fire_pressed(&self) -> bool;\n}\n\nimpl InputActionInvocations for Input {\n    fn is_fire_pressed(&self) -> bool { self.is_action_pressed(\"Fire\") }\n}"
    );
}

fn join_keystrokes(keystrokes: &Vec<String>) -> String {
    keystrokes.join("` or `")
}
#[test]
fn test_join_keystrokes() {
    assert_eq!(join_keystrokes(&vec!["Ctrl+A".into()]), "Ctrl+A");
    assert_eq!(
        join_keystrokes(&vec!["left_click".into(), "mouse_left".into()]),
        "left_click` or `mouse_left"
    );
}

fn format_action_to_invocation_trait(action: &str, keystrokes: &Vec<String>) -> String {
    let sc = pascal_to_snake_case(action);
    let joined_keystrokes = join_keystrokes(keystrokes);
    let conjunction = if keystrokes[0].contains("+") {
        "are"
    } else {
        "is"
    };

    vec![
        format!(
            "    /// Returns true while `{}` {} pressed",
            joined_keystrokes, conjunction
        ),
        format!("fn is_{}_pressed(&self) -> bool;", sc),
        format!(
            "/// Returns true when `{}` {} just pressed",
            joined_keystrokes, conjunction
        ),
        format!("fn is_{}_just_pressed(&self) -> bool;", sc),
        format!(
            "/// Returns true when `{}` {} just released",
            joined_keystrokes, conjunction
        ),
        format!("fn is_{}_just_released(&self) -> bool;", sc),
    ]
    .join("\n    ")
}
#[test]
fn test_format_action_to_invocation_trait() {
    assert_eq!(
        format_action_to_invocation_trait("Fire", &vec!["left_click".into()]),
        "    /// Returns true while `left_click` is pressed\n    fn is_fire_pressed(&self) -> bool;\n    /// Returns true when `left_click` is just pressed\n    fn is_fire_just_pressed(&self) -> bool;\n    /// Returns true when `left_click` is just released\n    fn is_fire_just_released(&self) -> bool;"
    );
    assert_eq!(
        format_action_to_invocation_trait("CtrlA", &vec!["Ctrl+A".into()]),
        "    /// Returns true while `Ctrl+A` are pressed\n    fn is_ctrl_a_pressed(&self) -> bool;\n    /// Returns true when `Ctrl+A` are just pressed\n    fn is_ctrl_a_just_pressed(&self) -> bool;\n    /// Returns true when `Ctrl+A` are just released\n    fn is_ctrl_a_just_released(&self) -> bool;"
    );
    assert_eq!(
        format_action_to_invocation_trait("MultiKey", &vec!["Shift+Ctrl+Alt+X".into(), "Y".into()]),
        "    /// Returns true while `Shift+Ctrl+Alt+X` or `Y` are pressed\n    fn is_multi_key_pressed(&self) -> bool;\n    /// Returns true when `Shift+Ctrl+Alt+X` or `Y` are just pressed\n    fn is_multi_key_just_pressed(&self) -> bool;\n    /// Returns true when `Shift+Ctrl+Alt+X` or `Y` are just released\n    fn is_multi_key_just_released(&self) -> bool;"
    );
}

fn format_action_to_invocation_impl(action: &str) -> String {
    let sc = pascal_to_snake_case(action);

    vec![
        format!(
            "    fn is_{}_pressed(&self) -> bool {{ self.is_action_pressed(\"{}\") }}",
            sc, action
        ),
        format!(
            "fn is_{}_just_pressed(&self) -> bool {{ self.is_action_just_pressed(\"{}\") }}",
            sc, action
        ),
        format!(
            "fn is_{}_just_released(&self) -> bool {{ self.is_action_just_released(\"{}\") }}",
            sc, action
        ),
    ]
    .join("\n    ")
}
#[test]
fn test_format_action_to_invocation_impl() {
    assert_eq!(
        format_action_to_invocation_impl("Fire"),
        "    fn is_fire_pressed(&self) -> bool { self.is_action_pressed(\"Fire\") }\n    fn is_fire_just_pressed(&self) -> bool { self.is_action_just_pressed(\"Fire\") }\n    fn is_fire_just_released(&self) -> bool { self.is_action_just_released(\"Fire\") }"
    );
}
