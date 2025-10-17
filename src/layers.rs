use std::{collections::HashMap, fs, io::Write, path::Path};

use crate::{projectgodot::ProjectGodot, utils::to_upper_camel_case};

const MOD_LAYERS: &str = "layer_consts";

pub fn generate_layers_consts(output_dir: &str, godot_project: &ProjectGodot) -> Vec<String> {
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    if !godot_project.layer_names.is_some()
        || godot_project
            .layer_names
            .as_ref()
            .unwrap()
            .layers
            .is_empty()
    {
        println!(
            "cargo::warning=No layer names found in project.godot, skipping layers.rs generation"
        );
        return vec![];
    }

    let mut layers = godot_project
        .layer_names
        .as_ref()
        .unwrap()
        .layers
        .iter()
        .map(|l| extract_group_data(l.0, l.1))
        .filter(|l| l.is_some())
        .map(|l| l.unwrap())
        .collect::<Vec<(String, i32, String)>>();

    // sort by group name, then by layer number
    layers.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let layers_by_group: HashMap<String, Vec<(i32, String)>> = {
        let mut map: HashMap<String, Vec<(i32, String)>> = HashMap::new();
        for (group, number, name) in layers {
            map.entry(group)
                .or_insert_with(Vec::new)
                .push((number, name));
        }
        map
    };

    let mut rendered_groups = layers_by_group
        .iter()
        .map(|(group, layers)| format_group_to_enum(group, layers))
        .collect::<Vec<String>>();

    rendered_groups.sort();

    let output_lines = format!("#![allow(dead_code)]\n\n{}", rendered_groups.join("\n"));

    let layers_path = Path::new(output_dir).join(format!("{}.rs", MOD_LAYERS));

    let mut file = fs::File::create(layers_path).unwrap();
    file.write_all(output_lines.as_bytes()).unwrap();

    vec![MOD_LAYERS.to_string()]
}

/// Formats a group of layers into a Rust enum string.
///
/// e.g. for group `"Physics2D"` and layers `[(1, "Layer1"), (2, "Layer2")]`, it returns:
///
/// ```no_run
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// pub enum Physics2D {
///   LAYER1 = 1,
///   LAYER2 = 2,
/// }
/// ```
fn format_group_to_enum(group: &str, layers: &Vec<(i32, String)>) -> String {
    let mut enum_str = format!(
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum {} {{\n",
        group
    );

    for (number, name) in layers {
        enum_str.push_str(&format!(
            "    {} = {},\n",
            name.to_uppercase().replace(" ", "_"),
            1 << (number - 1)
        ));
    }

    enum_str.push_str("}\n");
    enum_str
}

#[test]
fn test_format_group_to_enum() {
    let group = "Physics2D";
    let layers = vec![(1, "Layer1".to_string()), (2, "Layer2".to_string())];
    let expected = r#"#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Physics2D {
    LAYER1 = 1,
    LAYER2 = 2,
}
"#;
    assert_eq!(format_group_to_enum(group, &layers), expected);
}

/// Extracts group name and group number from a layer group string.
///
/// e.g. `"2d_physics/layer_1"` -> `("Physics2d", 2)`
fn extract_group_data(group: &str, name: &str) -> Option<(String, i32, String)> {
    let parts = group.split('/').collect::<Vec<&str>>();

    if parts.len() != 2 {
        return None;
    }

    let group = to_upper_camel_case(reorder_group_name(parts.first().unwrap()).as_str());
    let number = parts
        .last()
        .unwrap()
        .replace("layer_", "")
        .parse::<i32>()
        .unwrap();

    Some((group, number, name.to_string()))
}

#[test]
fn test_extract_group_data() {
    let input_group = "2d_physics/layer_1";
    let input_name = "Layer1";
    let expected = Some(("Physics2d".to_string(), 1, "Layer1".to_string()));
    assert_eq!(extract_group_data(input_group, input_name), expected);
}

/// Reorders a group name by reversing the order of its parts.
/// This ensures that groups like "2d_physics" are converted to "physics_2d",
/// preventing issues with names starting with digits.
///
/// e.g. "2d_physics" -> "physics_2d"
fn reorder_group_name(group: &str) -> String {
    let mut parts = group.split("_").collect::<Vec<&str>>();
    parts.reverse();
    parts.join("_")
}

#[test]
fn test_reorder_group_name() {
    let input = "2d_physics";
    let expected = "physics_2d".to_string();
    assert_eq!(reorder_group_name(input), expected);
}
