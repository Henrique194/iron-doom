use bevy::input::keyboard::{Key, KeyCode, KeyboardInput, NativeKey, NativeKeyCode};
use bevy::input::ButtonState;
use bevy::prelude::Entity;
use sdl2::keyboard::{Keycode as SdlKeycode, Scancode as SdlScancode};

pub fn convert_keyboard_event(
    keycode: Option<SdlKeycode>,
    scancode: Option<SdlScancode>,
    button_state: ButtonState,
) -> KeyboardInput {
    KeyboardInput {
        key_code: convert_scan_code(scancode),
        state: button_state,
        logical_key: convert_key_code(keycode),
        window: Entity::PLACEHOLDER,
    }
}

fn convert_scan_code(scancode: Option<SdlScancode>) -> KeyCode {
    let Some(scan_code) = scancode else {
        return KeyCode::Unidentified(NativeKeyCode::Unidentified);
    };

    match scan_code {
        SdlScancode::A => KeyCode::KeyA,
        SdlScancode::B => KeyCode::KeyB,
        SdlScancode::C => KeyCode::KeyC,
        SdlScancode::D => KeyCode::KeyD,
        SdlScancode::E => KeyCode::KeyE,
        SdlScancode::F => KeyCode::KeyF,
        SdlScancode::G => KeyCode::KeyG,
        SdlScancode::H => KeyCode::KeyH,
        SdlScancode::I => KeyCode::KeyI,
        SdlScancode::J => KeyCode::KeyJ,
        SdlScancode::K => KeyCode::KeyK,
        SdlScancode::L => KeyCode::KeyL,
        SdlScancode::M => KeyCode::KeyM,
        SdlScancode::N => KeyCode::KeyN,
        SdlScancode::O => KeyCode::KeyO,
        SdlScancode::P => KeyCode::KeyP,
        SdlScancode::Q => KeyCode::KeyQ,
        SdlScancode::R => KeyCode::KeyR,
        SdlScancode::S => KeyCode::KeyS,
        SdlScancode::T => KeyCode::KeyT,
        SdlScancode::U => KeyCode::KeyU,
        SdlScancode::V => KeyCode::KeyV,
        SdlScancode::W => KeyCode::KeyW,
        SdlScancode::X => KeyCode::KeyX,
        SdlScancode::Y => KeyCode::KeyY,
        SdlScancode::Z => KeyCode::KeyZ,
        SdlScancode::Num1 => KeyCode::Digit1,
        SdlScancode::Num2 => KeyCode::Digit2,
        SdlScancode::Num3 => KeyCode::Digit3,
        SdlScancode::Num4 => KeyCode::Digit4,
        SdlScancode::Num5 => KeyCode::Digit5,
        SdlScancode::Num6 => KeyCode::Digit6,
        SdlScancode::Num7 => KeyCode::Digit7,
        SdlScancode::Num8 => KeyCode::Digit8,
        SdlScancode::Num9 => KeyCode::Digit9,
        SdlScancode::Num0 => KeyCode::Digit0,
        SdlScancode::Return => KeyCode::Enter,
        SdlScancode::Escape => KeyCode::Escape,
        SdlScancode::Backspace => KeyCode::Backspace,
        SdlScancode::Tab => KeyCode::Tab,
        SdlScancode::Space => KeyCode::Space,
        SdlScancode::Minus => KeyCode::Minus,
        SdlScancode::Equals => KeyCode::Equal,
        SdlScancode::LeftBracket => KeyCode::BracketLeft,
        SdlScancode::RightBracket => KeyCode::BracketRight,
        SdlScancode::Backslash => KeyCode::Backslash,
        SdlScancode::Semicolon => KeyCode::Semicolon,
        SdlScancode::Apostrophe => KeyCode::Quote,
        SdlScancode::Grave => KeyCode::Backquote,
        SdlScancode::Comma => KeyCode::Comma,
        SdlScancode::Period => KeyCode::Period,
        SdlScancode::Slash => KeyCode::Slash,
        SdlScancode::CapsLock => KeyCode::CapsLock,
        SdlScancode::F1 => KeyCode::F1,
        SdlScancode::F2 => KeyCode::F2,
        SdlScancode::F3 => KeyCode::F3,
        SdlScancode::F4 => KeyCode::F4,
        SdlScancode::F5 => KeyCode::F5,
        SdlScancode::F6 => KeyCode::F6,
        SdlScancode::F7 => KeyCode::F7,
        SdlScancode::F8 => KeyCode::F8,
        SdlScancode::F9 => KeyCode::F9,
        SdlScancode::F10 => KeyCode::F10,
        SdlScancode::F11 => KeyCode::F11,
        SdlScancode::F12 => KeyCode::F12,
        SdlScancode::PrintScreen => KeyCode::PrintScreen,
        SdlScancode::ScrollLock => KeyCode::ScrollLock,
        SdlScancode::Pause => KeyCode::Pause,
        SdlScancode::Insert => KeyCode::Insert,
        SdlScancode::Home => KeyCode::Home,
        SdlScancode::PageUp => KeyCode::PageUp,
        SdlScancode::Delete => KeyCode::Delete,
        SdlScancode::End => KeyCode::End,
        SdlScancode::PageDown => KeyCode::PageDown,
        SdlScancode::Right => KeyCode::ArrowRight,
        SdlScancode::Left => KeyCode::ArrowLeft,
        SdlScancode::Down => KeyCode::ArrowDown,
        SdlScancode::Up => KeyCode::ArrowUp,
        SdlScancode::NumLockClear => KeyCode::NumLock,
        SdlScancode::KpDivide => KeyCode::NumpadDivide,
        SdlScancode::KpMultiply => KeyCode::NumpadMultiply,
        SdlScancode::KpMinus => KeyCode::NumpadSubtract,
        SdlScancode::KpPlus => KeyCode::NumpadAdd,
        SdlScancode::KpEnter => KeyCode::NumpadEnter,
        SdlScancode::Kp1 => KeyCode::Numpad1,
        SdlScancode::Kp2 => KeyCode::Numpad2,
        SdlScancode::Kp3 => KeyCode::Numpad3,
        SdlScancode::Kp4 => KeyCode::Numpad4,
        SdlScancode::Kp5 => KeyCode::Numpad5,
        SdlScancode::Kp6 => KeyCode::Numpad6,
        SdlScancode::Kp7 => KeyCode::Numpad7,
        SdlScancode::Kp8 => KeyCode::Numpad8,
        SdlScancode::Kp9 => KeyCode::Numpad9,
        SdlScancode::Kp0 => KeyCode::Numpad0,
        SdlScancode::KpPeriod => KeyCode::NumpadDecimal,
        SdlScancode::KpEquals => KeyCode::NumpadEqual,
        SdlScancode::RCtrl => KeyCode::ControlRight,
        SdlScancode::LCtrl => KeyCode::ControlLeft,
        SdlScancode::RShift => KeyCode::ShiftRight,
        SdlScancode::LShift => KeyCode::ShiftLeft,
        SdlScancode::RAlt => KeyCode::AltRight,
        SdlScancode::LAlt => KeyCode::AltLeft,
        _ => KeyCode::Unidentified(NativeKeyCode::Unidentified),
    }
}

