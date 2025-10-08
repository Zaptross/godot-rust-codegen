// Allow dead code because to better represent the structure of the file, even if some fields are not used.
#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Parsed representation of a `project.godot` file
pub struct ProjectGodot<'a> {
    pub config_version: Option<u32>,
    pub application: Option<ApplicationSection<'a>>,
    pub autoload: Option<AutoloadSection<'a>>,
    pub dotnet: Option<DotnetSection<'a>>,
    pub input: Option<InputSection<'a>>,
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
pub struct InputSection<'a> {
    pub inputs: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}

impl InputSection<'_> {
    /// Parse an input section from `project.godot` file content
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # pub struct InputSection<'a> {
    /// #     pub inputs: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    /// # }
    /// # impl InputSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<InputSection<'a>> {
    /// #         if !content.trim().starts_with("[input]") {
    /// #             return None;
    /// #         }
    /// #         let mut inputs = HashMap::new();
    /// #         let mut current_input = None;
    /// #         let mut in_block = false;
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') {
    /// #                 continue;
    /// #             }
    /// #             if line.starts_with('[') && line != "[input]" {
    /// #                 break;
    /// #             }
    /// #             if line.contains("={") {
    /// #                 let input_name = line.split("={").next().unwrap().trim();
    /// #                 current_input = Some(input_name);
    /// #                 inputs.insert(input_name, HashMap::new());
    /// #                 in_block = true;
    /// #             } else if line == "}" {
    /// #                 in_block = false;
    /// #                 current_input = None;
    /// #             } else if in_block && line.contains(": ") {
    /// #                 if let Some(input_name) = current_input {
    /// #                     let parts: Vec<&str> = line.splitn(2, ": ").collect();
    /// #                     if parts.len() == 2 {
    /// #                         let key = parts[0].trim().trim_matches('"').trim_matches(',');
    /// #                         let value = parts[1].trim().trim_matches('"').trim_matches(',');
    /// #                         if let Some(input_map) = inputs.get_mut(input_name) {
    /// #                             input_map.insert(key, value);
    /// #                         }
    /// #                     }
    /// #                 }
    /// #             }
    /// #         }
    /// #         Some(InputSection { inputs })
    /// #     }
    /// # }
    ///
    /// let content = r#"[input]
    /// Fire={
    /// "deadzone": 0.5,
    /// "events": [Object(InputEventMouseButton,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":false,"ctrl_pressed":false,"meta_pressed":false,"button_mask":0,"position":Vector2(0, 0),"global_position":Vector2(0, 0),"factor":1.0,"button_index":1,"canceled":false,"pressed":false,"double_click":false,"script":null)
    /// ]
    /// }
    /// "#;
    ///
    /// let input_section = InputSection::parse(content).unwrap();
    /// let fire_input = input_section.inputs.get("Fire").unwrap();
    /// assert_eq!(fire_input.get("deadzone"), Some(&"0.5"));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<InputSection<'a>> {
        if !content.trim().starts_with("[input]") {
            return None;
        }
        let mut inputs = HashMap::new();
        let mut current_input = None;
        let mut in_block = false;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line != "[input]" {
                break;
            }
            if line.contains("={") {
                let input_name = line.split("={").next().unwrap().trim();
                current_input = Some(input_name);
                inputs.insert(input_name, HashMap::new());
                in_block = true;
            } else if line == "}" {
                in_block = false;
                current_input = None;
            } else if in_block && line.contains(": ") {
                if let Some(input_name) = current_input {
                    let parts: Vec<&str> = line.splitn(2, ": ").collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim().trim_matches('"').trim_matches(',');
                        let value = parts[1].trim().trim_matches('"').trim_matches(',');
                        if let Some(input_map) = inputs.get_mut(input_name) {
                            input_map.insert(key, value);
                        }
                    }
                }
            }
        }
        Some(InputSection { inputs })
    }
}
