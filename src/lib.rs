use crate::{
    icon_comment::apply_icons_from_comments, mod_file::write_mod_file, projectgodot::ProjectGodot,
};
use std::{collections::HashMap, fs, path::Path};

mod gdextension;
mod icon_comment;
mod input_actions;
mod layers;
mod mod_file;
mod projectgodot;

pub struct Generator {
    /// Path to output generated files to.
    output_dir: String,
    /// Default: false
    output_dir_valid: bool,
    /// Path to the `.gdextension` file to parse for icon comments.
    gdextension_path: String,
    /// Default: false
    gdextension_path_valid: bool,
    /// Path to the `project.godot` file to parse for action and layer constants.
    project_godot_path: String,
    /// Default: false
    project_godot_path_valid: bool,
    /// Path to the Rust source files. Typically `./src`.
    source_path: String,
    /// Default: true
    source_path_valid: bool,
    /// Path to the godot res:// root. Typically `../godot`.
    resource_path: String,
    /// Default: true
    resource_path_valid: bool,

    validation_errors: Vec<String>,

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
            output_dir_valid: false,
            gdextension_path: "".into(),
            gdextension_path_valid: false,
            project_godot_path: "".into(),
            project_godot_path_valid: false,
            source_path: "./src".into(),
            source_path_valid: true,
            resource_path: "../godot".into(),
            resource_path_valid: true,
            validation_errors: vec![],
            icon_sources: HashMap::new(),
            layer_consts: false,
            action_consts: false,
            action_invocations: false,
            icon_comments: false,
        }
    }

    pub fn generate(&self) {
        if !self.validation_errors.is_empty() {
            for err in &self.validation_errors {
                println!("cargo::error={}", err);
            }
            return;
        }

        let mut _project_godot_content: String = String::new();
        let mut project: Option<ProjectGodot> = None;
        let mut modules: Vec<String> = vec![];

        if self.project_godot_path_valid {
            let file_read = fs::read_to_string(&self.project_godot_path);

            match file_read {
                Ok(content) => {
                    // by assigning to a higher scoped variable, we ensure the string lives long enough for the ProjectGodot struct to reference it
                    _project_godot_content = content;
                    project = Some(ProjectGodot::parse_from_str(
                        _project_godot_content.as_str(),
                    ));
                }
                Err(e) => {
                    println!("cargo::error=Failed to read project.godot: {}", e);
                    return;
                }
            }
        }

        if self.action_either_valid() {
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
            println!("cargo:rerun-if-changed={}", self.project_godot_path);
        }

        if self.icon_comments_valid() {
            apply_icons_from_comments(
                &self.source_path,
                &self.resource_path,
                &self.gdextension_path,
                &self.icon_sources,
            );
            println!("cargo:rerun-if-changed={}", self.gdextension_path);
        }

        if self.layer_consts_valid() {
            if let Some(proj) = &project {
                layers::generate_layers_consts(&self.output_dir, proj)
                    .iter()
                    .for_each(|m| modules.push(m.to_string()));
            }
            println!("cargo:rerun-if-changed={}", self.project_godot_path);
        }

        if !modules.is_empty() {
            write_mod_file(&self.output_dir, modules);
        }
    }

    /// Supply the output directory for the generated files.
    /// Creates the directory if it does not exist.
    pub fn set_output_dir(mut self, path: &str) -> Self {
        self.output_dir = path.to_string();
        self.output_dir_valid = true;

        if self.output_dir.is_empty() {
            self.validation_errors
                .push("Output directory must be set, and cannot be empty".into());
            self.output_dir_valid = false;
        }

        if !Path::new(&self.output_dir).exists() {
            if let Err(_e) = fs::create_dir_all(&self.output_dir) {
                self.validation_errors
                    .push("Failed to create output directory".into());
            }
            self.output_dir_valid = false;
        }

        self
    }

    /// Supply the path to the `.gdextension` file to enable generation of action and layer constants.
    pub fn set_gdextension_path(mut self, path: &str) -> Self {
        self.gdextension_path = path.to_string();
        self.gdextension_path_valid = true;

        if self.gdextension_path.is_empty() {
            self.validation_errors
                .push("gdextension path must be set with `set_gdextension_path`".into());
            self.gdextension_path_valid = false;
        }

        if !Path::new(&self.gdextension_path).exists() {
            self.validation_errors.push(format!(
                "gdextension path does not exist: {}",
                self.gdextension_path
            ));
            self.gdextension_path_valid = false;
        }

        self
    }

    /// Supply the path to the `project.godot` file to enable generation of action and layer constants.
    pub fn set_project_godot_path(mut self, path: &str) -> Self {
        self.project_godot_path = path.to_string();
        self.project_godot_path_valid = true;

        if self.project_godot_path.is_empty() {
            self.validation_errors
                .push("project.godot path must be set with `set_project_godot_path`".into());
            self.project_godot_path_valid = false;
        }

        if !Path::new(&self.project_godot_path).exists() {
            self.validation_errors.push(format!(
                "project.godot path does not exist: {}",
                self.project_godot_path
            ));
            self.project_godot_path_valid = false;
        }

        self
    }

    /// Supply the path to the Rust source files. Defaults to `./src`.
    pub fn set_source_path(mut self, path: &str) -> Self {
        self.source_path = path.to_string();

        if self.source_path.is_empty() {
            self.validation_errors
                .push("Source path must be set with `set_source_path`".into());
            self.source_path_valid = false;
            return self;
        }

        if !Path::new(&self.source_path).exists() {
            self.validation_errors.push(format!(
                "Source path does not exist: {}{}",
                self.source_path,
                if self.source_path.eq("./src") {
                    " (default, change with `set_source_path`)"
                } else {
                    ""
                }
            ));
            self.source_path_valid = false;
        }

        self
    }

    /// Supply the location of the godot `res://` root.
    pub fn set_resource_path(mut self, path: &str) -> Self {
        self.resource_path = path.to_string();

        if self.resource_path.is_empty() {
            self.validation_errors
                .push("Resource path must be set with `set_resource_path`".into());
            self.resource_path_valid = false;
            return self;
        }

        if !Path::new(&self.resource_path).exists() {
            self.validation_errors.push(format!(
                "Resource path does not exist: {}{}",
                self.resource_path,
                if self.resource_path.eq("../godot") {
                    " (default, change with `set_resource_path`)"
                } else {
                    ""
                }
            ));
            self.resource_path_valid = false;
        }

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
        if local_path.is_empty() || icon_path.is_empty() {
            self.validation_errors
                .push("Icon source paths must be non-empty strings".into());
        } else {
            self.icon_sources
                .insert(local_path.to_string(), icon_path.to_string());
        }

        self
    }

    /*
        because we can't guarantee the order of builder calls, we have to allow enabling features even if the paths aren't set yet,
        and then check requirements in generate()
    */

    /// Enable generation of layer constants from `project.godot`.
    pub fn output_layer_consts(mut self) -> Self {
        self.layer_consts = true;
        self
    }
    fn layer_consts_valid(&self) -> bool {
        self.layer_consts && self.project_godot_path_valid
    }

    /// Enable generation of action constants from `project.godot`.
    ///
    /// e.g. for the action `MoveLeft` in Godot, a function `MOVE_LEFT()` will be generated, returning `StringName("MoveLeft")`.
    pub fn output_action_consts(mut self) -> Self {
        self.action_consts = true;
        self
    }

    // applies to both action_consts and action_invocations
    fn action_either_valid(&self) -> bool {
        self.action_invocations && self.project_godot_path_valid
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
    fn icon_comments_valid(&self) -> bool {
        self.icon_comments
            && self.gdextension_path_valid
            && self.source_path_valid
            && self.resource_path_valid
    }
}
