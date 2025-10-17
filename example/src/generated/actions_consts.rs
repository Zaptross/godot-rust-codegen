#![allow(dead_code)]
#![allow(non_snake_case)]
use godot::builtin::StringName;

/// Maps to: `left_click` or `J`
pub fn FIRE() -> StringName { StringName::from("Fire") }
/// Maps to: `SPACE`
pub fn JUMP() -> StringName { StringName::from("jump") }
/// Maps to: `S`
pub fn MOVE_DOWN() -> StringName { StringName::from("move_down") }
/// Maps to: `A`
pub fn MOVE_LEFT() -> StringName { StringName::from("move_left") }
/// Maps to: `D`
pub fn MOVE_RIGHT() -> StringName { StringName::from("move_right") }
/// Maps to: `W`
pub fn MOVE_UP() -> StringName { StringName::from("move_up") }