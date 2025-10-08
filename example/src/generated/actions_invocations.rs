#![allow(dead_code)]
use godot::{builtin::StringName, classes::Input};

pub trait InputActionInvocations {
    fn is_fire_pressed(&self) -> bool;
    fn is_fire_just_pressed(&self) -> bool;
    fn is_fire_just_released(&self) -> bool;

    fn is_jump_pressed(&self) -> bool;
    fn is_jump_just_pressed(&self) -> bool;
    fn is_jump_just_released(&self) -> bool;

    fn is_move_down_pressed(&self) -> bool;
    fn is_move_down_just_pressed(&self) -> bool;
    fn is_move_down_just_released(&self) -> bool;

    fn is_move_left_pressed(&self) -> bool;
    fn is_move_left_just_pressed(&self) -> bool;
    fn is_move_left_just_released(&self) -> bool;

    fn is_move_right_pressed(&self) -> bool;
    fn is_move_right_just_pressed(&self) -> bool;
    fn is_move_right_just_released(&self) -> bool;

    fn is_move_up_pressed(&self) -> bool;
    fn is_move_up_just_pressed(&self) -> bool;
    fn is_move_up_just_released(&self) -> bool;
}

impl InputActionInvocations for Input {
    fn is_fire_pressed(&self) -> bool { self.is_action_pressed(StringName::from("Fire")) }
    fn is_fire_just_pressed(&self) -> bool { self.is_action_just_pressed(StringName::from("Fire")) }
    fn is_fire_just_released(&self) -> bool { self.is_action_just_released(StringName::from("Fire")) }

    fn is_jump_pressed(&self) -> bool { self.is_action_pressed(StringName::from("jump")) }
    fn is_jump_just_pressed(&self) -> bool { self.is_action_just_pressed(StringName::from("jump")) }
    fn is_jump_just_released(&self) -> bool { self.is_action_just_released(StringName::from("jump")) }

    fn is_move_down_pressed(&self) -> bool { self.is_action_pressed(StringName::from("move_down")) }
    fn is_move_down_just_pressed(&self) -> bool { self.is_action_just_pressed(StringName::from("move_down")) }
    fn is_move_down_just_released(&self) -> bool { self.is_action_just_released(StringName::from("move_down")) }

    fn is_move_left_pressed(&self) -> bool { self.is_action_pressed(StringName::from("move_left")) }
    fn is_move_left_just_pressed(&self) -> bool { self.is_action_just_pressed(StringName::from("move_left")) }
    fn is_move_left_just_released(&self) -> bool { self.is_action_just_released(StringName::from("move_left")) }

    fn is_move_right_pressed(&self) -> bool { self.is_action_pressed(StringName::from("move_right")) }
    fn is_move_right_just_pressed(&self) -> bool { self.is_action_just_pressed(StringName::from("move_right")) }
    fn is_move_right_just_released(&self) -> bool { self.is_action_just_released(StringName::from("move_right")) }

    fn is_move_up_pressed(&self) -> bool { self.is_action_pressed(StringName::from("move_up")) }
    fn is_move_up_just_pressed(&self) -> bool { self.is_action_just_pressed(StringName::from("move_up")) }
    fn is_move_up_just_released(&self) -> bool { self.is_action_just_released(StringName::from("move_up")) }
}