// Allow dead code because to better represent the structure of the file, even if some fields are not used.
#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct Gdextension<'a> {
    pub configuration: Option<ConfigurationSection<'a>>,
    pub libraries: Option<LibrariesSection<'a>>,
    pub icons: Option<IconsSection<'a>>,
    pub dependencies: Option<DependenciesSection<'a>>,
}

impl Gdextension<'_> {
    pub fn parse_from_str<'a>(content: &'a str) -> Gdextension<'a> {
        let mut gdextension = Gdextension::new();

        let sections = Self::split_sections(content);

        for section in sections {
            if let Some(config) = ConfigurationSection::parse(&section) {
                gdextension.configuration = Some(config);
            } else if let Some(libs) = LibrariesSection::parse(&section) {
                gdextension.libraries = Some(libs);
            } else if let Some(icons) = IconsSection::parse(&section) {
                gdextension.icons = Some(icons);
            } else if let Some(deps) = DependenciesSection::parse(&section) {
                gdextension.dependencies = Some(deps);
            }
        }

        gdextension
    }

    fn new() -> Self {
        Self {
            configuration: None,
            libraries: None,
            icons: None,
            dependencies: None,
        }
    }

    fn split_sections<'a>(file_content: &'a str) -> Vec<&'a str> {
        lazy_static! {
            static ref SECTION_REGEX: Regex = Regex::new(r"(?m)^\[(\w+)\]$").unwrap();
        }
        let mut sections = Vec::new();
        let mut last_index = 0;

        for cap in SECTION_REGEX.find_iter(file_content) {
            let start = cap.start();
            if last_index != 0 {
                sections.push(&file_content[last_index..start]);
            }
            last_index = start;
        }

        // Add the last section
        if last_index != 0 {
            sections.push(&file_content[last_index..]);
        }

        sections
    }
}

/// Configuration section of the `.gdextension` file
///
/// It has the following format:
/// ```text
/// [configuration]
/// entry_symbol="my_entry_symbol"
/// compatibility.minimum="4.0"
/// compatibility.maximum="4.5"
/// reloadable=false
/// android.aar_plugin=false
/// ```
pub struct ConfigurationSection<'a> {
    pub entry_symbol: Option<&'a str>,
    pub compatibility_minimum: Option<&'a str>,
    pub compatibility_maximum: Option<&'a str>,
    pub reloadable: Option<bool>,
    pub android_aar_plugin: Option<bool>,
}

