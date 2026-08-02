use super::super::window_interaction::{TextAreaKey, TextInputKey};
use katana_ui_core::widget::molecules::{CloseableTabKey, CloseableTabKeyboardShortcut};
use minifb::Key;

const DIGIT_ZERO: u8 = 0;
const DIGIT_ONE: u8 = 1;
const DIGIT_TWO: u8 = 2;
const DIGIT_THREE: u8 = 3;
const DIGIT_FOUR: u8 = 4;
const DIGIT_FIVE: u8 = 5;
const DIGIT_SIX: u8 = 6;
const DIGIT_SEVEN: u8 = 7;
const DIGIT_EIGHT: u8 = 8;
const DIGIT_NINE: u8 = 9;

pub(super) fn tabs_keyboard_shortcut(
    key: Key,
    command_or_control: bool,
    shift: bool,
) -> Option<CloseableTabKeyboardShortcut> {
    let tab_key = match key {
        Key::Tab => CloseableTabKey::Tab,
        Key::W => CloseableTabKey::W,
        Key::Escape => CloseableTabKey::Escape,
        Key::Key0 | Key::NumPad0 => CloseableTabKey::Digit(DIGIT_ZERO),
        Key::Key1 | Key::NumPad1 => CloseableTabKey::Digit(DIGIT_ONE),
        Key::Key2 | Key::NumPad2 => CloseableTabKey::Digit(DIGIT_TWO),
        Key::Key3 | Key::NumPad3 => CloseableTabKey::Digit(DIGIT_THREE),
        Key::Key4 | Key::NumPad4 => CloseableTabKey::Digit(DIGIT_FOUR),
        Key::Key5 | Key::NumPad5 => CloseableTabKey::Digit(DIGIT_FIVE),
        Key::Key6 | Key::NumPad6 => CloseableTabKey::Digit(DIGIT_SIX),
        Key::Key7 | Key::NumPad7 => CloseableTabKey::Digit(DIGIT_SEVEN),
        Key::Key8 | Key::NumPad8 => CloseableTabKey::Digit(DIGIT_EIGHT),
        Key::Key9 | Key::NumPad9 => CloseableTabKey::Digit(DIGIT_NINE),
        _ => return None,
    };
    Some(CloseableTabKeyboardShortcut::new(
        tab_key,
        command_or_control,
        shift,
    ))
}

pub(super) fn text_area_key(key: Key, shifted: bool) -> Option<TextAreaKey> {
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

pub(super) fn text_input_key(key: Key, shifted: bool) -> Option<TextInputKey> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_shortcuts_cover_every_supported_key_family() {
        let keys = [
            Key::Tab,
            Key::W,
            Key::Escape,
            Key::Key0,
            Key::Key1,
            Key::Key2,
            Key::Key3,
            Key::Key4,
            Key::Key5,
            Key::Key6,
            Key::Key7,
            Key::Key8,
            Key::Key9,
            Key::NumPad0,
            Key::NumPad1,
            Key::NumPad2,
            Key::NumPad3,
            Key::NumPad4,
            Key::NumPad5,
            Key::NumPad6,
            Key::NumPad7,
            Key::NumPad8,
            Key::NumPad9,
        ];

        for key in keys {
            assert!(tabs_keyboard_shortcut(key, false, false).is_some());
            assert!(tabs_keyboard_shortcut(key, true, true).is_some());
        }
        assert!(tabs_keyboard_shortcut(Key::F1, false, false).is_none());
    }

    #[test]
    fn text_entry_keys_cover_letters_digits_symbols_and_commands() {
        let letters = [
            (Key::A, 'a'),
            (Key::B, 'b'),
            (Key::C, 'c'),
            (Key::D, 'd'),
            (Key::E, 'e'),
            (Key::F, 'f'),
            (Key::G, 'g'),
            (Key::H, 'h'),
            (Key::I, 'i'),
            (Key::J, 'j'),
            (Key::K, 'k'),
            (Key::L, 'l'),
            (Key::M, 'm'),
            (Key::N, 'n'),
            (Key::O, 'o'),
            (Key::P, 'p'),
            (Key::Q, 'q'),
            (Key::R, 'r'),
            (Key::S, 's'),
            (Key::T, 't'),
            (Key::U, 'u'),
            (Key::V, 'v'),
            (Key::W, 'w'),
            (Key::X, 'x'),
            (Key::Y, 'y'),
            (Key::Z, 'z'),
        ];
        for (key, character) in letters {
            assert_eq!(character_for_key(key, false), Some(character));
            assert_eq!(
                character_for_key(key, true),
                Some(character.to_ascii_uppercase())
            );
        }

        let digits = [
            (Key::Key0, Key::NumPad0, '0', ')'),
            (Key::Key1, Key::NumPad1, '1', '!'),
            (Key::Key2, Key::NumPad2, '2', '@'),
            (Key::Key3, Key::NumPad3, '3', '#'),
            (Key::Key4, Key::NumPad4, '4', '$'),
            (Key::Key5, Key::NumPad5, '5', '%'),
            (Key::Key6, Key::NumPad6, '6', '^'),
            (Key::Key7, Key::NumPad7, '7', '&'),
            (Key::Key8, Key::NumPad8, '8', '*'),
            (Key::Key9, Key::NumPad9, '9', '('),
        ];
        for (key, numpad, plain, shifted) in digits {
            assert_eq!(character_for_key(key, false), Some(plain));
            assert_eq!(character_for_key(numpad, false), Some(plain));
            assert_eq!(character_for_key(key, true), Some(shifted));
            assert_eq!(character_for_key(numpad, true), None);
        }

        let symbols = [
            (Key::Minus, '-', '_'),
            (Key::Equal, '=', '+'),
            (Key::Comma, ',', '<'),
            (Key::Period, '.', '>'),
            (Key::Slash, '/', '?'),
            (Key::Semicolon, ';', ':'),
            (Key::Apostrophe, '\'', '"'),
        ];
        for (key, plain, shifted) in symbols {
            assert_eq!(character_for_key(key, false), Some(plain));
            assert_eq!(character_for_key(key, true), Some(shifted));
        }
        assert_eq!(character_for_key(Key::Space, false), Some(' '));
        assert_eq!(character_for_key(Key::Space, true), Some(' '));
        assert_eq!(character_for_key(Key::NumPadMinus, false), Some('-'));
        assert_eq!(character_for_key(Key::NumPadPlus, false), Some('='));
        assert_eq!(character_for_key(Key::NumPadDot, false), Some('.'));
        assert_eq!(character_for_key(Key::NumPadSlash, false), Some('/'));
        assert_eq!(character_for_key(Key::F1, false), None);

        assert!(matches!(
            text_area_key(Key::Backspace, false),
            Some(TextAreaKey::Backspace)
        ));
        assert!(matches!(
            text_area_key(Key::Enter, false),
            Some(TextAreaKey::Submit)
        ));
        assert!(matches!(
            text_area_key(Key::NumPadEnter, true),
            Some(TextAreaKey::Newline)
        ));
        assert!(matches!(
            text_area_key(Key::A, false),
            Some(TextAreaKey::Character('a'))
        ));
        assert_eq!(
            text_input_key(Key::Backspace, false),
            Some(TextInputKey::Backspace)
        );
        assert_eq!(text_input_key(Key::Enter, true), Some(TextInputKey::Submit));
        assert_eq!(
            text_input_key(Key::A, false),
            Some(TextInputKey::Character('a'))
        );
        assert_eq!(text_input_key(Key::F1, false), None);
    }
}
