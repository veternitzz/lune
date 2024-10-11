use std::str::FromStr;

use mlua::prelude::*;
use rawkey::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/**
    A key kind supported by rawkey.
*/
pub enum KeyKind {
    Escape,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    LeftShift,
    LeftControl,
    Tab,
    Backspace,
    Space,
    PageUp,
    PageDown,
    Alt,
}

impl KeyKind {
    pub const ALL: [Self; 13] = [
        Self::Escape,
        Self::UpArrow,
        Self::DownArrow,
        Self::LeftArrow,
        Self::RightArrow,
        Self::LeftShift,
        Self::LeftControl,
        Self::Tab,
        Self::Backspace,
        Self::Space,
        Self::PageUp,
        Self::PageDown,
        Self::Alt,
    ];

    /**
        Returns the human-friendly name of this key kind.
    */
    pub fn name(self) -> &'static str {
        match self {
            Self::Escape => "esc",
            Self::UpArrow => "arrowUp",
            Self::DownArrow => "arrowDown",
            Self::LeftArrow => "arrowLeft",
            Self::RightArrow => "arrowRight",
            Self::LeftShift => "shiftLeft",
            Self::LeftControl => "ctrlLeft",
            Self::Tab => "tab",
            Self::Backspace => "backspace",
            Self::Space => "space",
            Self::PageUp => "pageUp",
            Self::PageDown => "pageDown",
            Self::Alt => "alt",
        }
    }

    /**
        Returns the keycode object for the key kind.
    */
    pub fn key_code_object(self) -> KeyCode {
        match self {
            Self::Escape => KeyCode::Escape,
            Self::UpArrow => KeyCode::UpArrow,
            Self::DownArrow => KeyCode::DownArrow,
            Self::LeftArrow => KeyCode::LeftArrow,
            Self::RightArrow => KeyCode::RightArrow,
            Self::LeftShift => KeyCode::LeftShift,
            Self::LeftControl => KeyCode::LeftControl,
            Self::Tab => KeyCode::Tab,
            Self::Backspace => KeyCode::BackSpace,
            Self::Space => KeyCode::Space,
            Self::PageUp => KeyCode::PageUp,
            Self::PageDown => KeyCode::PageDown,
            Self::Alt => KeyCode::Alt,
        }
    }
}

impl FromStr for KeyKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "esc" => Self::Escape,
            "arrowUp" => Self::UpArrow,
            "arrowDown" => Self::DownArrow,
            "arrowLeft" => Self::LeftArrow,
            "arrowRight" => Self::RightArrow,
            "shitLeft" => Self::LeftShift,
            "ctrlLeft" => Self::LeftControl,
            "tab" => Self::Tab,
            "backspace" => Self::Backspace,
            "space" => Self::Space,
            "pageUp" => Self::PageUp,
            "pageDown" => Self::PageDown,
            "alt" => Self::Alt,
            _ => return Err(()),
        })
    }
}

impl FromLua<'_> for KeyKind {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        if let LuaValue::String(s) = value {
            let s = s.to_str()?;
            match s.parse() {
                Ok(key) => Ok(key),
                Err(()) => Err(LuaError::FromLuaConversionError {
                    from: "string",
                    to: "KeyKind",
                    message: Some(format!(
                        "Invalid key kind '{s}'\nValid kinds are: {}",
                        Self::ALL
                            .iter()
                            .map(|kind| kind.name())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                }),
            }
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "KeyKind",
                message: None,
            })
        }
    }
}