fn convert_key_code(keycode: Option<SdlKeycode>) -> Key {
    let Some(key_code) = keycode else {
        return Key::Unidentified(NativeKey::Unidentified);
    };

    match key_code {
        SdlKeycode::Backspace => Key::Backspace,
        SdlKeycode::Tab => Key::Tab,
        SdlKeycode::Return => Key::Enter,
        SdlKeycode::Escape => Key::Escape,
        SdlKeycode::Space => Key::Space,
        SdlKeycode::Exclaim => Key::Character("!".into()),
        SdlKeycode::Quotedbl => Key::Character("\"".into()),
        SdlKeycode::Hash => Key::Character("#".into()),
        SdlKeycode::Dollar => Key::Character("$".into()),
        SdlKeycode::Percent => Key::Character("%".into()),
        SdlKeycode::Ampersand => Key::Character("&".into()),
        SdlKeycode::Quote => Key::Character("'".into()),
        SdlKeycode::LeftParen => Key::Character("(".into()),
        SdlKeycode::RightParen => Key::Character(")".into()),
        SdlKeycode::Asterisk => Key::Character("*".into()),
        SdlKeycode::Plus => Key::Character("+".into()),
        SdlKeycode::Comma => Key::Character(",".into()),
        SdlKeycode::Minus => Key::Character("-".into()),
        SdlKeycode::Period => Key::Character(".".into()),
        SdlKeycode::Slash => Key::Character("/".into()),
        SdlKeycode::Num0 => Key::Character("0".into()),
        SdlKeycode::Num1 => Key::Character("1".into()),
        SdlKeycode::Num2 => Key::Character("2".into()),
        SdlKeycode::Num3 => Key::Character("3".into()),
        SdlKeycode::Num4 => Key::Character("4".into()),
        SdlKeycode::Num5 => Key::Character("5".into()),
        SdlKeycode::Num6 => Key::Character("6".into()),
        SdlKeycode::Num7 => Key::Character("7".into()),
        SdlKeycode::Num8 => Key::Character("8".into()),
        SdlKeycode::Num9 => Key::Character("9".into()),
        SdlKeycode::Colon => Key::Character(":".into()),
        SdlKeycode::Semicolon => Key::Character(";".into()),
        SdlKeycode::Less => Key::Character("<".into()),
        SdlKeycode::Equals => Key::Character("=".into()),
        SdlKeycode::Greater => Key::Character(">".into()),
        SdlKeycode::Question => Key::Character("?".into()),
        SdlKeycode::At => Key::Character("@".into()),
        SdlKeycode::LeftBracket => Key::Character("[".into()),
        SdlKeycode::Backslash => Key::Character("\\".into()),
        SdlKeycode::RightBracket => Key::Character("]".into()),
        SdlKeycode::Caret => Key::Dead(Some('^')),
        SdlKeycode::Underscore => Key::Character("_".into()),
        SdlKeycode::Backquote => Key::Dead(Some('`')),
        SdlKeycode::A => Key::Character("a".into()),
        SdlKeycode::B => Key::Character("b".into()),
        SdlKeycode::C => Key::Character("c".into()),
        SdlKeycode::D => Key::Character("d".into()),
        SdlKeycode::E => Key::Character("e".into()),
        SdlKeycode::F => Key::Character("f".into()),
        SdlKeycode::G => Key::Character("g".into()),
        SdlKeycode::H => Key::Character("h".into()),
        SdlKeycode::I => Key::Character("i".into()),
        SdlKeycode::J => Key::Character("j".into()),
        SdlKeycode::K => Key::Character("k".into()),
        SdlKeycode::L => Key::Character("l".into()),
        SdlKeycode::M => Key::Character("m".into()),
        SdlKeycode::N => Key::Character("n".into()),
        SdlKeycode::O => Key::Character("o".into()),
        SdlKeycode::P => Key::Character("p".into()),
        SdlKeycode::Q => Key::Character("q".into()),
        SdlKeycode::R => Key::Character("r".into()),
        SdlKeycode::S => Key::Character("s".into()),
        SdlKeycode::T => Key::Character("t".into()),
        SdlKeycode::U => Key::Character("u".into()),
        SdlKeycode::V => Key::Character("v".into()),
        SdlKeycode::W => Key::Character("w".into()),
        SdlKeycode::X => Key::Character("x".into()),
        SdlKeycode::Y => Key::Character("y".into()),
        SdlKeycode::Z => Key::Character("z".into()),
        SdlKeycode::Delete => Key::Delete,
        SdlKeycode::RCtrl => Key::Control,
        SdlKeycode::LCtrl => Key::Control,
        SdlKeycode::RShift => Key::Shift,
        SdlKeycode::LShift => Key::Shift,
        SdlKeycode::RAlt => Key::Alt,
        SdlKeycode::LAlt => Key::Alt,
        _ => Key::Unidentified(NativeKey::Unidentified),
    }
}
