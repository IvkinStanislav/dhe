use evdev::{EventType, InputEvent, InputEventKind};

use super::KeyboardError;

/// The keyboard state is expressed as a 128-bit bitmap.
/// If the key number N is pressed, then the Nth element of the array has the value 1.
/// The key number is determined by the result of the `Key::number` function.
#[derive(Default, PartialEq, Eq, Hash)]
pub struct KeyboardState {
    value: u128,
}

impl KeyboardState {
    /// Apply all specified keys to the state as if they were pressed.
    pub fn apply_keys(&mut self, keys: &[Key]) {
        for &key in keys {
            self.apply_event(KeyEvent {
                key,
                position: KeyPosition::Press,
            })
        }
    }

    /// Apply all specified events to state.
    pub fn apply_events(&mut self, events: &[KeyEvent]) {
        events.iter().for_each(|&event| self.apply_event(event));
    }

    /// Apply the specified event to the state.
    fn apply_event(&mut self, event: KeyEvent) {
        let KeyEvent { key, position } = event;

        match position {
            KeyPosition::Press => {
                self.value |= 1_u128.rotate_left(key.number());
            }
            KeyPosition::Release => {
                self.value &= (!1_u128).rotate_left(key.number());
            }
        };
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumIter,
    strum::EnumString,
    strum::Display,
)]
#[repr(u8)]
pub enum Key {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    K0,

    Enter,
    Escape,
    BackSpace,
    Tab,
    Space,
    Minus,
    Equal,
    LBrace,
    RBrace,
    Backslash,
    Semicolon,
    Apostrophe,
    Grave,
    Comma,
    Dot,
    Slash,
    CapsLock,
    LCtrl,
    LShift,
    LAlt,
    LWin,
    RCtrl,
    RShift,
    RAlt,
    RWin,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,

    NumLock,
    Keypadslash,
    KeypadAsterisk,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadDot,
}

impl Key {
    fn number(self) -> u32 {
        self as u32
    }
}

impl From<Key> for evdev::Key {
    fn from(key: Key) -> Self {
        use Key::*;
        match key {
            A => evdev::Key::KEY_A,
            B => evdev::Key::KEY_B,
            C => evdev::Key::KEY_C,
            D => evdev::Key::KEY_D,
            E => evdev::Key::KEY_E,
            F => evdev::Key::KEY_F,
            G => evdev::Key::KEY_G,
            H => evdev::Key::KEY_H,
            I => evdev::Key::KEY_I,
            J => evdev::Key::KEY_J,
            K => evdev::Key::KEY_K,
            L => evdev::Key::KEY_L,
            M => evdev::Key::KEY_M,
            N => evdev::Key::KEY_N,
            O => evdev::Key::KEY_O,
            P => evdev::Key::KEY_P,
            Q => evdev::Key::KEY_Q,
            R => evdev::Key::KEY_R,
            S => evdev::Key::KEY_S,
            T => evdev::Key::KEY_T,
            U => evdev::Key::KEY_U,
            V => evdev::Key::KEY_V,
            W => evdev::Key::KEY_W,
            X => evdev::Key::KEY_X,
            Y => evdev::Key::KEY_Y,
            Z => evdev::Key::KEY_Z,

            K1 => evdev::Key::KEY_1,
            K2 => evdev::Key::KEY_2,
            K3 => evdev::Key::KEY_3,
            K4 => evdev::Key::KEY_4,
            K5 => evdev::Key::KEY_5,
            K6 => evdev::Key::KEY_6,
            K7 => evdev::Key::KEY_7,
            K8 => evdev::Key::KEY_8,
            K9 => evdev::Key::KEY_9,
            K0 => evdev::Key::KEY_0,

            Enter => evdev::Key::KEY_ENTER,
            Escape => evdev::Key::KEY_ESC,
            BackSpace => evdev::Key::KEY_BACKSPACE,
            Tab => evdev::Key::KEY_TAB,
            Space => evdev::Key::KEY_SPACE,
            Minus => evdev::Key::KEY_MINUS,
            Equal => evdev::Key::KEY_EQUAL,
            LBrace => evdev::Key::KEY_LEFTBRACE,
            RBrace => evdev::Key::KEY_RIGHTBRACE,
            Backslash => evdev::Key::KEY_BACKSLASH,
            Semicolon => evdev::Key::KEY_SEMICOLON,
            Apostrophe => evdev::Key::KEY_APOSTROPHE,
            Grave => evdev::Key::KEY_GRAVE,
            Comma => evdev::Key::KEY_COMMA,
            Dot => evdev::Key::KEY_DOT,
            Slash => evdev::Key::KEY_SLASH,
            CapsLock => evdev::Key::KEY_CAPSLOCK,
            LCtrl => evdev::Key::KEY_LEFTCTRL,
            LShift => evdev::Key::KEY_LEFTSHIFT,
            LAlt => evdev::Key::KEY_LEFTALT,
            LWin => evdev::Key::KEY_LEFTMETA,
            RCtrl => evdev::Key::KEY_RIGHTCTRL,
            RShift => evdev::Key::KEY_RIGHTSHIFT,
            RAlt => evdev::Key::KEY_RIGHTALT,
            RWin => evdev::Key::KEY_RIGHTMETA,

            F1 => evdev::Key::KEY_F1,
            F2 => evdev::Key::KEY_F2,
            F3 => evdev::Key::KEY_F3,
            F4 => evdev::Key::KEY_F4,
            F5 => evdev::Key::KEY_F5,
            F6 => evdev::Key::KEY_F6,
            F7 => evdev::Key::KEY_F7,
            F8 => evdev::Key::KEY_F8,
            F9 => evdev::Key::KEY_F9,
            F10 => evdev::Key::KEY_F10,
            F11 => evdev::Key::KEY_F11,
            F12 => evdev::Key::KEY_F12,
            PrintScreen => evdev::Key::KEY_SCREENSAVER,
            ScrollLock => evdev::Key::KEY_SCROLLLOCK,
            Pause => evdev::Key::KEY_PAUSE,
            Insert => evdev::Key::KEY_INSERT,
            Home => evdev::Key::KEY_HOME,
            PageUp => evdev::Key::KEY_PAGEUP,
            Delete => evdev::Key::KEY_DELETE,
            End => evdev::Key::KEY_END,
            PageDown => evdev::Key::KEY_PAGEDOWN,
            Right => evdev::Key::KEY_RIGHT,
            Left => evdev::Key::KEY_LEFT,
            Down => evdev::Key::KEY_DOWN,
            Up => evdev::Key::KEY_UP,

            NumLock => evdev::Key::KEY_NUMLOCK,
            Keypadslash => evdev::Key::KEY_KPSLASH,
            KeypadAsterisk => evdev::Key::KEY_KPASTERISK,
            KeypadMinus => evdev::Key::KEY_KPMINUS,
            KeypadPlus => evdev::Key::KEY_KPPLUS,
            KeypadEnter => evdev::Key::KEY_KPENTER,
            Keypad1 => evdev::Key::KEY_KP1,
            Keypad2 => evdev::Key::KEY_KP2,
            Keypad3 => evdev::Key::KEY_KP3,
            Keypad4 => evdev::Key::KEY_KP4,
            Keypad5 => evdev::Key::KEY_KP5,
            Keypad6 => evdev::Key::KEY_KP6,
            Keypad7 => evdev::Key::KEY_KP7,
            Keypad8 => evdev::Key::KEY_KP8,
            Keypad9 => evdev::Key::KEY_KP9,
            Keypad0 => evdev::Key::KEY_KP0,
            KeypadDot => evdev::Key::KEY_KPDOT,
        }
    }
}

