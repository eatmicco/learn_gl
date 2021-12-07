pub enum KeycodeEx {
    None,
    LeftArrow,
    UpArrow,
    DownArrow,
    RightArrow
}

pub struct Keyboard(pub KeycodeEx);

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard(KeycodeEx::None)
    }
}