// File included to test remote icon comment parsing

use godot::prelude::*;
use godot::{
    classes::{Control, IControl},
    obj::Base,
};

use crate::generated::scene_actions::SceneActions;

#[derive(GodotClass)]
#[class(init,base=Control)] // zgrcg:icon="res://icons/gd/Control.svg"
pub struct Menu {
    base: Base<Control>,
}

#[godot_api]
impl IControl for Menu {}

#[godot_api]
impl Menu {
    #[func]
    pub fn on_open_multiplayer(&self) {
        self.to_gd()
            .upcast::<Node>()
            .change_scene_to_multiplayer_main();
    }
}
