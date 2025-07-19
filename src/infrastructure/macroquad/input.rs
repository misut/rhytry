use crate::interface::input::Input;
use crate::interface::key::Key;
use crate::interface::keyboard::Keyboard;

use macroquad::prelude::*;

impl Keyboard {
    fn key_code(&self) -> KeyCode {
        match self {
            Keyboard::D => KeyCode::D,
            Keyboard::F => KeyCode::F,
            Keyboard::J => KeyCode::J,
            Keyboard::L => KeyCode::L,
        }
    }
}

impl Key {
    fn key_code(&self) -> KeyCode {
        match self {
            Key::Escape => KeyCode::Escape,
            Key::LeftDon(kb) => kb.key_code(),
            Key::RightDon(kb) => kb.key_code(),
            Key::LeftKat(kb) => kb.key_code(),
            Key::RightKat(kb) => kb.key_code(),
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