impl ConfigurationSection<'_> {
    /// Parse a configuration section from `.gdextension` file content
    ///
    /// Returns `None` if the content doesn't start with `[configuration]` header.
    /// Parses key-value pairs and converts string values to appropriate types.
    ///
    /// # Example
    /// ```
    /// # struct ConfigurationSection<'a> {
    /// #     pub entry_symbol: Option<&'a str>,
    /// #     pub compatibility_minimum: Option<&'a str>,
    /// #     pub compatibility_maximum: Option<&'a str>,
    /// #     pub reloadable: Option<bool>,
    /// #     pub android_aar_plugin: Option<bool>,
    /// # }
    /// # impl ConfigurationSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<ConfigurationSection<'a>> {
    /// #         if !content.trim().starts_with("[configuration]") {
    /// #             return None;
    /// #         }
    /// #         let mut config = ConfigurationSection {
    /// #             entry_symbol: None,
    /// #             compatibility_minimum: None,
    /// #             compatibility_maximum: None,
    /// #             reloadable: None,
    /// #             android_aar_plugin: None,
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
    /// #                     "entry_symbol" => config.entry_symbol = Some(value),
    /// #                     "compatibility.minimum" => config.compatibility_minimum = Some(value),
    /// #                     "compatibility.maximum" => config.compatibility_maximum = Some(value),
    /// #                     "reloadable" => config.reloadable = Some(value.eq_ignore_ascii_case("true")),
    /// #                     "android.aar_plugin" => config.android_aar_plugin = Some(value.eq_ignore_ascii_case("true")),
    /// #                     _ => {}
    /// #                 }
    /// #             }
    /// #         }
    /// #         Some(config)
    /// #     }
    /// # }
    ///
    /// let content = r#"[configuration]
    /// entry_symbol="gdext_rust_init"
    /// compatibility.minimum="4.1"
    /// compatibility.maximum="4.2"
    /// reloadable=true
    /// android.aar_plugin=false
    /// "#;
    ///
    /// let config = ConfigurationSection::parse(content).unwrap();
    /// assert_eq!(config.entry_symbol, Some("gdext_rust_init"));
    /// assert_eq!(config.compatibility_minimum, Some("4.1"));
    /// assert_eq!(config.compatibility_maximum, Some("4.2"));
    /// assert_eq!(config.reloadable, Some(true));
    /// assert_eq!(config.android_aar_plugin, Some(false));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<ConfigurationSection<'a>> {
        // exit early if no configuration section header
        if !content.trim().starts_with("[configuration]") {
            return None;
        }

        let mut config = ConfigurationSection {
            entry_symbol: None,
            compatibility_minimum: None,
            compatibility_maximum: None,
            reloadable: None,
            android_aar_plugin: None,
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
                    "entry_symbol" => config.entry_symbol = Some(value),
                    "compatibility.minimum" => config.compatibility_minimum = Some(value),
                    "compatibility.maximum" => config.compatibility_maximum = Some(value),
                    "reloadable" => config.reloadable = Some(value.eq_ignore_ascii_case("true")),
                    "android.aar_plugin" => {
                        config.android_aar_plugin = Some(value.eq_ignore_ascii_case("true"))
                    }
                    _ => {}
                }
            }
        }

        Some(config)
    }

    /// Convert the ConfigurationSection back to a string representation
    ///
    /// Generates a properly formatted `[configuration]` section with all non-None fields.
    /// String values are quoted, boolean values are written as `true`/`false`.
    ///
    /// # Example
    /// ```
    /// # struct ConfigurationSection<'a> {
    /// #     pub entry_symbol: Option<&'a str>,
    /// #     pub compatibility_minimum: Option<&'a str>,
    /// #     pub compatibility_maximum: Option<&'a str>,
    /// #     pub reloadable: Option<bool>,
    /// #     pub android_aar_plugin: Option<bool>,
    /// # }
    /// # impl ConfigurationSection<'_> {
    /// #     pub fn to_string(&self) -> String {
    /// #         format!(
    /// #             "[configuration]{}{}{}{}{}\n",
    /// #             if let Some(entry_symbol) = self.entry_symbol {
    /// #                 format!("\nentry_symbol=\"{}\"", entry_symbol)
    /// #             } else {
    /// #                 "".to_string()
    /// #             },
    /// #             if let Some(compatibility_minimum) = self.compatibility_minimum {
    /// #                 format!("\ncompatibility.minimum=\"{}\"", compatibility_minimum)
    /// #             } else {
    /// #                 "".to_string()
    /// #             },
    /// #             if let Some(compatibility_maximum) = self.compatibility_maximum {
    /// #                 format!("\ncompatibility.maximum=\"{}\"", compatibility_maximum)
    /// #             } else {
    /// #                 "".to_string()
    /// #             },
    /// #             if let Some(reloadable) = self.reloadable {
    /// #                 format!("\nreloadable={}", if reloadable { "true" } else { "false" })
    /// #             } else {
    /// #                 "".to_string()
    /// #             },
    /// #             if let Some(android_aar_plugin) = self.android_aar_plugin {
    /// #                 format!("\nandroid.aar_plugin={}", if android_aar_plugin { "true" } else { "false" })
    /// #             } else {
    /// #                 "".to_string()
    /// #             },
    /// #         )
    /// #     }
    /// # }
    ///
    /// let config = ConfigurationSection {
    ///     entry_symbol: Some("gdext_rust_init"),
    ///     compatibility_minimum: Some("4.1"),
    ///     compatibility_maximum: None,
    ///     reloadable: Some(true),
    ///     android_aar_plugin: Some(false),
    /// };
    ///
    /// let output = config.to_string();
    /// assert!(output.contains("[configuration]"));
    /// assert!(output.contains("entry_symbol=\"gdext_rust_init\""));
    /// assert!(output.contains("compatibility.minimum=\"4.1\""));
    /// assert!(output.contains("reloadable=true"));
    /// assert!(output.contains("android.aar_plugin=false"));
    /// assert!(!output.contains("compatibility.maximum"));
    /// ```
    pub fn to_string(&self) -> String {
        format!(
            "[configuration]{}{}{}{}{}\n",
            if let Some(entry_symbol) = self.entry_symbol {
                format!("\nentry_symbol=\"{}\"", entry_symbol)
            } else {
                "".to_string()
            },
            if let Some(compatibility_minimum) = self.compatibility_minimum {
                format!("\ncompatibility.minimum=\"{}\"", compatibility_minimum)
            } else {
                "".to_string()
            },
            if let Some(compatibility_maximum) = self.compatibility_maximum {
                format!("\ncompatibility.maximum=\"{}\"", compatibility_maximum)
            } else {
                "".to_string()
            },
            if let Some(reloadable) = self.reloadable {
                format!("\nreloadable={}", if reloadable { "true" } else { "false" })
            } else {
                "".to_string()
            },
            if let Some(android_aar_plugin) = self.android_aar_plugin {
                format!(
                    "\nandroid.aar_plugin={}",
                    if android_aar_plugin { "true" } else { "false" }
                )
            } else {
                "".to_string()
            },
        )
    }
}

