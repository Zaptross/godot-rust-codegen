use std::{fs, io::Write};

use crate::projectgodot::ProjectGodot;

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
    let mut actions = inputs.inputs.iter().map(|a| a.0).collect::<Vec<&&str>>();
    actions.sort();

    let mut output_mods: Vec<String> = vec![];

    if output_consts {
        let input_actions = actions
            .iter()
            .map(|action| format_action_to_const(action))
            .collect::<Vec<String>>()
            .join("\n");

        make_path_if_not_exists(get_action_consts_file(output_dir).as_str());

        let mut file = fs::File::create(get_action_consts_file(output_dir)).unwrap();
        file.write_all(get_consts_file_content(input_actions.as_str()).as_bytes())
            .unwrap();

        output_mods.push(mod_name(MOD_CONSTS));
    }

    if output_invocations {
        let trait_defs = actions
            .iter()
            .map(|action| format_action_to_invocation_trait(action))
            .collect::<Vec<String>>()
            .join("\n\n");
        let impl_defs = actions
            .iter()
            .map(|action| format_action_to_invocation_impl(action))
            .collect::<Vec<String>>()
            .join("\n\n");

        make_path_if_not_exists(get_action_consts_file(output_dir).as_str());

        let mut file =
            fs::File::create(format!("{}/{}.rs", output_dir, mod_name(MOD_INVOCATIONS))).unwrap();
        file.write_all(get_invocations_file_content(&trait_defs, &impl_defs).as_bytes())
            .unwrap();

        output_mods.push(mod_name(MOD_INVOCATIONS));
    }

    output_mods
}

fn make_path_if_not_exists(path: &str) {
    let path_obj = std::path::Path::new(path);
    if !path_obj.exists() {
        if let Some(parent) = path_obj.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap();
            }
        }
        fs::File::create(path_obj).unwrap();
    }
}

fn get_action_consts_file(output_dir: &str) -> String {
    if (!output_dir.ends_with('/')) && (!output_dir.ends_with('\\')) {
        format!("{}/{}.rs", output_dir, mod_name(MOD_CONSTS))
    } else {
        format!("{}{}.rs", output_dir, mod_name(MOD_CONSTS))
    }
}

fn get_consts_file_content(consts: &str) -> String {
    format!(
        "#![allow(dead_code)]\nuse godot::builtin::StringName;\n\n{}",
        consts
    )
}

fn format_action_to_const(action: &str) -> String {
    format!(
        "pub fn {}() -> StringName {{ StringName::from(\"{}\") }}",
        pascal_to_snake_case(action).to_ascii_uppercase(),
        action
    )
}

fn get_invocations_file_content(trait_defs: &str, impl_defs: &str) -> String {
    format!(
        "#![allow(dead_code)]\nuse godot::{{builtin::StringName, classes::Input}};\n\npub trait InputActionInvocations {{\n{}\n}}\n\nimpl InputActionInvocations for Input {{\n{}\n}}",
        trait_defs, impl_defs
    )
}
fn format_action_to_invocation_trait(action: &str) -> String {
    let sc = pascal_to_snake_case(action);

    vec![
        format!("    fn is_{}_pressed(&self) -> bool;", sc),
        format!("fn is_{}_just_pressed(&self) -> bool;", sc),
        format!("fn is_{}_just_released(&self) -> bool;", sc),
    ]
    .join("\n    ")
}
fn format_action_to_invocation_impl(action: &str) -> String {
    let sc = pascal_to_snake_case(action);

    vec![
        format!(
            "    fn is_{}_pressed(&self) -> bool {{ self.is_action_pressed(StringName::from(\"{}\")) }}",
            sc,
            action
        ),
        format!(
            "fn is_{}_just_pressed(&self) -> bool {{ self.is_action_just_pressed(StringName::from(\"{}\")) }}",
            sc,
            action
        ),
        format!(
            "fn is_{}_just_released(&self) -> bool {{ self.is_action_just_released(StringName::from(\"{}\")) }}",
            sc,
            action
        ),
    ]
    .join("\n    ")
}

fn pascal_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}