impl TryFrom<evdev::Key> for Key {
    type Error = KeyboardError;
    fn try_from(key: evdev::Key) -> Result<Self, Self::Error> {
        use Key::*;
        let key = match key {
            evdev::Key::KEY_A => A,
            evdev::Key::KEY_B => B,
            evdev::Key::KEY_C => C,
            evdev::Key::KEY_D => D,
            evdev::Key::KEY_E => E,
            evdev::Key::KEY_F => F,
            evdev::Key::KEY_G => G,
            evdev::Key::KEY_H => H,
            evdev::Key::KEY_I => I,
            evdev::Key::KEY_J => J,
            evdev::Key::KEY_K => K,
            evdev::Key::KEY_L => L,
            evdev::Key::KEY_M => M,
            evdev::Key::KEY_N => N,
            evdev::Key::KEY_O => O,
            evdev::Key::KEY_P => P,
            evdev::Key::KEY_Q => Q,
            evdev::Key::KEY_R => R,
            evdev::Key::KEY_S => S,
            evdev::Key::KEY_T => T,
            evdev::Key::KEY_U => U,
            evdev::Key::KEY_V => V,
            evdev::Key::KEY_W => W,
            evdev::Key::KEY_X => X,
            evdev::Key::KEY_Y => Y,
            evdev::Key::KEY_Z => Z,

            evdev::Key::KEY_1 => K1,
            evdev::Key::KEY_2 => K2,
            evdev::Key::KEY_3 => K3,
            evdev::Key::KEY_4 => K4,
            evdev::Key::KEY_5 => K5,
            evdev::Key::KEY_6 => K6,
            evdev::Key::KEY_7 => K7,
            evdev::Key::KEY_8 => K8,
            evdev::Key::KEY_9 => K9,
            evdev::Key::KEY_0 => K0,

            evdev::Key::KEY_ENTER => Enter,
            evdev::Key::KEY_ESC => Escape,
            evdev::Key::KEY_BACKSPACE => BackSpace,
            evdev::Key::KEY_TAB => Tab,
            evdev::Key::KEY_SPACE => Space,
            evdev::Key::KEY_MINUS => Minus,
            evdev::Key::KEY_EQUAL => Equal,
            evdev::Key::KEY_LEFTBRACE => LBrace,
            evdev::Key::KEY_RIGHTBRACE => RBrace,
            evdev::Key::KEY_BACKSLASH => Backslash,
            evdev::Key::KEY_SEMICOLON => Semicolon,
            evdev::Key::KEY_APOSTROPHE => Apostrophe,
            evdev::Key::KEY_GRAVE => Grave,
            evdev::Key::KEY_COMMA => Comma,
            evdev::Key::KEY_DOT => Dot,
            evdev::Key::KEY_SLASH => Slash,
            evdev::Key::KEY_CAPSLOCK => CapsLock,
            evdev::Key::KEY_LEFTCTRL => LCtrl,
            evdev::Key::KEY_LEFTSHIFT => LShift,
            evdev::Key::KEY_LEFTALT => LAlt,
            evdev::Key::KEY_LEFTMETA => LWin,
            evdev::Key::KEY_RIGHTCTRL => RCtrl,
            evdev::Key::KEY_RIGHTSHIFT => RShift,
            evdev::Key::KEY_RIGHTALT => RAlt,
            evdev::Key::KEY_RIGHTMETA => RWin,

            evdev::Key::KEY_F1 => F1,
            evdev::Key::KEY_F2 => F2,
            evdev::Key::KEY_F3 => F3,
            evdev::Key::KEY_F4 => F4,
            evdev::Key::KEY_F5 => F5,
            evdev::Key::KEY_F6 => F6,
            evdev::Key::KEY_F7 => F7,
            evdev::Key::KEY_F8 => F8,
            evdev::Key::KEY_F9 => F9,
            evdev::Key::KEY_F10 => F10,
            evdev::Key::KEY_F11 => F11,
            evdev::Key::KEY_F12 => F12,
            evdev::Key::KEY_SCREENSAVER => PrintScreen,
            evdev::Key::KEY_SCROLLLOCK => ScrollLock,
            evdev::Key::KEY_PAUSE => Pause,
            evdev::Key::KEY_INSERT => Insert,
            evdev::Key::KEY_HOME => Home,
            evdev::Key::KEY_PAGEUP => PageUp,
            evdev::Key::KEY_DELETE => Delete,
            evdev::Key::KEY_END => End,
            evdev::Key::KEY_PAGEDOWN => PageDown,
            evdev::Key::KEY_RIGHT => Right,
            evdev::Key::KEY_LEFT => Left,
            evdev::Key::KEY_DOWN => Down,
            evdev::Key::KEY_UP => Up,

            evdev::Key::KEY_NUMLOCK => NumLock,
            evdev::Key::KEY_KPSLASH => Keypadslash,
            evdev::Key::KEY_KPASTERISK => KeypadAsterisk,
            evdev::Key::KEY_KPMINUS => KeypadMinus,
            evdev::Key::KEY_KPPLUS => KeypadPlus,
            evdev::Key::KEY_KPENTER => KeypadEnter,
            evdev::Key::KEY_KP1 => Keypad1,
            evdev::Key::KEY_KP2 => Keypad2,
            evdev::Key::KEY_KP3 => Keypad3,
            evdev::Key::KEY_KP4 => Keypad4,
            evdev::Key::KEY_KP5 => Keypad5,
            evdev::Key::KEY_KP6 => Keypad6,
            evdev::Key::KEY_KP7 => Keypad7,
            evdev::Key::KEY_KP8 => Keypad8,
            evdev::Key::KEY_KP9 => Keypad9,
            evdev::Key::KEY_KP0 => Keypad0,
            evdev::Key::KEY_KPDOT => KeypadDot,
            key => return Err(KeyboardError::KeyNotSupported(key.code().to_string())),
        };
        Ok(key)
    }
}

