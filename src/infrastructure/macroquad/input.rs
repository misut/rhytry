use crate::interface::input::Input;
use crate::interface::key::Key;

use macroquad::prelude::*;

impl Key {
    fn key_code(&self) -> KeyCode {
        match self {
            Key::Escape => KeyCode::Escape,
            Key::LeftDon => KeyCode::F,
            Key::RightDon => KeyCode::J,
            Key::LeftKat => KeyCode::D,
            Key::RightKat => KeyCode::K,
        }
    }
}

impl Input for Key {
    fn down(&self) -> bool {
        is_key_down(self.key_code())
    }

    fn pressed(&self) -> bool {
        is_key_pressed(self.key_code())
    }

    fn released(&self) -> bool {
        is_key_released(self.key_code())
    }
}
