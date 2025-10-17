#![allow(dead_code)]
use godot::{
    prelude::Node,
    global::Error
};

pub trait SceneActions {
    fn change_scene_to(&self, scene_path: &str) -> Option<Error>;
    /// `res://scenes/LevelOne.tscn`
    fn change_scene_to_level_one(&self) -> Option<Error>;
    /// `res://scenes/Main.tscn`
    fn change_scene_to_main(&self) -> Option<Error>;
    /// `res://scenes/multiplayer/Main.tscn`
    fn change_scene_to_multiplayer_main(&self) -> Option<Error>;
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

    fn change_scene_to_level_one(&self) -> Option<Error> { self.change_scene_to("res://scenes/LevelOne.tscn") }
    fn change_scene_to_main(&self) -> Option<Error> { self.change_scene_to("res://scenes/Main.tscn") }
    fn change_scene_to_multiplayer_main(&self) -> Option<Error> { self.change_scene_to("res://scenes/multiplayer/Main.tscn") }
}