/// Keyboard key position.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyPosition {
    Press,
    Release,
}

impl From<KeyPosition> for i32 {
    fn from(position: KeyPosition) -> i32 {
        use KeyPosition::*;
        match position {
            Release => 0,
            Press => 1,
        }
    }
}

impl TryFrom<i32> for KeyPosition {
    type Error = KeyboardError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use KeyPosition::*;
        let position = match value {
            0 => Release,
            1 => Press,
            value => return Err(KeyboardError::KeyPositionNotSupported(value.to_string())),
        };
        Ok(position)
    }
}

/// Keyboard event about key state change.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEvent {
    pub key: Key,
    pub position: KeyPosition,
}

impl From<KeyEvent> for InputEvent {
    fn from(event: KeyEvent) -> Self {
        let KeyEvent { key, position } = event;
        InputEvent::new(
            EventType::KEY,
            evdev::Key::from(key).code(),
            i32::from(position),
        )
    }
}

impl TryFrom<InputEvent> for KeyEvent {
    type Error = KeyboardError;
    fn try_from(event: InputEvent) -> Result<Self, Self::Error> {
        let key: Key = match event.kind() {
            InputEventKind::Key(key) => key.try_into(),
            kind => Err(KeyboardError::InputEventKindNotSupported(format!(
                "{kind:?}"
            ))),
        }?;
        let position: KeyPosition = event.value().try_into()?;

        Ok(Self { key, position })
    }
}
