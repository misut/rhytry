use crate::interface::keyboard::Keyboard;

pub enum Key {
    Escape,
    LeftDon(Keyboard),
    RightDon(Keyboard),
    LeftKat(Keyboard),
    RightKat(Keyboard),
}
