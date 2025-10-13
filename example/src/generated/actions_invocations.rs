#![allow(dead_code)]
use godot::{builtin::StringName, classes::Input};

pub trait InputActionInvocations {
    /// Returns true while `left_click` or `J` is pressed
    fn is_fire_pressed(&self) -> bool;
    /// Returns true when `left_click` or `J` is just pressed
    fn is_fire_just_pressed(&self) -> bool;
    /// Returns true when `left_click` or `J` is just released
    fn is_fire_just_released(&self) -> bool;

    /// Returns true while `SPACE` is pressed
    fn is_jump_pressed(&self) -> bool;
    /// Returns true when `SPACE` is just pressed
    fn is_jump_just_pressed(&self) -> bool;
    /// Returns true when `SPACE` is just released
    fn is_jump_just_released(&self) -> bool;

    /// Returns true while `S` is pressed
    fn is_move_down_pressed(&self) -> bool;
    /// Returns true when `S` is just pressed
    fn is_move_down_just_pressed(&self) -> bool;
    /// Returns true when `S` is just released
    fn is_move_down_just_released(&self) -> bool;

    /// Returns true while `A` is pressed
    fn is_move_left_pressed(&self) -> bool;
    /// Returns true when `A` is just pressed
    fn is_move_left_just_pressed(&self) -> bool;
    /// Returns true when `A` is just released
    fn is_move_left_just_released(&self) -> bool;

    /// Returns true while `D` is pressed
    fn is_move_right_pressed(&self) -> bool;
    /// Returns true when `D` is just pressed
    fn is_move_right_just_pressed(&self) -> bool;
    /// Returns true when `D` is just released
    fn is_move_right_just_released(&self) -> bool;

    /// Returns true while `W` is pressed
    fn is_move_up_pressed(&self) -> bool;
    /// Returns true when `W` is just pressed
    fn is_move_up_just_pressed(&self) -> bool;
    /// Returns true when `W` is just released
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