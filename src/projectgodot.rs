// Allow dead code because to better represent the structure of the file, even if some fields are not used.
#![allow(dead_code)]

use godot::{global::Key, obj::EngineEnum};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Parsed representation of a `project.godot` file
pub struct ProjectGodot<'a> {
    pub config_version: Option<u32>,
    pub application: Option<ApplicationSection<'a>>,
    pub autoload: Option<AutoloadSection<'a>>,
    pub dotnet: Option<DotnetSection<'a>>,
    pub input: Option<InputSection>,
    pub layer_names: Option<LayerNamesSection<'a>>,
    pub rendering: Option<RenderingSection<'a>>,
}

impl ProjectGodot<'_> {
    pub fn parse_from_str<'a>(content: &'a str) -> ProjectGodot<'a> {
        let mut godot_project = ProjectGodot::new();
        let sections = Self::split_sections(content);

        if let Some(global_section) = sections.first() {
            if !global_section.trim().starts_with('[') {
                for line in global_section.lines() {
                    if let Some((key, value)) = line.split_once('=') {
                        if key.trim() == "config_version" {
                            godot_project.config_version = value.trim().parse::<u32>().ok();
                        }
                    }
                }
            }
        }

        for section in &sections {
            let trimmed_section = section.trim();
            if trimmed_section.starts_with("[application]") {
                godot_project.application = ApplicationSection::parse(section);
            } else if trimmed_section.starts_with("[autoload]") {
                godot_project.autoload = AutoloadSection::parse(section);
            } else if trimmed_section.starts_with("[dotnet]") {
                godot_project.dotnet = DotnetSection::parse(section);
            } else if trimmed_section.starts_with("[input]") {
                godot_project.input = InputSection::parse(section);
            } else if trimmed_section.starts_with("[layer_names]") {
                godot_project.layer_names = LayerNamesSection::parse(section);
            } else if trimmed_section.starts_with("[rendering]") {
                godot_project.rendering = RenderingSection::parse(section);
            }
        }

        godot_project
    }

    fn new() -> Self {
        Self {
            config_version: None,
            application: None,
            autoload: None,
            dotnet: None,
            input: None,
            layer_names: None,
            rendering: None,
        }
    }

    fn split_sections<'a>(file_content: &'a str) -> Vec<&'a str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[\w+\]").unwrap();
        }

        let mut result = Vec::new();
        let mut last = 0;
        for mat in RE.find_iter(file_content) {
            if mat.start() > last {
                let section = &file_content[last..mat.start()];
                if !section.trim().is_empty() {
                    result.push(section);
                }
            }
            last = mat.start();
        }

        if last < file_content.len() {
            result.push(&file_content[last..]);
        }

        result
    }
}

/// Application section of the `project.godot` file
///
/// It has the following format:
/// ```text
/// [application]
/// config/name="ExampleProject"
/// run/main_scene="res://src/assets/main.tscn"
/// config/features=PackedStringArray("4.5", "GL Compatibility")
/// config/icon="res://icon.svg"
/// ```
pub struct ApplicationSection<'a> {
    pub name: Option<&'a str>,
    pub main_scene: Option<&'a str>,
    pub features: Option<Vec<&'a str>>,
    pub icon: Option<&'a str>,
}

