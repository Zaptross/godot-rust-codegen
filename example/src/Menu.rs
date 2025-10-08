// File included to test remote icon comment parsing

use godot::prelude::*;
use godot::{
    classes::{Control, IControl},
    obj::Base,
};

#[derive(GodotClass)]
#[class(init,base=Control)] // zgrcg:icon="res://icons/gd/Control.svg"
pub struct Menu {
    base: Base<Control>,
}

#[godot_api]
impl IControl for Menu {}
impl Menu {}
