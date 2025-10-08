// File included to test icon comment parsing

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
impl IControl for Menu {
    // fn init(base: Base<Control>) -> Self {
    //  Self {
    //   base
    //  }
    // }
}

impl Menu {}