impl ApplicationSection<'_> {
    /// Parse a configuration section from `.gdextension` file content
    ///
    /// Returns `None` if the content doesn't start with `[configuration]` header.
    /// Parses key-value pairs and converts string values to appropriate types.
    ///
    /// # Example
    /// ```
    /// # pub struct ApplicationSection<'a> {
    /// #     pub name: Option<&'a str>,
    /// #     pub main_scene: Option<&'a str>,
    /// #     pub features: Option<Vec<&'a str>>,
    /// #     pub icon: Option<&'a str>,
    /// # }
    /// # impl ApplicationSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<ApplicationSection<'a>> {
    /// #         if !content.trim().starts_with("[application]") {
    /// #             return None;
    /// #         }
    /// #         let mut config = ApplicationSection {
    /// #             name: None,
    /// #             main_scene: None,
    /// #             features: None,
    /// #             icon: None,
    /// #         };
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') {
    /// #                 continue;
    /// #             }
    /// #             if let Some((key, value)) = line.split_once('=') {
    /// #                 let key = key.trim();
    /// #                 let value = value.trim().trim_matches('"');
    /// #                 match key {
    /// #                     "config/name" => config.name = Some(value),
    /// #                     "run/main_scene" => config.main_scene = Some(value),
    /// #                     "config/icon" => config.icon = Some(value),
    /// #                     "config/features" => {
    /// #                         let features_str = value
    /// #                             .trim_start_matches("PackedStringArray(")
    /// #                             .trim_end_matches(')');
    /// #                         let features: Vec<&str> = features_str
    /// #                             .split(',')
    /// #                             .map(|s| s.trim().trim_matches('"'))
    /// #                             .collect();
    /// #                         config.features = Some(features);
    /// #                     }
    /// #                     _ => {}
    /// #                 }
    /// #             }
    /// #         }
    /// #         Some(config)
    /// #     }
    /// # }
    ///
    /// let content = r#"[application]
    /// config/name="ExampleProject"
    /// run/main_scene="res://src/assets/main.tscn"
    /// config/features=PackedStringArray("4.5", "GL Compatibility")
    /// config/icon="res://icon.svg"
    /// "#;
    ///
    /// let config = ApplicationSection::parse(content).unwrap();
    /// assert_eq!(config.name, Some("ExampleProject"));
    /// assert_eq!(config.main_scene, Some("res://src/assets/main.tscn"));
    /// assert_eq!(config.icon, Some("res://icon.svg"));
    /// assert_eq!(config.features, Some(vec!["4.5", "GL Compatibility"]));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<ApplicationSection<'a>> {
        if !content.trim().starts_with("[application]") {
            return None;
        }
        let mut config = ApplicationSection {
            name: None,
            main_scene: None,
            features: None,
            icon: None,
        };
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                match key {
                    "config/name" => config.name = Some(value),
                    "run/main_scene" => config.main_scene = Some(value),
                    "config/icon" => config.icon = Some(value),
                    "config/features" => {
                        let features_str = value
                            .trim_start_matches("PackedStringArray(")
                            .trim_end_matches(')');
                        let features: Vec<&str> = features_str
                            .split(',')
                            .map(|s| s.trim().trim_matches('"'))
                            .collect();
                        config.features = Some(features);
                    }
                    _ => {}
                }
            }
        }
        Some(config)
    }
}

/// Autoload section of the `project.godot` file
///
/// It has the following format:
/// ```text
/// [autoload]
/// gamestate="*res://src/game/gamestate.tscn"
/// ```
pub struct AutoloadSection<'a> {
    pub autoloads: HashMap<&'a str, &'a str>,
}

impl AutoloadSection<'_> {
    /// Parse an autoload section from `project.godot` file content
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # pub struct AutoloadSection<'a> {
    /// #     pub autoloads: HashMap<&'a str, &'a str>,
    /// # }
    /// # impl AutoloadSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<AutoloadSection<'a>> {
    /// #         if !content.trim().starts_with("[autoload]") {
    /// #             return None;
    /// #         }
    /// #         let mut autoloads = HashMap::new();
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
    /// #                 continue;
    /// #             }
    /// #             if let Some((key, value)) = line.split_once('=') {
    /// #                 let key = key.trim();
    /// #                 let value = value.trim().trim_matches('"');
    /// #                 autoloads.insert(key, value);
    /// #             }
    /// #         }
    /// #         Some(AutoloadSection { autoloads })
    /// #     }
    /// # }
    ///
    /// let content = r#"[autoload]
    /// gamestate="*res://src/game/gamestate.tscn"
    /// "#;
    ///
    /// let autoload_section = AutoloadSection::parse(content).unwrap();
    /// assert_eq!(autoload_section.autoloads.get("gamestate"), Some(&"*res://src/game/gamestate.tscn"));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<AutoloadSection<'a>> {
        if !content.trim().starts_with("[autoload]") {
            return None;
        }
        let mut autoloads = HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                autoloads.insert(key, value);
            }
        }
        Some(AutoloadSection { autoloads })
    }
}

