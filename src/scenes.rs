use std::{fs, io::Write, path::Path};

use crate::utils::{make_path_if_not_exists, pascal_to_snake_case, to_resource_path};

const ACTIONS: &str = "actions";
const CONSTS: &str = "consts";

/// Finds all `.tscn` files in the given resource path and generates scene constants and/or actions as specified.
pub fn generate_scenes(
    output_dir: &str,
    resource_path: &str,
    scene_consts: bool,
    scene_actions: bool,
) -> Vec<String> {
    let mut generated_modules = Vec::new();

    let output_dir = Path::new(output_dir);
    let resource_dir = Path::new(resource_path);

    // recursively find all .tscn files
    let mut scenes_and_paths = Vec::new();
    for entry in walkdir::WalkDir::new(resource_dir) {
        let entry = entry.unwrap();
        if entry.path().is_file()
            && entry.path().extension().is_some()
            && entry.path().extension().unwrap() == "tscn"
        {
            let scene_path = entry.path().to_str().unwrap().replace("\\", "/");
            let scene_name = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            scenes_and_paths.push((
                scene_name,
                to_resource_path(scene_path.as_str(), resource_path),
            ));
        }
    }

    if scene_consts {
        let mn = mod_name(CONSTS);
        let consts_output = format_scenes_to_consts(&scenes_and_paths);
        let consts_path = output_dir.join(format!("{}.rs", mn));

        make_path_if_not_exists(consts_path.to_str().unwrap());

        let mut consts_file = fs::File::create(consts_path).unwrap();
        consts_file.write_all(consts_output.as_bytes()).unwrap();

        generated_modules.push(mn)
    }

    if scene_actions {
        let mn = mod_name(ACTIONS);
        let actions_output = format_scenes_to_actions(&scenes_and_paths);
        let actions_path = output_dir.join(format!("{}.rs", mn));

        make_path_if_not_exists(actions_path.to_str().unwrap());

        let mut actions_file = fs::File::create(actions_path).unwrap();
        actions_file.write_all(actions_output.as_bytes()).unwrap();

        generated_modules.push(mn)
    }

    generated_modules
}

fn mod_name(output: &str) -> String {
    format!("scene_{}", output)
}

fn format_scenes_to_consts(scenes_and_paths: &Vec<(String, String)>) -> String {
    format!(
        "#[allow(dead_code)]\n{}",
        scenes_and_paths
            .iter()
            .map(|(name, path)| format_scene_to_const(name, path))
            .collect::<Vec<String>>()
            .join("\n")
    )
}
#[test]
fn test_format_scenes_to_consts() {
    let scenes_and_paths = vec![
        ("Main".to_string(), "res://scenes/Main.tscn".to_string()),
        (
            "LevelOne".to_string(),
            "res://scenes/LevelOne.tscn".to_string(),
        ),
    ];

    let expected = "#[allow(dead_code)]\npub const MAIN: &'static str = \"res://scenes/Main.tscn\";\npub const LEVEL_ONE: &'static str = \"res://scenes/LevelOne.tscn\";";

    let result = format_scenes_to_consts(&scenes_and_paths);
    assert_eq!(result, expected);
}

fn format_scene_to_const(scene_name: &str, scene_path: &str) -> String {
    format!(
        "pub const {}: &'static str = \"{}\";",
        pascal_to_snake_case(scene_name).to_uppercase(),
        scene_path
    )
}
#[test]
fn test_format_scene_to_const() {
    assert_eq!(
        format_scene_to_const("Main", "res://scenes/Main.tscn"),
        "pub const MAIN: &'static str = \"res://scenes/Main.tscn\";"
    );
    assert_eq!(
        format_scene_to_const("LevelOne", "res://scenes/LevelOne.tscn"),
        "pub const LEVEL_ONE: &'static str = \"res://scenes/LevelOne.tscn\";"
    );
}

fn format_scenes_to_actions(scenes_and_paths: &Vec<(String, String)>) -> String {
    format!(
        r#"#[allow(dead_code)]
use godot::{{
    prelude::Node,
    global::Error
}};

pub trait SceneActions {{
    fn change_scene_to(&self, scene_path: &str) -> Option<Error>;
{}
}}

impl SceneActions for Node {{
    fn change_scene_to(&self, scene_path: &str) -> Option<Error> {{
        let st = self.get_tree();
        let mut err = None;

        if st.is_some() {{
            err = Some(st.unwrap().change_scene_to_file(scene_path));
        }}

        err
    }}

{}
}}"#,
        scenes_and_paths
            .iter()
            .map(|(name, _)| format_scene_to_action_trait(name))
            .collect::<Vec<String>>()
            .join("\n"),
        scenes_and_paths
            .iter()
            .map(|(name, path)| format_scene_to_action_impl(name, path))
            .collect::<Vec<String>>()
            .join("\n")
    )
}
#[test]
fn test_format_scenes_to_actions() {
    let scenes_and_paths = vec![
        ("Main".to_string(), "res://scenes/Main.tscn".to_string()),
        (
            "LevelOne".to_string(),
            "res://scenes/LevelOne.tscn".to_string(),
        ),
    ];

    let expected = r#"#[allow(dead_code)]
use godot::{
    prelude::Node,
    global::Error
};

pub trait SceneActions {
    fn change_scene_to(&self, scene_path: &str) -> Option<Error>;
    fn change_scene_to_main(&self) -> Option<Error>;
    fn change_scene_to_level_one(&self) -> Option<Error>;
}

impl SceneActions for Node {
    fn change_scene_to(&self, scene_path: &str) -> Option<Error> {
        let st = self.get_tree();
        let mut err = None;

        if st.is_some() {
            err = Some(st.unwrap().change_scene_to_file(scene_path));
        }

        err
    }

    fn change_scene_to_main(&self) -> Option<Error> { self.change_scene_to("res://scenes/Main.tscn"); }
    fn change_scene_to_level_one(&self) -> Option<Error> { self.change_scene_to("res://scenes/LevelOne.tscn"); }
}"#;

    let result = format_scenes_to_actions(&scenes_and_paths);
    assert_eq!(result, expected);
}

fn format_scene_to_action_trait(scene_name: &str) -> String {
    format!(
        "    fn change_scene_to_{}(&self) -> Option<Error>;",
        pascal_to_snake_case(scene_name)
    )
}
#[test]
fn test_format_scene_to_action_trait() {
    assert_eq!(
        format_scene_to_action_trait("Main"),
        "    fn change_scene_to_main(&self) -> Option<Error>;"
    );
    assert_eq!(
        format_scene_to_action_trait("LevelOne"),
        "    fn change_scene_to_level_one(&self) -> Option<Error>;"
    );
}

fn format_scene_to_action_impl(scene_name: &str, scene_path: &str) -> String {
    format!(
        "    fn change_scene_to_{}(&self) -> Option<Error> {{ self.change_scene_to(\"{}\"); }}",
        pascal_to_snake_case(scene_name),
        scene_path
    )
}
#[test]
fn test_format_scene_to_action_impl() {
    assert_eq!(
        format_scene_to_action_impl("Main", "res://scenes/Main.tscn"),
        "    fn change_scene_to_main(&self) -> Option<Error> { self.change_scene_to(\"res://scenes/Main.tscn\"); }"
    );
    assert_eq!(
        format_scene_to_action_impl("LevelOne", "res://scenes/LevelOne.tscn"),
        "    fn change_scene_to_level_one(&self) -> Option<Error> { self.change_scene_to(\"res://scenes/LevelOne.tscn\"); }"
    );
}