/// Libraries section of the `.gdextension` file
///
/// It has the following format:
/// ```text
/// [libraries]
/// windows.debug.x86_64="res://path/to/library.dll"
/// linux.release.x86_64="res://path/to/library.so"
/// ```
pub struct LibrariesSection<'a> {
    /// Map of library name to path
    ///
    /// e.g:
    ///
    /// ```windows.debug.x86_64="res://path/to/library.dll"```
    ///
    /// becomes:
    ///
    /// ```"windows.debug.x86_64"``` => ```"res://path/to/library.dll"```
    pub libraries: HashMap<&'a str, &'a str>,
}

impl LibrariesSection<'_> {
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # pub struct LibrariesSection<'a> {
    /// #     pub libraries: std::collections::HashMap<&'a str, &'a str>,
    /// # }
    /// # impl LibrariesSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<LibrariesSection<'a>> {
    /// #         if !content.trim().starts_with("[libraries]") {
    /// #             return None;
    /// #         }
    /// #         let mut libraries = HashMap::new();
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') {
    /// #                 continue;
    /// #             }
    /// #             if let Some((key, value)) = line.split_once('=') {
    /// #                 let key = key.trim();
    /// #                 let value = value.trim().trim_matches('"');
    /// #                 libraries.insert(key, value);
    /// #             }
    /// #         }
    /// #         Some(LibrariesSection { libraries })
    /// #     }
    /// # }
    ///
    /// let content = r#"[libraries]
    /// windows.debug.x86_64="res://path/to/library.dll"
    /// linux.release.x86_64="res://path/to/library.so"
    /// "#;
    ///
    /// let libraries = LibrariesSection::parse(content).unwrap();
    /// assert_eq!(libraries.libraries.get("windows.debug.x86_64"), Some(&"res://path/to/library.dll"));
    /// assert_eq!(libraries.libraries.get("linux.release.x86_64"), Some(&"res://path/to/library.so"));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<LibrariesSection<'a>> {
        // exit early if no libraries section header
        if !content.trim().starts_with("[libraries]") {
            return None;
        }

        let mut libraries = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                libraries.insert(key, value);
            }
        }

        Some(LibrariesSection { libraries })
    }

    /// Convert the LibrariesSection back to a string representation
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # pub struct LibrariesSection<'a> {
    /// #     pub libraries: std::collections::HashMap<&'a str, &'a str>,
    /// # }
    /// # impl LibrariesSection<'_> {
    /// #     pub fn to_string(&self) -> String {
    /// #         let mut result = String::from("[libraries]\n");
    /// #         for (key, value) in &self.libraries {
    /// #             result.push_str(&format!("{}=\"{}\"\n", key, value));
    /// #         }
    /// #         result
    /// #     }
    /// # }
    /// let mut libraries_map = HashMap::new();
    /// libraries_map.insert("windows.debug.x86_64", "res://path/to/library.dll");
    /// libraries_map.insert("linux.release.x86_64", "res://path/to/library.so");
    /// let libraries = LibrariesSection { libraries: libraries_map };
    /// let output = libraries.to_string();
    /// assert!(output.contains("[libraries]"));
    /// assert!(output.contains("windows.debug.x86_64=\"res://path/to/library.dll\""));
    /// assert!(output.contains("linux.release.x86_64=\"res://path/to/library.so\""));
    /// ```
    pub fn to_string(&self) -> String {
        let mut result = String::from("[libraries]\n");
        for (key, value) in &self.libraries {
            result.push_str(&format!("{}=\"{}\"\n", key, value));
        }
        result
    }
}