/// Dotnet section of the `project.godot` file
///
/// It has the following format:
/// ```text
/// [dotnet]
/// project/assembly_name="ExampleProject"
/// ```
pub struct DotnetSection<'a> {
    pub assembly_name: Option<&'a str>,
}

impl DotnetSection<'_> {
    /// Parse a dotnet section from `project.godot` file content
    ///
    /// # Example
    /// ```
    /// # pub struct DotnetSection<'a> {
    /// #     pub assembly_name: Option<&'a str>,
    /// # }
    /// # impl DotnetSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<DotnetSection<'a>> {
    /// #         if !content.trim().starts_with("[dotnet]") {
    /// #             return None;
    /// #         }
    /// #         let mut assembly_name = None;
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
    /// #                 continue;
    /// #             }
    /// #             if let Some((key, value)) = line.split_once('=') {
    /// #                 if key.trim() == "project/assembly_name" {
    /// #                     assembly_name = Some(value.trim().trim_matches('"'));
    /// #                 }
    /// #             }
    /// #         }
    /// #         Some(DotnetSection { assembly_name })
    /// #     }
    /// # }
    ///
    /// let content = r#"[dotnet]
    /// project/assembly_name="ExampleProject"
    /// "#;
    ///
    /// let dotnet_section = DotnetSection::parse(content).unwrap();
    /// assert_eq!(dotnet_section.assembly_name, Some("ExampleProject"));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<DotnetSection<'a>> {
        if !content.trim().starts_with("[dotnet]") {
            return None;
        }
        let mut assembly_name = None;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                if key.trim() == "project/assembly_name" {
                    assembly_name = Some(value.trim().trim_matches('"'));
                }
            }
        }
        Some(DotnetSection { assembly_name })
    }
}

/// Rendering section of the `project.godot` file
///
/// It has the following format:
/// ```text
/// [rendering]
/// renderer/rendering_method="gl_compatibility"
/// renderer/rendering_method.mobile="gl_compatibility"
/// ```
pub struct RenderingSection<'a> {
    pub rendering_method: Option<&'a str>,
    pub rendering_method_mobile: Option<&'a str>,
}

impl RenderingSection<'_> {
    /// Parse a rendering section from `project.godot` file content
    ///
    /// # Example
    /// ```
    /// # pub struct RenderingSection<'a> {
    /// #     pub rendering_method: Option<&'a str>,
    /// #     pub rendering_method_mobile: Option<&'a str>,
    /// # }
    /// # impl RenderingSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<RenderingSection<'a>> {
    /// #         if !content.trim().starts_with("[rendering]") {
    /// #             return None;
    /// #         }
    /// #         let mut rendering_method = None;
    /// #         let mut rendering_method_mobile = None;
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
    /// #                 continue;
    /// #             }
    /// #             if let Some((key, value)) = line.split_once('=') {
    /// #                 let key = key.trim();
    /// #                 let value = value.trim().trim_matches('"');
    /// #                 match key {
    /// #                     "renderer/rendering_method" => rendering_method = Some(value),
    /// #                     "renderer/rendering_method.mobile" => rendering_method_mobile = Some(value),
    /// #                     _ => {}
    /// #                 }
    /// #             }
    /// #         }
    /// #         Some(RenderingSection { rendering_method, rendering_method_mobile })
    /// #     }
    /// # }
    ///
    /// let content = r#"[rendering]
    /// renderer/rendering_method="gl_compatibility"
    /// renderer/rendering_method.mobile="gl_compatibility"
    /// "#;
    ///
    /// let rendering_section = RenderingSection::parse(content).unwrap();
    /// assert_eq!(rendering_section.rendering_method, Some("gl_compatibility"));
    /// assert_eq!(rendering_section.rendering_method_mobile, Some("gl_compatibility"));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<RenderingSection<'a>> {
        if !content.trim().starts_with("[rendering]") {
            return None;
        }
        let mut rendering_method = None;
        let mut rendering_method_mobile = None;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                match key {
                    "renderer/rendering_method" => rendering_method = Some(value),
                    "renderer/rendering_method.mobile" => rendering_method_mobile = Some(value),
                    _ => {}
                }
            }
        }
        Some(RenderingSection {
            rendering_method,
            rendering_method_mobile,
        })
    }
}

