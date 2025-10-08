#![allow(dead_code)]
use godot::builtin::StringName;

pub fn FIRE() -> StringName { StringName::from("Fire") }
pub fn JUMP() -> StringName { StringName::from("jump") }
pub fn MOVE_DOWN() -> StringName { StringName::from("move_down") }
pub fn MOVE_LEFT() -> StringName { StringName::from("move_left") }
pub fn MOVE_RIGHT() -> StringName { StringName::from("move_right") }
pub fn MOVE_UP() -> StringName { StringName::from("move_up") }