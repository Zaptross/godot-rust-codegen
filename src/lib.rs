use crate::{
    icon_comment::apply_icons_from_comments, mod_file::write_mod_file, projectgodot::ProjectGodot,
};
use std::{collections::HashMap, fs};

mod gdextension;
mod icon_comment;
mod input_actions;
mod layers;
mod mod_file;
mod projectgodot;

pub struct Generator {
    /// Path to output generated files to.
    output_dir: String,
    /// Path to the `.gdextension` file to parse for icon comments.
    gdextension_path: String,
    /// Path to the `project.godot` file to parse for action and layer constants.
    project_godot_path: String,
    /// Path to the Rust source files. Typically `./src`.
    source_path: String,
    /// Path to the godot res:// root. Typically `../godot`.
    resource_path: String,

    icon_sources: HashMap<String, String>,
    layer_consts: bool,
    action_consts: bool,
    action_invocations: bool,
    icon_comments: bool,
}

impl Generator {
    pub fn builder() -> Self {
        Self {
            output_dir: "".into(),
            gdextension_path: "".into(),
            project_godot_path: "".into(),
            source_path: "./src".into(),
            resource_path: "../godot".into(),
            icon_sources: HashMap::new(),
            layer_consts: false,
            action_consts: false,
            action_invocations: false,
            icon_comments: false,
        }
    }

    pub fn generate(&self) {
        let mut project: Option<ProjectGodot> = None;
        let mut _project_content: Option<String> = None;
        let mut modules: Vec<String> = vec![];

        if self.output_dir.is_empty() {
            println!("cargo::error=Output directory must be set, use `set_output_dir`");
        }
        if self.action_consts || self.layer_consts {
            if self.project_godot_path.is_empty() {
                println!(
                    "cargo::error=project.godot path must be set with `set_project_godot_path`"
                );
            }

            let file_read = fs::read_to_string(&self.project_godot_path);

            match file_read {
                Ok(content) => {
                    // by assigning to a higher scoped variable, we ensure the string lives long enough for the ProjectGodot struct to reference it
                    _project_content = Some(content);
                    project = Some(ProjectGodot::parse_from_str(
                        _project_content.as_ref().unwrap().as_str(),
                    ));
                }
                Err(e) => {
                    println!("cargo::error=Failed to read project.godot: {}", e);
                    return;
                }
            }

            if self.action_consts {
                if let Some(proj) = &project {
                    input_actions::generate_actions(
                        &self.output_dir,
                        self.action_consts,
                        self.action_invocations,
                        proj,
                    )
                    .iter()
                    .for_each(|m| modules.push(m.to_string()));
                }
            }

            println!("cargo:rerun-if-changed={}", self.project_godot_path);
        }
        if self.icon_comments {
            if self.gdextension_path.is_empty() {
                println!("cargo::error=GDExtension path must be set with `set_gdextension_path`");
                return;
            }
            if !std::path::Path::new(&self.source_path).exists() {
                println!(
                    "cargo::error=Source path does not exist: {}{}",
                    self.source_path,
                    if self.source_path.eq("./src") {
                        " (default, change with `set_source_path`)"
                    } else {
                        ""
                    }
                );
                return;
            }
            if !std::path::Path::new(&self.resource_path).exists() {
                println!(
                    "cargo::error=Resource path does not exist: {}{}",
                    self.resource_path,
                    if self.resource_path.eq("../godot") {
                        " (default, change with `set_resource_path`)"
                    } else {
                        ""
                    }
                );
                return;
            }

            apply_icons_from_comments(
                &self.source_path,
                &self.resource_path,
                &self.gdextension_path,
                &self.icon_sources,
            );

            println!("cargo:rerun-if-changed={}", self.gdextension_path);
        }
        if self.layer_consts {
            if let Some(proj) = &project {
                layers::generate_layers_consts(&self.output_dir, proj)
                    .iter()
                    .for_each(|m| modules.push(m.to_string()));
            }

            println!("cargo:rerun-if-changed={}", self.project_godot_path);
        }

        write_mod_file(&self.output_dir, modules);
    }

    /// Supply the output directory for the generated files.
    pub fn set_output_dir(mut self, path: &str) -> Self {
        self.output_dir = path.to_string();
        self
    }

    /// Supply the path to the `.gdextension` file to enable generation of action and layer constants.
    pub fn set_gdextension_path(mut self, path: &str) -> Self {
        self.gdextension_path = path.to_string();
        self
    }

    /// Supply the path to the `project.godot` file to enable generation of action and layer constants.
    pub fn set_project_godot_path(mut self, path: &str) -> Self {
        self.project_godot_path = path.to_string();
        self
    }

    /// Supply the path to the Rust source files. Defaults to `./src`.
    pub fn set_source_path(mut self, path: &str) -> Self {
        self.source_path = path.to_string();
        self
    }

    /// Supply the location of the godot res:// root.
    pub fn set_resource_path(mut self, path: &str) -> Self {
        self.resource_path = path.to_string();
        self
    }

    /// Add a source for icons, mapping a local path to a remote URL.
    ///
    /// `local_path` should be the path as used in Godot, e.g. `res://icons/gd/`. Entries are matched by this prefix. Entries are overwritten in order by this prefix.
    ///
    /// `icon_path` should be a directory path or URL where the icon can be found.
    ///
    /// e.g. `icon_path`: `./icons/` would look for `res://icons/gd/icon.png` at `./icons/icon.png`.
    pub fn add_icon_source(mut self, local_path: &str, icon_path: &str) -> Self {
        self.icon_sources
            .insert(local_path.to_string(), icon_path.to_string());
        self
    }

    /// Enable generation of layer constants from `project.godot`.
    pub fn output_layer_consts(mut self) -> Self {
        self.layer_consts = true;
        self
    }

    /// Enable generation of action constants from `project.godot`.
    ///
    /// e.g. for the action `MoveLeft` in Godot, a function `MOVE_LEFT()` will be generated, returning `StringName("MoveLeft")`.
    pub fn output_action_consts(mut self) -> Self {
        self.action_consts = true;
        self
    }

    /// Enable generation of action invocation traits from `project.godot`.
    ///
    /// e.g. for the action `MoveLeft` in Godot, the `Input` struct will be extended with the methods `is_move_left_pressed()`, `is_move_left_just_pressed()`, and `is_move_left_just_released()` which return booleans.
    pub fn output_action_invocations(mut self) -> Self {
        self.action_invocations = true;
        self
    }

    /// Enable parsing of icon comments from source files and applying them to the .gdextension file.
    ///
    /// e.g. a comment like `// zgrcg:icon="res://icons/gd/Control.svg"` above a struct definition will set the icon for that class in the .gdextension file to the specified icon.
    pub fn output_icon_comments(mut self) -> Self {
        self.icon_comments = true;
        self
    }
}
