use super::window_interaction::{
    StorybookWindowState, TextAreaKey, TextInputKey, apply_text_area_key, apply_text_input_key,
};
use minifb::{Key, KeyRepeat, Window};

pub(super) fn apply_keyboard(window: &Window, state: &mut StorybookWindowState) -> bool {
    let shifted = window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift);
    let mut changed = false;
    for key in window.get_keys_pressed(KeyRepeat::Yes) {
        if state.selected_page == "text-area"
            && let Some(input) = text_area_key(key, shifted)
        {
            changed |= apply_text_area_key(state, input);
            continue;
        }
        if let Some(input) = text_input_key(key, shifted) {
            changed |= apply_text_input_key(state, input);
        }
    }
    changed
}

fn text_area_key(key: Key, shifted: bool) -> Option<TextAreaKey> {
    if key == Key::Backspace {
        return Some(TextAreaKey::Backspace);
    }
    if key == Key::Enter || key == Key::NumPadEnter {
        return if shifted {
            Some(TextAreaKey::Newline)
        } else {
            Some(TextAreaKey::Submit)
        };
    }
    character_for_key(key, shifted).map(TextAreaKey::Character)
}

fn text_input_key(key: Key, shifted: bool) -> Option<TextInputKey> {
    if key == Key::Backspace {
        return Some(TextInputKey::Backspace);
    }
    if key == Key::Enter || key == Key::NumPadEnter {
        return Some(TextInputKey::Submit);
    }
    character_for_key(key, shifted).map(TextInputKey::Character)
}

fn character_for_key(key: Key, shifted: bool) -> Option<char> {
    letter_for_key(key, shifted)
        .or_else(|| digit_for_key(key, shifted))
        .or_else(|| symbol_for_key(key, shifted))
}

fn letter_for_key(key: Key, shifted: bool) -> Option<char> {
    let character = match key {
        Key::A => 'a',
        Key::B => 'b',
        Key::C => 'c',
        Key::D => 'd',
        Key::E => 'e',
        Key::F => 'f',
        Key::G => 'g',
        Key::H => 'h',
        Key::I => 'i',
        Key::J => 'j',
        Key::K => 'k',
        Key::L => 'l',
        Key::M => 'm',
        Key::N => 'n',
        Key::O => 'o',
        Key::P => 'p',
        Key::Q => 'q',
        Key::R => 'r',
        Key::S => 's',
        Key::T => 't',
        Key::U => 'u',
        Key::V => 'v',
        Key::W => 'w',
        Key::X => 'x',
        Key::Y => 'y',
        Key::Z => 'z',
        _ => return None,
    };
    Some(if shifted {
        character.to_ascii_uppercase()
    } else {
        character
    })
}

fn digit_for_key(key: Key, shifted: bool) -> Option<char> {
    match (key, shifted) {
        (Key::Key0 | Key::NumPad0, false) => Some('0'),
        (Key::Key1 | Key::NumPad1, false) => Some('1'),
        (Key::Key2 | Key::NumPad2, false) => Some('2'),
        (Key::Key3 | Key::NumPad3, false) => Some('3'),
        (Key::Key4 | Key::NumPad4, false) => Some('4'),
        (Key::Key5 | Key::NumPad5, false) => Some('5'),
        (Key::Key6 | Key::NumPad6, false) => Some('6'),
        (Key::Key7 | Key::NumPad7, false) => Some('7'),
        (Key::Key8 | Key::NumPad8, false) => Some('8'),
        (Key::Key9 | Key::NumPad9, false) => Some('9'),
        (Key::Key1, true) => Some('!'),
        (Key::Key2, true) => Some('@'),
        (Key::Key3, true) => Some('#'),
        (Key::Key4, true) => Some('$'),
        (Key::Key5, true) => Some('%'),
        (Key::Key6, true) => Some('^'),
        (Key::Key7, true) => Some('&'),
        (Key::Key8, true) => Some('*'),
        (Key::Key9, true) => Some('('),
        (Key::Key0, true) => Some(')'),
        _ => None,
    }
}

fn symbol_for_key(key: Key, shifted: bool) -> Option<char> {
    match (key, shifted) {
        (Key::Space, _) => Some(' '),
        (Key::Minus | Key::NumPadMinus, false) => Some('-'),
        (Key::Minus, true) => Some('_'),
        (Key::Equal | Key::NumPadPlus, false) => Some('='),
        (Key::Equal, true) => Some('+'),
        (Key::Comma, false) => Some(','),
        (Key::Comma, true) => Some('<'),
        (Key::Period | Key::NumPadDot, false) => Some('.'),
        (Key::Period, true) => Some('>'),
        (Key::Slash | Key::NumPadSlash, false) => Some('/'),
        (Key::Slash, true) => Some('?'),
        (Key::Semicolon, false) => Some(';'),
        (Key::Semicolon, true) => Some(':'),
        (Key::Apostrophe, false) => Some('\''),
        (Key::Apostrophe, true) => Some('"'),
        _ => None,
    }
}
