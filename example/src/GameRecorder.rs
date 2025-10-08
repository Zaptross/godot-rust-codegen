// File included to test local icon comment parsing

use godot::prelude::*;
use godot::{
    classes::{INode, Node},
    obj::Base,
};

#[derive(GodotClass)]
#[class(init,base=Node)]
// zgrcg:icon="res://icons/local/godot-rust.svg"
pub struct GameRecorder {
    base: Base<Node>,
}

#[godot_api]
impl INode for GameRecorder {}
impl GameRecorder {}