/// Icons section of the `.gdextension` file
///
/// # Example format:
/// ```text
/// [icons]
/// MyClass="res://path/to/icon.png"
/// MyOtherClass="res://path/to/other_icon.png"
/// ```
pub struct IconsSection<'a> {
    pub name: &'a str,
    /// Map of icon name to path
    ///
    /// e.g: to set a custom icon for a rust class ```My_Class```
    ///
    /// ```MyClass="res://path/to/icon.png"```
    ///
    /// becomes:
    ///
    /// ```"MyClass"``` => ```"res://path/to/icon.png"```
    pub icons: HashMap<&'a str, &'a str>,
}

impl IconsSection<'_> {
    /// Parse an icons section from `.gdextension` file content
    ///
    /// Returns `None` if the content doesn't start with `[icons]` header.
    /// Parses key-value pairs where keys are class names and values are icon paths.
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # struct IconsSection<'a> {
    /// #     pub name: &'a str,
    /// #     pub icons: HashMap<&'a str, &'a str>,
    /// # }
    /// # impl IconsSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<IconsSection<'a>> {
    /// #         if !content.trim().starts_with("[icons]") {
    /// #             return None;
    /// #         }
    /// #         let mut icons = HashMap::new();
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
    /// #                 continue;
    /// #             }
    /// #             if let Some((key, value)) = line.split_once('=') {
    /// #                 let key = key.trim();
    /// #                 let value = value.trim().trim_matches('"');
    /// #                 icons.insert(key, value);
    /// #             }
    /// #         }
    /// #         Some(IconsSection { name: "icons", icons })
    /// #     }
    /// # }
    ///
    /// let content = r#"[icons]
    /// MyClass="res://path/to/icon.png"
    /// MyOtherClass="res://path/to/other_icon.png"
    /// AnotherClass="res://icons/another.svg"
    /// "#;
    ///
    /// let icons_section = IconsSection::parse(content).unwrap();
    /// assert_eq!(icons_section.name, "icons");
    /// assert_eq!(icons_section.icons.get("MyClass"), Some(&"res://path/to/icon.png"));
    /// assert_eq!(icons_section.icons.get("MyOtherClass"), Some(&"res://path/to/other_icon.png"));
    /// assert_eq!(icons_section.icons.get("AnotherClass"), Some(&"res://icons/another.svg"));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<IconsSection<'a>> {
        // exit early if no icons section header
        if !content.trim().starts_with("[icons]") {
            return None;
        }

        let mut icons = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                icons.insert(key, value);
            }
        }

        Some(IconsSection {
            name: "icons",
            icons,
        })
    }

    /// Convert the IconsSection back to a string representation
    ///
    /// Generates a properly formatted `[icons]` section with all icon mappings.
    /// Class names and icon paths are properly quoted.
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # struct IconsSection<'a> {
    /// #     pub name: &'a str,
    /// #     pub icons: HashMap<&'a str, &'a str>,
    /// # }
    /// # impl IconsSection<'_> {
    /// #     pub fn to_string(&self) -> String {
    /// #         let mut result = String::from("[icons]\n");
    /// #         for (key, value) in &self.icons {
    /// #             result.push_str(&format!("{}=\"{}\"\n", key, value));
    /// #         }
    /// #         result
    /// #     }
    /// # }
    ///
    /// let mut icons_map = HashMap::new();
    /// icons_map.insert("MyClass", "res://path/to/icon.png");
    /// icons_map.insert("MyOtherClass", "res://path/to/other_icon.png");
    /// let icons_section = IconsSection { name: "icons", icons: icons_map };
    ///
    /// let output = icons_section.to_string();
    /// assert!(output.contains("[icons]"));
    /// assert!(output.contains("MyClass=\"res://path/to/icon.png\""));
    /// assert!(output.contains("MyOtherClass=\"res://path/to/other_icon.png\""));
    /// ```
    pub fn to_string(&self) -> String {
        let mut result = String::from("[icons]\n");
        for (key, value) in &self.icons {
            result.push_str(&format!("{}=\"{}\"\n", key, value));
        }
        result
    }
}

