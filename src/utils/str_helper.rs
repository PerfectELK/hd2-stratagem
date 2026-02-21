use rdev::{Key as RdevKey};


pub fn str_to_key(s: &str) -> Option<RdevKey> {
    match s {
        // Буквы
        "A" => Some(RdevKey::KeyA),
        "B" => Some(RdevKey::KeyB),
        "C" => Some(RdevKey::KeyC),
        "D" => Some(RdevKey::KeyD),
        "E" => Some(RdevKey::KeyE),
        "F" => Some(RdevKey::KeyF),
        "G" => Some(RdevKey::KeyG),
        "H" => Some(RdevKey::KeyH),
        "I" => Some(RdevKey::KeyI),
        "J" => Some(RdevKey::KeyJ),
        "K" => Some(RdevKey::KeyK),
        "L" => Some(RdevKey::KeyL),
        "M" => Some(RdevKey::KeyM),
        "N" => Some(RdevKey::KeyN),
        "O" => Some(RdevKey::KeyO),
        "P" => Some(RdevKey::KeyP),
        "Q" => Some(RdevKey::KeyQ),
        "R" => Some(RdevKey::KeyR),
        "S" => Some(RdevKey::KeyS),
        "T" => Some(RdevKey::KeyT),
        "U" => Some(RdevKey::KeyU),
        "V" => Some(RdevKey::KeyV),
        "W" => Some(RdevKey::KeyW),
        "X" => Some(RdevKey::KeyX),
        "Y" => Some(RdevKey::KeyY),
        "Z" => Some(RdevKey::KeyZ),

        // Цифры (верхний ряд)
        "1" => Some(RdevKey::Num1),
        "2" => Some(RdevKey::Num2),
        "3" => Some(RdevKey::Num3),
        "4" => Some(RdevKey::Num4),
        "5" => Some(RdevKey::Num5),
        "6" => Some(RdevKey::Num6),
        "7" => Some(RdevKey::Num7),
        "8" => Some(RdevKey::Num8),
        "9" => Some(RdevKey::Num9),
        "0" => Some(RdevKey::Num0),

        // Функциональные клавиши
        "F1" => Some(RdevKey::F1),
        "F2" => Some(RdevKey::F2),
        "F3" => Some(RdevKey::F3),
        "F4" => Some(RdevKey::F4),
        "F5" => Some(RdevKey::F5),
        "F6" => Some(RdevKey::F6),
        "F7" => Some(RdevKey::F7),
        "F8" => Some(RdevKey::F8),
        "F9" => Some(RdevKey::F9),
        "F10" => Some(RdevKey::F10),
        "F11" => Some(RdevKey::F11),
        "F12" => Some(RdevKey::F12),

        // Клавиши цифровой клавиатуры (Numpad)
        "Num0" => Some(RdevKey::Num0),
        "Num1" => Some(RdevKey::Num1),
        "Num2" => Some(RdevKey::Num2),
        "Num3" => Some(RdevKey::Num3),
        "Num4" => Some(RdevKey::Num4),
        "Num5" => Some(RdevKey::Num5),
        "Num6" => Some(RdevKey::Num6),
        "Num7" => Some(RdevKey::Num7),
        "Num8" => Some(RdevKey::Num8),
        "Num9" => Some(RdevKey::Num9),

        // Клавиши-модификаторы
        "Alt" => Some(RdevKey::Alt),
        "AltGr" => Some(RdevKey::AltGr),

        // Левая и правая версии модификаторов
        "Ctrl" => Some(RdevKey::ControlLeft),
        "Shift" => Some(RdevKey::ShiftLeft),
        "LeftAlt" => Some(RdevKey::Alt),
        "RightAlt" => Some(RdevKey::AltGr),


        // Клавиши управления
        "Return" => Some(RdevKey::Return),
        "Escape" => Some(RdevKey::Escape),
        "Backspace" => Some(RdevKey::Backspace),
        "Delete" => Some(RdevKey::Delete),
        "Insert" => Some(RdevKey::Insert),
        "Tab" => Some(RdevKey::Tab),
        "Space" => Some(RdevKey::Space),
        "CapsLock" => Some(RdevKey::CapsLock),
        "NumLock" => Some(RdevKey::NumLock),
        "ScrollLock" => Some(RdevKey::ScrollLock),
        "Pause" => Some(RdevKey::Pause),
        "PrintScreen" => Some(RdevKey::PrintScreen),

        // Клавиши со стрелками
        "Up" => Some(RdevKey::UpArrow),
        "Down" => Some(RdevKey::DownArrow),
        "Left" => Some(RdevKey::LeftArrow),
        "Right" => Some(RdevKey::RightArrow),

        // Навигационные клавиши
        "Home" => Some(RdevKey::Home),
        "End" => Some(RdevKey::End),
        "PageUp" => Some(RdevKey::PageUp),
        "PageDown" => Some(RdevKey::PageDown),

        // Специальные символы и знаки пунктуации
        "Backquote" => Some(RdevKey::BackQuote),
        "Backslash" => Some(RdevKey::BackSlash),
        "Comma" => Some(RdevKey::Comma),
        "Dot" => Some(RdevKey::Dot),
        "Equal" => Some(RdevKey::Equal),
        "Minus" => Some(RdevKey::Minus),
        "Quote" => Some(RdevKey::Quote),
        "Semicolon" => Some(RdevKey::SemiColon),
        "Slash" => Some(RdevKey::Slash),

        // Дополнительные клавиши
        "Function" => Some(RdevKey::Function),
        _ => None,
    }
}