/// Layer names section of the `project.godot` file
///
/// It has the following format:
/// ```text
/// [layer_names]
/// 2d_physics/layer_1="collisions"
/// 2d_physics/layer_2="noncolliding"
/// ```
pub struct LayerNamesSection<'a> {
    pub layers: HashMap<&'a str, &'a str>,
}

impl LayerNamesSection<'_> {
    /// Parse a layer_names section from `project.godot` file content
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # pub struct LayerNamesSection<'a> {
    /// #     pub layers: HashMap<&'a str, &'a str>,
    /// # }
    /// # impl LayerNamesSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<LayerNamesSection<'a>> {
    /// #         if !content.trim().starts_with("[layer_names]") {
    /// #             return None;
    /// #         }
    /// #         let mut layers = HashMap::new();
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
    /// #                 continue;
    /// #             }
    /// #             if let Some((key, value)) = line.split_once('=') {
    /// #                 let key = key.trim();
    /// #                 let value = value.trim().trim_matches('"');
    /// #                 layers.insert(key, value);
    /// #             }
    /// #         }
    /// #         Some(LayerNamesSection { layers })
    /// #     }
    /// # }
    ///
    /// let content = r#"[layer_names]
    /// 2d_physics/layer_1="collisions"
    /// 2d_physics/layer_2="noncolliding"
    /// "#;
    ///
    /// let layer_names_section = LayerNamesSection::parse(content).unwrap();
    /// assert_eq!(layer_names_section.layers.get("2d_physics/layer_1"), Some(&"collisions"));
    /// assert_eq!(layer_names_section.layers.get("2d_physics/layer_2"), Some(&"noncolliding"));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<LayerNamesSection<'a>> {
        if !content.trim().starts_with("[layer_names]") {
            return None;
        }
        let mut layers = HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                layers.insert(key, value);
            }
        }
        Some(LayerNamesSection { layers })
    }
}

/// Input section of the `project.godot` file
///
/// It has the following format:
/// ```text
/// [input]
///
/// Fire={
/// "deadzone": 0.5,
/// "events": [Object(InputEventMouseButton,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"button_mask":0,"position":Vector2(0, 0),"global_position":Vector2(0, 0),"factor":1.0,"button_index":1,"canceled":false,"pressed":false,"double_click":false,"script":null)
/// ]
/// }
/// ```
pub struct InputSection {
    pub inputs: HashMap<String, Input>,
}

impl InputSection {
    /// Parse an input section from `project.godot` file content
    pub fn parse<'a>(content: &'a str) -> Option<InputSection> {
        if !content.trim().starts_with("[input]") {
            return None;
        }

        let mut in_block = false;
        let mut block_lines = Vec::new();
        let mut inputs = HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if line.contains("={") {
                in_block = true;
                block_lines = vec![line.to_string()];
            } else if in_block {
                block_lines.push(line.to_string());
                if line == "}" {
                    in_block = false;
                    if let Some(input) = parse_input_from_input_block(block_lines.clone()) {
                        inputs.insert(input.name.clone(), input.clone());
                    }
                }
            }
        }
        return Some(InputSection { inputs });
    }
}

#[test]
fn test_input_section_parse() {
    let content = r#"[input]
    Fire={
    "deadzone": 0.5,
    "events": [Object(InputEventMouseButton,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"button_mask":0,"position":Vector2(0, 0),"global_position":Vector2(0, 0),"factor":1.0,"button_index":1,"canceled":false,"pressed":false,"double_click":false,"script":null)
    ]
    }
    "#;
    let input_section = InputSection::parse(content).unwrap();

    assert_eq!(input_section.inputs.len(), 1);
    let fire_input = input_section.inputs.get("Fire").unwrap();
    assert_eq!(fire_input.name, "Fire");
}

#[derive(Clone)]
pub struct Input {
    pub name: String,
    pub deadzone: Option<f32>,
    pub events: Vec<InputEvent>,
}

