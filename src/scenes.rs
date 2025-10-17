use std::{collections::HashMap, fs, io::Write, path::Path};

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
    let mut scenes_and_paths: HashMap<String, (String, String)> = HashMap::new();
    for entry in walkdir::WalkDir::new(resource_dir) {
        let entry = entry.unwrap();
        if entry.path().is_file()
            && entry.path().extension().is_some()
            && entry.path().extension().unwrap() == "tscn"
        {
            let scene_path = entry.path().to_str().unwrap().replace("\\", "/");
            let mut scene_name = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            // while there is a name collision, prepend parent folder name
            let mut parent = entry.path().parent();
            while scenes_and_paths.contains_key(&scene_name) {
                if let Some(p) = parent {
                    if let Some(folder_name) = p.file_name() {
                        scene_name = format!("{}{}", folder_name.to_str().unwrap(), scene_name);
                        parent = p.parent();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            scenes_and_paths.insert(
                scene_name.clone(),
                (
                    scene_name,
                    to_resource_path(scene_path.as_str(), resource_path),
                ),
            );
        }
    }

    // convert to vec and sort by least directories then alphabetical
    let mut scenes_and_paths: Vec<(String, String)> =
        scenes_and_paths.into_iter().map(|(_, v)| v).collect();
    scenes_and_paths
        .sort_by(|a, b| least_directories_then_alphabetical(&a.1.as_str(), &b.1.as_str()));

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

fn least_directories_then_alphabetical(a: &&str, b: &&str) -> std::cmp::Ordering {
    let a_dirs = a.matches('/').count();
    let b_dirs = b.matches('/').count();

    if a_dirs != b_dirs {
        a_dirs.cmp(&b_dirs)
    } else {
        a.cmp(b)
    }
}
#[test]
fn test_least_directories_then_alphabetical() {
    let mut paths = vec![
        "res://scenes/subfolder/LevelTwo.tscn",
        "res://scenes/Main.tscn",
        "res://scenes/LevelOne.tscn",
        "res://scenes/subfolder/AnotherScene.tscn",
    ];

    paths.sort_by(least_directories_then_alphabetical);

    let expected = vec![
        "res://scenes/LevelOne.tscn",
        "res://scenes/Main.tscn",
        "res://scenes/subfolder/AnotherScene.tscn",
        "res://scenes/subfolder/LevelTwo.tscn",
    ];

    assert_eq!(paths, expected);
}

fn format_scenes_to_consts(scenes_and_paths: &Vec<(String, String)>) -> String {
    format!(
        "#![allow(dead_code)]\n{}",
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

    let expected = "#![allow(dead_code)]\n/// `res://scenes/Main.tscn`\npub const MAIN: &'static str = \"res://scenes/Main.tscn\";\n/// `res://scenes/LevelOne.tscn`\npub const LEVEL_ONE: &'static str = \"res://scenes/LevelOne.tscn\";";

    let result = format_scenes_to_consts(&scenes_and_paths);
    assert_eq!(result, expected);
}

fn format_scene_to_const(scene_name: &str, scene_path: &str) -> String {
    format!(
        "{}\npub const {}: &'static str = \"{}\";",
        format_scene_to_doc_comment(scene_path),
        pascal_to_snake_case(scene_name).to_uppercase(),
        scene_path
    )
}
#[test]
fn test_format_scene_to_const() {
    assert_eq!(
        format_scene_to_const("Main", "res://scenes/Main.tscn"),
        "/// `res://scenes/Main.tscn`\npub const MAIN: &'static str = \"res://scenes/Main.tscn\";"
    );
    assert_eq!(
        format_scene_to_const("LevelOne", "res://scenes/LevelOne.tscn"),
        "/// `res://scenes/LevelOne.tscn`\npub const LEVEL_ONE: &'static str = \"res://scenes/LevelOne.tscn\";"
    );
}

fn format_scenes_to_actions(scenes_and_paths: &Vec<(String, String)>) -> String {
    format!(
        r#"#![allow(dead_code)]
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
            .map(|(name, path)| format_scene_to_action_trait(name, path))
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

    let expected = r#"#![allow(dead_code)]
use godot::{
    prelude::Node,
    global::Error
};

pub trait SceneActions {
    fn change_scene_to(&self, scene_path: &str) -> Option<Error>;
    /// `res://scenes/Main.tscn`
    fn change_scene_to_main(&self) -> Option<Error>;
    /// `res://scenes/LevelOne.tscn`
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

    fn change_scene_to_main(&self) -> Option<Error> { self.change_scene_to("res://scenes/Main.tscn") }
    fn change_scene_to_level_one(&self) -> Option<Error> { self.change_scene_to("res://scenes/LevelOne.tscn") }
}"#;

    let result = format_scenes_to_actions(&scenes_and_paths);
    assert_eq!(result, expected);
}

fn format_scene_to_action_trait(scene_name: &str, scene_path: &str) -> String {
    format!(
        "    {}\n    fn change_scene_to_{}(&self) -> Option<Error>;",
        format_scene_to_doc_comment(scene_path),
        pascal_to_snake_case(scene_name)
    )
}
#[test]
fn test_format_scene_to_action_trait() {
    assert_eq!(
        format_scene_to_action_trait("Main", "res://scenes/Main.tscn"),
        "    /// `res://scenes/Main.tscn`\n    fn change_scene_to_main(&self) -> Option<Error>;"
    );
    assert_eq!(
        format_scene_to_action_trait("LevelOne", "res://scenes/LevelOne.tscn"),
        "    /// `res://scenes/LevelOne.tscn`\n    fn change_scene_to_level_one(&self) -> Option<Error>;"
    );
}

fn format_scene_to_action_impl(scene_name: &str, scene_path: &str) -> String {
    format!(
        "    fn change_scene_to_{}(&self) -> Option<Error> {{ self.change_scene_to(\"{}\") }}",
        pascal_to_snake_case(scene_name),
        scene_path
    )
}
#[test]
fn test_format_scene_to_action_impl() {
    assert_eq!(
        format_scene_to_action_impl("Main", "res://scenes/Main.tscn"),
        "    fn change_scene_to_main(&self) -> Option<Error> { self.change_scene_to(\"res://scenes/Main.tscn\") }"
    );
    assert_eq!(
        format_scene_to_action_impl("LevelOne", "res://scenes/LevelOne.tscn"),
        "    fn change_scene_to_level_one(&self) -> Option<Error> { self.change_scene_to(\"res://scenes/LevelOne.tscn\") }"
    );
}

fn format_scene_to_doc_comment(scene_path: &str) -> String {
    format!("/// `{}`", scene_path)
}