/// Dependencies section of the `.gdextension` file
///
/// It has the following format:
/// ```text
/// [dependencies]
/// macos.release = {
///     "res://bin/libdependency.macos.template_release.framework" : "Contents/Frameworks"
/// }
/// windows.debug = {
///     "res://bin/libdependency.windows.template_debug.x86_64.dll" : "",
///     "res://bin/libdependency.windows.template_debug.x86_32.dll" : ""
/// }
/// ```
pub struct DependenciesSection<'a> {
    pub dependencies: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}

impl DependenciesSection<'_> {
    /// Parse a dependencies section from `.gdextension` file content
    ///
    /// Returns `None` if the content doesn't start with `[dependencies]` header.
    /// Parses platform-specific dependency mappings in a nested structure.
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # struct DependenciesSection<'a> {
    /// #     pub dependencies: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    /// # }
    /// # impl DependenciesSection<'_> {
    /// #     pub fn parse<'a>(content: &'a str) -> Option<DependenciesSection<'a>> {
    /// #         if !content.trim().starts_with("[dependencies]") {
    /// #             return None;
    /// #         }
    /// #         let mut dependencies = HashMap::new();
    /// #         let mut current_platform = None;
    /// #         let mut in_block = false;
    /// #         for line in content.lines() {
    /// #             let line = line.trim();
    /// #             if line.is_empty() || line.starts_with('#') {
    /// #                 continue;
    /// #             }
    /// #             if line.starts_with('[') && line != "[dependencies]" {
    /// #                 break;
    /// #             }
    /// #             if line.contains(" = {") {
    /// #                 let platform = line.split(" = {").next().unwrap().trim();
    /// #                 current_platform = Some(platform);
    /// #                 dependencies.insert(platform, HashMap::new());
    /// #                 in_block = true;
    /// #             } else if line == "}" {
    /// #                 in_block = false;
    /// #                 current_platform = None;
    /// #             } else if in_block && line.contains(" : ") {
    /// #                 if let Some(platform) = current_platform {
    /// #                     let parts: Vec<&str> = line.split(" : ").collect();
    /// #                     if parts.len() == 2 {
    /// #                         let key = parts[0].trim().trim_matches('"').trim_matches(',');
    /// #                         let value = parts[1].trim().trim_matches('"').trim_matches(',');
    /// #                         if let Some(platform_deps) = dependencies.get_mut(platform) {
    /// #                             platform_deps.insert(key, value);
    /// #                         }
    /// #                     }
    /// #                 }
    /// #             }
    /// #         }
    /// #         Some(DependenciesSection { dependencies })
    /// #     }
    /// # }
    ///
    /// let content = r#"[dependencies]
    /// macos.release = {
    ///     "res://bin/libdependency.macos.template_release.framework" : "Contents/Frameworks"
    /// }
    /// windows.debug = {
    ///     "res://bin/libdependency.windows.template_debug.x86_64.dll" : "",
    ///     "res://bin/libdependency.windows.template_debug.x86_32.dll" : ""
    /// }
    /// "#;
    ///
    /// let deps_section = DependenciesSection::parse(content).unwrap();
    /// assert!(deps_section.dependencies.contains_key("macos.release"));
    /// assert!(deps_section.dependencies.contains_key("windows.debug"));
    /// let macos_deps = deps_section.dependencies.get("macos.release").unwrap();
    /// assert_eq!(macos_deps.get("res://bin/libdependency.macos.template_release.framework"), Some(&"Contents/Frameworks"));
    /// let windows_deps = deps_section.dependencies.get("windows.debug").unwrap();
    /// assert_eq!(windows_deps.get("res://bin/libdependency.windows.template_debug.x86_64.dll"), Some(&""));
    /// ```
    pub fn parse<'a>(content: &'a str) -> Option<DependenciesSection<'a>> {
        // exit early if no dependencies section header
        if !content.trim().starts_with("[dependencies]") {
            return None;
        }

        let mut dependencies = HashMap::new();
        let mut current_platform = None;
        let mut in_block = false;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Stop if we encounter another section
            if line.starts_with('[') && line != "[dependencies]" {
                break;
            }

            if line.contains(" = {") {
                // Start of a platform block
                let platform = line.split(" = {").next().unwrap().trim();
                current_platform = Some(platform);
                dependencies.insert(platform, HashMap::new());
                in_block = true;
            } else if line == "}" {
                // End of a platform block
                in_block = false;
                current_platform = None;
            } else if in_block && line.contains(" : ") {
                // Dependency entry within a platform block
                if let Some(platform) = current_platform {
                    let parts: Vec<&str> = line.split(" : ").collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim().trim_matches('"').trim_matches(',');
                        let value = parts[1].trim().trim_matches('"').trim_matches(',');
                        if let Some(platform_deps) = dependencies.get_mut(platform) {
                            platform_deps.insert(key, value);
                        }
                    }
                }
            }
        }

        Some(DependenciesSection { dependencies })
    }

    /// Convert the DependenciesSection back to a string representation
    ///
    /// Generates a properly formatted `[dependencies]` section with nested platform blocks.
    /// Each platform contains its dependency mappings with proper indentation and formatting.
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// # struct DependenciesSection<'a> {
    /// #     pub dependencies: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    /// # }
    /// # impl DependenciesSection<'_> {
    /// #     pub fn to_string(&self) -> String {
    /// #         let mut result = String::from("[dependencies]\n");
    /// #         for (platform, deps) in &self.dependencies {
    /// #             result.push_str(&format!("{} = {{\n", platform));
    /// #             for (key, value) in deps {
    /// #                 if value.is_empty() {
    /// #                     result.push_str(&format!("    \"{}\" : \"\"\n", key));
    /// #                 } else {
    /// #                     result.push_str(&format!("    \"{}\" : \"{}\"\n", key, value));
    /// #                 }
    /// #             }
    /// #             result.push_str("}\n");
    /// #         }
    /// #         result
    /// #     }
    /// # }
    ///
    /// let mut dependencies_map = HashMap::new();
    /// let mut macos_deps = HashMap::new();
    /// macos_deps.insert("res://bin/libdependency.macos.template_release.framework", "Contents/Frameworks");
    /// dependencies_map.insert("macos.release", macos_deps);
    ///
    /// let mut windows_deps = HashMap::new();
    /// windows_deps.insert("res://bin/libdependency.windows.template_debug.x86_64.dll", "");
    /// dependencies_map.insert("windows.debug", windows_deps);
    ///
    /// let deps_section = DependenciesSection { dependencies: dependencies_map };
    /// let output = deps_section.to_string();
    /// assert!(output.contains("[dependencies]"));
    /// assert!(output.contains("macos.release = {"));
    /// assert!(output.contains("windows.debug = {"));
    /// assert!(output.contains("\"res://bin/libdependency.macos.template_release.framework\" : \"Contents/Frameworks\""));
    /// assert!(output.contains("\"res://bin/libdependency.windows.template_debug.x86_64.dll\" : \"\""));
    /// ```
    pub fn to_string(&self) -> String {
        let mut result = String::from("[dependencies]\n");
        for (platform, deps) in &self.dependencies {
            result.push_str(&format!("{} = {{\n", platform));
            for (key, value) in deps {
                if value.is_empty() {
                    result.push_str(&format!("    \"{}\" : \"\"\n", key));
                } else {
                    result.push_str(&format!("    \"{}\" : \"{}\"\n", key, value));
                }
            }
            result.push_str("}\n");
        }
        result
    }
}