fn parse_input_from_input_block(block_lines: Vec<String>) -> Option<Input> {
    let mut name = String::new();
    let mut deadzone = None;
    let mut events = Vec::new();

    for line in block_lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.contains("={") {
            name = line.split("={").next().unwrap().trim().to_string();
        } else if line == "}" {
            break;
        } else if line.contains(": ") {
            let (key, value) = line.split_once(':').unwrap();
            let key = key.trim().trim_matches('"');
            let value = value.trim().trim_matches(',');

            match key {
                "deadzone" => {
                    deadzone = value.parse::<f32>().ok();
                }
                "events" => {
                    let events_str = value.trim_start_matches('[').trim_end_matches(']').trim();
                    let event_strs = split_events_array(events_str);
                    for event_str in event_strs {
                        if let Some(event) = extract_input_event_from_string(&event_str) {
                            events.push(event);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if name.is_empty() {
        return None;
    }

    Some(Input {
        name,
        deadzone,
        events,
    })
}

#[test]
fn test_parse_input_from_input_block() {
    let input = r#"Fire={
"deadzone": 0.5,
"events": [Object(InputEventMouseButton,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"button_mask":0,"position":Vector2(0, 0),"global_position":Vector2(0, 0),"factor":1.0,"button_index":1,"canceled":false,"pressed":false,"double_click":false,"script":null)
]
}"#;

    let block_lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let parsed_input = parse_input_from_input_block(block_lines).unwrap();

    assert_eq!(parsed_input.name, "Fire");
    assert_eq!(parsed_input.deadzone, Some(0.5));
    assert_eq!(parsed_input.events.len(), 1);
}

#[derive(Clone)]
pub struct InputEvent {
    pub event_type: String, // e.g. InputEventKey, InputEventMouseButton
    pub str_properties: HashMap<String, String>,
    pub bool_properties: HashMap<String, bool>,
    pub int_properties: HashMap<String, i32>,
    pub float_properties: HashMap<String, f32>,
    pub vec2_properties: HashMap<String, (f32, f32)>,
}

impl InputEvent {
    // TODO - this may need to be extended to cover more cases, but for now it covers the basics
    // eg: controller buttons
    pub fn get_key_string(&self) -> Option<String> {
        let ctrl = self
            .int_properties
            .get("ctrl_pressed")
            .copied()
            .unwrap_or(0)
            == 1;
        let shift = self
            .int_properties
            .get("shift_pressed")
            .copied()
            .unwrap_or(0)
            == 1;
        let alt = self.int_properties.get("alt_pressed").copied().unwrap_or(0) == 1;

        let key_str = match self.event_type.as_str() {
            "InputEventKey" => key_str_from_codes(
                self.int_properties.get("keycode").copied(),
                self.int_properties.get("physical_keycode").copied(),
                self.int_properties.get("unicode").copied(),
            ),
            "InputEventMouseButton" => mouse_button_str_from_code(
                self.int_properties
                    .get("button_index")
                    .copied()
                    .unwrap_or(8),
                self.bool_properties
                    .get("double_click")
                    .copied()
                    .unwrap_or(false),
            ),
            _ => None,
        };

        return Some(format!(
            "{}{}{}{}",
            if ctrl { "ctrl+" } else { "" },
            if shift { "shift+" } else { "" },
            if alt { "alt+" } else { "" },
            key_str.unwrap_or_else(|| "".to_string())
        ));
    }
}

#[test]
fn test_input_event_get_key_string_keys() {
    let mut event = InputEvent {
        event_type: "InputEventKey".to_string(),
        str_properties: HashMap::new(),
        bool_properties: HashMap::new(),
        int_properties: HashMap::new(),
        float_properties: HashMap::new(),
        vec2_properties: HashMap::new(),
    };

    event.int_properties.insert("unicode".to_string(), 97); // 'a'
    assert_eq!(event.get_key_string(), Some("a".to_string()));

    event.int_properties.insert("keycode".to_string(), 65); // 'A'
    assert_eq!(event.get_key_string(), Some("A".to_string()));

    event.int_properties.insert("ctrl_pressed".to_string(), 1);
    assert_eq!(event.get_key_string(), Some("ctrl+A".to_string()));

    event.int_properties.insert("shift_pressed".to_string(), 1);
    assert_eq!(event.get_key_string(), Some("ctrl+shift+A".to_string()));

    event.int_properties.insert("alt_pressed".to_string(), 1);
    assert_eq!(event.get_key_string(), Some("ctrl+shift+alt+A".to_string()));
}

#[test]
fn test_input_event_get_key_string_mouse() {
    let mut event = InputEvent {
        event_type: "InputEventMouseButton".to_string(),
        str_properties: HashMap::new(),
        bool_properties: HashMap::new(),
        int_properties: HashMap::new(),
        float_properties: HashMap::new(),
        vec2_properties: HashMap::new(),
    };

    event.int_properties.insert("button_index".to_string(), 1); // Left click
    assert_eq!(event.get_key_string(), Some("left_click".to_string()));

    event
        .bool_properties
        .insert("double_click".to_string(), true);
    assert_eq!(
        event.get_key_string(),
        Some("double_left_click".to_string())
    );

    event.int_properties.insert("button_index".to_string(), 2); // Right click
    assert_eq!(
        event.get_key_string(),
        Some("double_right_click".to_string())
    );

    event.int_properties.insert("button_index".to_string(), 4); // Wheel up
    event
        .bool_properties
        .insert("double_click".to_string(), false);
    assert_eq!(event.get_key_string(), Some("wheel_up".to_string()));

    event.int_properties.insert("button_index".to_string(), 5); // Wheel down
    event
        .bool_properties
        .insert("double_click".to_string(), true);
    assert_eq!(
        event.get_key_string(),
        Some("double_wheel_down".to_string())
    );

    event.int_properties.insert("ctrl_pressed".to_string(), 1);
    assert_eq!(
        event.get_key_string(),
        Some("ctrl+double_wheel_down".to_string())
    );

    event.int_properties.insert("button_index".to_string(), 8); // Invalid button
    assert_eq!(event.get_key_string(), Some("ctrl+".to_string()));
}

fn mouse_button_str_from_code(button_index: i32, double_click: bool) -> Option<String> {
    Some(format!(
        "{}{}{}",
        if double_click { "double_" } else { "" },
        match button_index {
            1 => "left",
            2 => "right",
            3 => "middle",
            4 => "wheel_up",
            5 => "wheel_down",
            6 => "wheel_left",
            7 => "wheel_right",
            _ => return None,
        },
        if button_index <= 3 { "_click" } else { "" }
    ))
}

#[test]
fn test_mouse_button_str_from_code() {
    assert_eq!(
        mouse_button_str_from_code(1, false),
        Some("left_click".to_string())
    );
    assert_eq!(
        mouse_button_str_from_code(2, true),
        Some("double_right_click".to_string())
    );
    assert_eq!(
        mouse_button_str_from_code(3, false),
        Some("middle_click".to_string())
    );
    assert_eq!(
        mouse_button_str_from_code(4, false),
        Some("wheel_up".to_string())
    );
    assert_eq!(
        mouse_button_str_from_code(5, true),
        Some("double_wheel_down".to_string())
    );
    assert_eq!(mouse_button_str_from_code(8, false), None);
    assert_eq!(mouse_button_str_from_code(8, true), None);
}

fn key_str_from_codes(
    keycode: Option<i32>,
    physical_keycode: Option<i32>,
    unicode: Option<i32>,
) -> Option<String> {
    if let Some(code) = keycode
        && code != 0
    {
        Some(Key::from_ord(code).as_str().to_string())
    } else if let Some(code) = physical_keycode
        && code != 0
    {
        Some(Key::from_ord(code).as_str().to_string())
    } else if let Some(code) = unicode
        && code != 0
    {
        let code_u8 = [code as u8];
        return Some(
            (match code {
                8 => "backspace",
                9 => "tab",
                13 => "enter",
                27 => "escape",
                32 => "space",
                127 => "delete",
                256 => "left",
                257 => "right",
                258 => "up",
                259 => "down",
                260 => "page_up",
                261 => "page_down",
                262 => "home",
                263 => "end",
                264 => "insert",
                265 => "f1",
                266 => "f2",
                267 => "f3",
                268 => "f4",
                269 => "f5",
                270 => "f6",
                271 => "f7",
                272 => "f8",
                273 => "f9",
                274 => "f10",
                275 => "f11",
                276 => "f12",
                _ if (32..=126).contains(&code) => {
                    // Printable ASCII range
                    std::str::from_utf8(&code_u8).unwrap_or("")
                }
                _ => "",
            })
            .to_string(),
        );
    } else {
        None
    }
}

#[test]
fn test_key_str_from_codes() {
    assert_eq!(
        key_str_from_codes(Some(65), None, None),
        Some("A".to_string())
    );
    assert_eq!(
        key_str_from_codes(None, Some(66), None),
        Some("B".to_string())
    );
    assert_eq!(
        key_str_from_codes(None, None, Some(67)),
        Some("C".to_string())
    );
    assert_eq!(
        key_str_from_codes(None, None, Some(13)),
        Some("enter".to_string())
    );
    assert_eq!(
        key_str_from_codes(None, None, Some(32)),
        Some("space".to_string())
    );
    assert_eq!(
        key_str_from_codes(None, None, Some(256)),
        Some("left".to_string())
    );
    assert_eq!(
        key_str_from_codes(None, None, Some(300)),
        Some("".to_string())
    );
    assert_eq!(key_str_from_codes(None, None, None), None);
}

/// Splits the events array string into individual event strings.
///
/// e.g. Given the events array string:
/// `[Object(InputEventKey, "test":false), Object(InputEventMouseButton, "test":false)]`
///
/// it returns:
///
/// `["Object(InputEventKey, \"test\":false)", "Object(InputEventMouseButton, \"test\":false)"]`
fn split_events_array(events: &str) -> Vec<String> {
    lazy_static! {
        static ref INPUT_OBJECT_REGEX: Regex =
            Regex::new(r#"Object\(((?:[^()]|\((?:[^()]|\([^()]*\))*\))*)\)"#).unwrap();
    };

    INPUT_OBJECT_REGEX
        .find_iter(events)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

#[test]
fn test_split_events_array() {
    let input = r#"[Object(InputEventKey,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"echo":false,"scancode":0,"physical_scancode":0,"pressed":false,"repeated":false,"script":null), Object(InputEventMouseButton,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"button_mask":0,"position":Vector2(0, 0),"global_position":Vector2(0, 0),"factor":1.0,"button_index":1,"canceled":false,"pressed":false,"double_click":false,"script":null)]"#;
    let expected = vec![
        r#"Object(InputEventKey,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"echo":false,"scancode":0,"physical_scancode":0,"pressed":false,"repeated":false,"script":null)"#,
        r#"Object(InputEventMouseButton,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"button_mask":0,"position":Vector2(0, 0),"global_position":Vector2(0, 0),"factor":1.0,"button_index":1,"canceled":false,"pressed":false,"double_click":false,"script":null)"#,
    ];
    assert_eq!(expected.len(), 2);
    for (i, event) in split_events_array(input).iter().enumerate() {
        assert_eq!(event, expected[i]);
    }
}

fn extract_input_event_from_string(event_str: &str) -> Option<InputEvent> {
    let maybe_split = event_str.trim_start_matches("Object(").split_once(',');
    if maybe_split.is_none() {
        return None;
    }

    let (event_type, properties_str) = maybe_split.unwrap();
    let event_type = event_type.trim();

    let properties = split_properties_string(properties_str);

    let mut bool_properties: HashMap<String, bool> = HashMap::new();
    let mut int_properties: HashMap<String, i32> = HashMap::new();
    let mut float_properties: HashMap<String, f32> = HashMap::new();
    let mut vec2_properties: HashMap<String, (f32, f32)> = HashMap::new();
    let mut str_properties: HashMap<String, String> = HashMap::new();

    for (key, value) in properties {
        if value == "true" || value == "false" {
            bool_properties.insert(key.to_string(), value == "true");
        } else if let Ok(int_value) = value.parse::<i32>() {
            int_properties.insert(key.to_string(), int_value);
        } else if let Ok(float_value) = value.parse::<f32>() {
            float_properties.insert(key.to_string(), float_value);
        } else if value.starts_with("Vector2(") && value.ends_with(')') {
            let vec_str = &value["Vector2(".len()..value.len() - 1];
            let parts: Vec<&str> = vec_str.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (
                    parts[0].trim().parse::<f32>(),
                    parts[1].trim().parse::<f32>(),
                ) {
                    vec2_properties.insert(key.to_string(), (x, y));
                }
            }
        } else if value.starts_with('"') && value.ends_with('"') {
            str_properties.insert(key.to_string(), value.trim_matches('"').to_string());
        } else if value == "null" {
            str_properties.insert(key.to_string(), "null".to_string());
        } else {
            str_properties.insert(key.to_string(), value.to_string());
        }
    }

    Some(InputEvent {
        event_type: event_type.to_string(),
        str_properties,
        bool_properties,
        int_properties,
        float_properties,
        vec2_properties,
    })
}

#[test]
fn test_extract_input_event_from_string() {
    let input = r#"Object(InputEventKey,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"echo":false,"scancode":0,"physical_scancode":0,"global_position":Vector2(0, 0),"pressed":false,"repeated":false,"factor":1.0,"script":null)"#;
    let event = extract_input_event_from_string(input).unwrap();
    assert_eq!(event.event_type, "InputEventKey");
    assert_eq!(
        event.bool_properties.get("resource_local_to_scene"),
        Some(&false)
    );
    assert_eq!(
        event.str_properties.get("resource_name"),
        Some(&"".to_string())
    );
    assert_eq!(event.int_properties.get("device"), Some(&-1));
    assert_eq!(event.float_properties.get("factor"), Some(&1.0));
    assert_eq!(
        event.vec2_properties.get("global_position"),
        Some(&(0.0, 0.0))
    );
}

fn split_properties_string(properties: &str) -> Vec<(&str, &str)> {
    let mut result = Vec::new();
    let mut in_quotes = false;
    let mut in_parentheses = 0;
    let mut last_split = 0;

    for (i, c) in properties.char_indices() {
        match c {
            '"' => in_quotes = !in_quotes,
            '(' => {
                if !in_quotes {
                    in_parentheses += 1;
                }
            }
            ')' => {
                if !in_quotes && in_parentheses > 0 {
                    in_parentheses -= 1;
                }
            }
            ',' => {
                if !in_quotes && in_parentheses == 0 {
                    let part = &properties[last_split..i];
                    if let Some((key, value)) = part.split_once(':') {
                        result.push((key.trim().trim_matches('"'), value.trim()));
                    }
                    last_split = i + 1;
                }
            }
            _ => {}
        }
    }

    // Add the last part
    if last_split < properties.len() {
        let part = &properties[last_split..];
        if let Some((key, value)) = part.split_once(':') {
            result.push((key.trim().trim_matches('"'), value.trim()));
        }
    }

    result
}

#[test]
fn test_split_properties_string() {
    let input = r#""resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"echo":false,"scancode":0,"physical_scancode":0,"pressed":false,"repeated":false,"factor":1.0,"script":null"#;
    let expected = vec![
        ("resource_local_to_scene", "false"),
        ("resource_name", "\"\""),
        ("device", "-1"),
        ("window_id", "0"),
        ("alt_pressed", "false"),
        ("shift_pressed", "false"),
        ("ctrl_pressed", "false"),
        ("meta_pressed", "false"),
        ("echo", "false"),
        ("scancode", "0"),
        ("physical_scancode", "0"),
        ("pressed", "false"),
        ("repeated", "false"),
        ("factor", "1.0"),
        ("script", "null"),
    ];
    let result = split_properties_string(input);
    assert_eq!(result.len(), expected.len());
    for (i, (key, value)) in result.iter().enumerate() {
        assert_eq!(*key, expected[i].0);
        assert_eq!(*value, expected[i].1);
    }
}
