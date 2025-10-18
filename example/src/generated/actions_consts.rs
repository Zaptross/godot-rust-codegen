#![allow(dead_code)]
#![allow(non_snake_case)]
use godot::builtin::StringName;

/// Maps to: `left_click` or `J`
pub fn FIRE() -> StringName { StringName::from("Fire") }
/// Maps to: `left_click` or `J`
pub const FIRE_STR: &'static str = "Fire";
/// Maps to: `SPACE`
pub fn JUMP() -> StringName { StringName::from("jump") }
/// Maps to: `SPACE`
pub const JUMP_STR: &'static str = "jump";
/// Maps to: `S`
pub fn MOVE_DOWN() -> StringName { StringName::from("move_down") }
/// Maps to: `S`
pub const MOVE_DOWN_STR: &'static str = "move_down";
/// Maps to: `A`
pub fn MOVE_LEFT() -> StringName { StringName::from("move_left") }
/// Maps to: `A`
pub const MOVE_LEFT_STR: &'static str = "move_left";
/// Maps to: `D`
pub fn MOVE_RIGHT() -> StringName { StringName::from("move_right") }
/// Maps to: `D`
pub const MOVE_RIGHT_STR: &'static str = "move_right";
/// Maps to: `W`
pub fn MOVE_UP() -> StringName { StringName::from("move_up") }
/// Maps to: `W`
pub const MOVE_UP_STR: &'static str = "move_up";