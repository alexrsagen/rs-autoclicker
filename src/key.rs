use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Error {
	InvalidKey,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidKey => f.write_str("invalid key"),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Key {
	Keyboard { key: rdev::Key },
	Mouse { button: rdev::Button },
}

impl Default for Key {
	fn default() -> Self {
		Self::Keyboard { key: rdev::Key::Insert }
	}
}

impl FromStr for Key {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().replace('_', "").as_str() {
			// "alt" => Ok(Self::Keyboard { key: rdev::Key::Alt }),
			// "altgr" => Ok(Self::Keyboard { key: rdev::Key::AltGr }),
			"backspace" => Ok(Self::Keyboard { key: rdev::Key::Backspace }),
			"capslock" => Ok(Self::Keyboard { key: rdev::Key::CapsLock }),
			// "controlleft" => Ok(Self::Keyboard { key: rdev::Key::ControlLeft }),
			// "controlright" => Ok(Self::Keyboard { key: rdev::Key::ControlRight }),
			"delete" => Ok(Self::Keyboard { key: rdev::Key::Delete }),
			"downarrow" => Ok(Self::Keyboard { key: rdev::Key::DownArrow }),
			"end" => Ok(Self::Keyboard { key: rdev::Key::End }),
			"escape" => Ok(Self::Keyboard { key: rdev::Key::Escape }),
			"f1" => Ok(Self::Keyboard { key: rdev::Key::F1 }),
			"f10" => Ok(Self::Keyboard { key: rdev::Key::F10 }),
			"f11" => Ok(Self::Keyboard { key: rdev::Key::F11 }),
			"f12" => Ok(Self::Keyboard { key: rdev::Key::F12 }),
			"f2" => Ok(Self::Keyboard { key: rdev::Key::F2 }),
			"f3" => Ok(Self::Keyboard { key: rdev::Key::F3 }),
			"f4" => Ok(Self::Keyboard { key: rdev::Key::F4 }),
			"f5" => Ok(Self::Keyboard { key: rdev::Key::F5 }),
			"f6" => Ok(Self::Keyboard { key: rdev::Key::F6 }),
			"f7" => Ok(Self::Keyboard { key: rdev::Key::F7 }),
			"f8" => Ok(Self::Keyboard { key: rdev::Key::F8 }),
			"f9" => Ok(Self::Keyboard { key: rdev::Key::F9 }),
			"home" => Ok(Self::Keyboard { key: rdev::Key::Home }),
			"leftarrow" => Ok(Self::Keyboard { key: rdev::Key::LeftArrow }),
			"metaleft" => Ok(Self::Keyboard { key: rdev::Key::MetaLeft }),
			"metaright" => Ok(Self::Keyboard { key: rdev::Key::MetaRight }),
			"pagedown" => Ok(Self::Keyboard { key: rdev::Key::PageDown }),
			"pageup" => Ok(Self::Keyboard { key: rdev::Key::PageUp }),
			"return" => Ok(Self::Keyboard { key: rdev::Key::Return }),
			"rightarrow" => Ok(Self::Keyboard { key: rdev::Key::RightArrow }),
			"shiftleft" => Ok(Self::Keyboard { key: rdev::Key::ShiftLeft }),
			"shiftright" => Ok(Self::Keyboard { key: rdev::Key::ShiftRight }),
			"space" => Ok(Self::Keyboard { key: rdev::Key::Space }),
			"tab" => Ok(Self::Keyboard { key: rdev::Key::Tab }),
			"uparrow" => Ok(Self::Keyboard { key: rdev::Key::UpArrow }),
			"printscreen" => Ok(Self::Keyboard { key: rdev::Key::PrintScreen }),
			"scrolllock" => Ok(Self::Keyboard { key: rdev::Key::ScrollLock }),
			"pause" => Ok(Self::Keyboard { key: rdev::Key::Pause }),
			"numlock" => Ok(Self::Keyboard { key: rdev::Key::NumLock }),
			"backquote" => Ok(Self::Keyboard { key: rdev::Key::BackQuote }),
			"num1" => Ok(Self::Keyboard { key: rdev::Key::Num1 }),
			"num2" => Ok(Self::Keyboard { key: rdev::Key::Num2 }),
			"num3" => Ok(Self::Keyboard { key: rdev::Key::Num3 }),
			"num4" => Ok(Self::Keyboard { key: rdev::Key::Num4 }),
			"num5" => Ok(Self::Keyboard { key: rdev::Key::Num5 }),
			"num6" => Ok(Self::Keyboard { key: rdev::Key::Num6 }),
			"num7" => Ok(Self::Keyboard { key: rdev::Key::Num7 }),
			"num8" => Ok(Self::Keyboard { key: rdev::Key::Num8 }),
			"num9" => Ok(Self::Keyboard { key: rdev::Key::Num9 }),
			"num0" => Ok(Self::Keyboard { key: rdev::Key::Num0 }),
			"minus" => Ok(Self::Keyboard { key: rdev::Key::Minus }),
			"equal" => Ok(Self::Keyboard { key: rdev::Key::Equal }),
			"keyq" => Ok(Self::Keyboard { key: rdev::Key::KeyQ }),
			"keyw" => Ok(Self::Keyboard { key: rdev::Key::KeyW }),
			"keye" => Ok(Self::Keyboard { key: rdev::Key::KeyE }),
			"keyr" => Ok(Self::Keyboard { key: rdev::Key::KeyR }),
			"keyt" => Ok(Self::Keyboard { key: rdev::Key::KeyT }),
			"keyy" => Ok(Self::Keyboard { key: rdev::Key::KeyY }),
			"keyu" => Ok(Self::Keyboard { key: rdev::Key::KeyU }),
			"keyi" => Ok(Self::Keyboard { key: rdev::Key::KeyI }),
			"keyo" => Ok(Self::Keyboard { key: rdev::Key::KeyO }),
			"keyp" => Ok(Self::Keyboard { key: rdev::Key::KeyP }),
			"leftbracket" => Ok(Self::Keyboard { key: rdev::Key::LeftBracket }),
			"rightbracket" => Ok(Self::Keyboard { key: rdev::Key::RightBracket }),
			"keya" => Ok(Self::Keyboard { key: rdev::Key::KeyA }),
			"keys" => Ok(Self::Keyboard { key: rdev::Key::KeyS }),
			"keyd" => Ok(Self::Keyboard { key: rdev::Key::KeyD }),
			"keyf" => Ok(Self::Keyboard { key: rdev::Key::KeyF }),
			"keyg" => Ok(Self::Keyboard { key: rdev::Key::KeyG }),
			"keyh" => Ok(Self::Keyboard { key: rdev::Key::KeyH }),
			"keyj" => Ok(Self::Keyboard { key: rdev::Key::KeyJ }),
			"keyk" => Ok(Self::Keyboard { key: rdev::Key::KeyK }),
			"keyl" => Ok(Self::Keyboard { key: rdev::Key::KeyL }),
			"semicolon" => Ok(Self::Keyboard { key: rdev::Key::SemiColon }),
			"quote" => Ok(Self::Keyboard { key: rdev::Key::Quote }),
			"backslash" => Ok(Self::Keyboard { key: rdev::Key::BackSlash }),
			"intlbackslash" => Ok(Self::Keyboard { key: rdev::Key::IntlBackslash }),
			"keyz" => Ok(Self::Keyboard { key: rdev::Key::KeyZ }),
			"keyx" => Ok(Self::Keyboard { key: rdev::Key::KeyX }),
			"keyc" => Ok(Self::Keyboard { key: rdev::Key::KeyC }),
			"keyv" => Ok(Self::Keyboard { key: rdev::Key::KeyV }),
			"keyb" => Ok(Self::Keyboard { key: rdev::Key::KeyB }),
			"keyn" => Ok(Self::Keyboard { key: rdev::Key::KeyN }),
			"keym" => Ok(Self::Keyboard { key: rdev::Key::KeyM }),
			"comma" => Ok(Self::Keyboard { key: rdev::Key::Comma }),
			"dot" => Ok(Self::Keyboard { key: rdev::Key::Dot }),
			"slash" => Ok(Self::Keyboard { key: rdev::Key::Slash }),
			"insert" => Ok(Self::Keyboard { key: rdev::Key::Insert }),
			"kpreturn" => Ok(Self::Keyboard { key: rdev::Key::KpReturn }),
			"kpminus" => Ok(Self::Keyboard { key: rdev::Key::KpMinus }),
			"kpplus" => Ok(Self::Keyboard { key: rdev::Key::KpPlus }),
			"kpmultiply" => Ok(Self::Keyboard { key: rdev::Key::KpMultiply }),
			"kpdivide" => Ok(Self::Keyboard { key: rdev::Key::KpDivide }),
			"kp0" => Ok(Self::Keyboard { key: rdev::Key::Kp0 }),
			"kp1" => Ok(Self::Keyboard { key: rdev::Key::Kp1 }),
			"kp2" => Ok(Self::Keyboard { key: rdev::Key::Kp2 }),
			"kp3" => Ok(Self::Keyboard { key: rdev::Key::Kp3 }),
			"kp4" => Ok(Self::Keyboard { key: rdev::Key::Kp4 }),
			"kp5" => Ok(Self::Keyboard { key: rdev::Key::Kp5 }),
			"kp6" => Ok(Self::Keyboard { key: rdev::Key::Kp6 }),
			"kp7" => Ok(Self::Keyboard { key: rdev::Key::Kp7 }),
			"kp8" => Ok(Self::Keyboard { key: rdev::Key::Kp8 }),
			"kp9" => Ok(Self::Keyboard { key: rdev::Key::Kp9 }),
			"kpdelete" => Ok(Self::Keyboard { key: rdev::Key::KpDelete }),
			// "function" => Ok(Self::Keyboard { key: rdev::Key::Function }),
			"mouseleft" => Ok(Self::Mouse { button: rdev::Button::Left }),
			"mousemiddle" => Ok(Self::Mouse { button: rdev::Button::Middle }),
			"mouseright" => Ok(Self::Mouse { button: rdev::Button::Right }),
			"mouse4" => Ok(Self::Mouse { button: rdev::Button::Unknown(1) }),
			"mouse5" => Ok(Self::Mouse { button: rdev::Button::Unknown(2) }),
			_ => Err(Error::InvalidKey),
		}
	}
}

impl fmt::Display for Key {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			// Self::Keyboard { key: rdev::Key::Alt } => f.write_str("alt"),
			// Self::Keyboard { key: rdev::Key::AltGr } => f.write_str("altgr"),
			Self::Keyboard { key: rdev::Key::Backspace } => f.write_str("backspace"),
			Self::Keyboard { key: rdev::Key::CapsLock } => f.write_str("capslock"),
			// Self::Keyboard { key: rdev::Key::ControlLeft } => f.write_str("controlleft"),
			// Self::Keyboard { key: rdev::Key::ControlRight } => f.write_str("controlright"),
			Self::Keyboard { key: rdev::Key::Delete } => f.write_str("delete"),
			Self::Keyboard { key: rdev::Key::DownArrow } => f.write_str("downarrow"),
			Self::Keyboard { key: rdev::Key::End } => f.write_str("end"),
			Self::Keyboard { key: rdev::Key::Escape } => f.write_str("escape"),
			Self::Keyboard { key: rdev::Key::F1 } => f.write_str("f1"),
			Self::Keyboard { key: rdev::Key::F10 } => f.write_str("f10"),
			Self::Keyboard { key: rdev::Key::F11 } => f.write_str("f11"),
			Self::Keyboard { key: rdev::Key::F12 } => f.write_str("f12"),
			Self::Keyboard { key: rdev::Key::F2 } => f.write_str("f2"),
			Self::Keyboard { key: rdev::Key::F3 } => f.write_str("f3"),
			Self::Keyboard { key: rdev::Key::F4 } => f.write_str("f4"),
			Self::Keyboard { key: rdev::Key::F5 } => f.write_str("f5"),
			Self::Keyboard { key: rdev::Key::F6 } => f.write_str("f6"),
			Self::Keyboard { key: rdev::Key::F7 } => f.write_str("f7"),
			Self::Keyboard { key: rdev::Key::F8 } => f.write_str("f8"),
			Self::Keyboard { key: rdev::Key::F9 } => f.write_str("f9"),
			Self::Keyboard { key: rdev::Key::Home } => f.write_str("home"),
			Self::Keyboard { key: rdev::Key::LeftArrow } => f.write_str("leftarrow"),
			Self::Keyboard { key: rdev::Key::MetaLeft } => f.write_str("metaleft"),
			Self::Keyboard { key: rdev::Key::MetaRight } => f.write_str("metaright"),
			Self::Keyboard { key: rdev::Key::PageDown } => f.write_str("pagedown"),
			Self::Keyboard { key: rdev::Key::PageUp } => f.write_str("pageup"),
			Self::Keyboard { key: rdev::Key::Return } => f.write_str("return"),
			Self::Keyboard { key: rdev::Key::RightArrow } => f.write_str("rightarrow"),
			Self::Keyboard { key: rdev::Key::ShiftLeft } => f.write_str("shiftleft"),
			Self::Keyboard { key: rdev::Key::ShiftRight } => f.write_str("shiftright"),
			Self::Keyboard { key: rdev::Key::Space } => f.write_str("space"),
			Self::Keyboard { key: rdev::Key::Tab } => f.write_str("tab"),
			Self::Keyboard { key: rdev::Key::UpArrow } => f.write_str("uparrow"),
			Self::Keyboard { key: rdev::Key::PrintScreen } => f.write_str("printscreen"),
			Self::Keyboard { key: rdev::Key::ScrollLock } => f.write_str("scrolllock"),
			Self::Keyboard { key: rdev::Key::Pause } => f.write_str("pause"),
			Self::Keyboard { key: rdev::Key::NumLock } => f.write_str("numlock"),
			Self::Keyboard { key: rdev::Key::BackQuote } => f.write_str("backquote"),
			Self::Keyboard { key: rdev::Key::Num1 } => f.write_str("num1"),
			Self::Keyboard { key: rdev::Key::Num2 } => f.write_str("num2"),
			Self::Keyboard { key: rdev::Key::Num3 } => f.write_str("num3"),
			Self::Keyboard { key: rdev::Key::Num4 } => f.write_str("num4"),
			Self::Keyboard { key: rdev::Key::Num5 } => f.write_str("num5"),
			Self::Keyboard { key: rdev::Key::Num6 } => f.write_str("num6"),
			Self::Keyboard { key: rdev::Key::Num7 } => f.write_str("num7"),
			Self::Keyboard { key: rdev::Key::Num8 } => f.write_str("num8"),
			Self::Keyboard { key: rdev::Key::Num9 } => f.write_str("num9"),
			Self::Keyboard { key: rdev::Key::Num0 } => f.write_str("num0"),
			Self::Keyboard { key: rdev::Key::Minus } => f.write_str("minus"),
			Self::Keyboard { key: rdev::Key::Equal } => f.write_str("equal"),
			Self::Keyboard { key: rdev::Key::KeyQ } => f.write_str("keyq"),
			Self::Keyboard { key: rdev::Key::KeyW } => f.write_str("keyw"),
			Self::Keyboard { key: rdev::Key::KeyE } => f.write_str("keye"),
			Self::Keyboard { key: rdev::Key::KeyR } => f.write_str("keyr"),
			Self::Keyboard { key: rdev::Key::KeyT } => f.write_str("keyt"),
			Self::Keyboard { key: rdev::Key::KeyY } => f.write_str("keyy"),
			Self::Keyboard { key: rdev::Key::KeyU } => f.write_str("keyu"),
			Self::Keyboard { key: rdev::Key::KeyI } => f.write_str("keyi"),
			Self::Keyboard { key: rdev::Key::KeyO } => f.write_str("keyo"),
			Self::Keyboard { key: rdev::Key::KeyP } => f.write_str("keyp"),
			Self::Keyboard { key: rdev::Key::LeftBracket } => f.write_str("leftbracket"),
			Self::Keyboard { key: rdev::Key::RightBracket } => f.write_str("rightbracket"),
			Self::Keyboard { key: rdev::Key::KeyA } => f.write_str("keya"),
			Self::Keyboard { key: rdev::Key::KeyS } => f.write_str("keys"),
			Self::Keyboard { key: rdev::Key::KeyD } => f.write_str("keyd"),
			Self::Keyboard { key: rdev::Key::KeyF } => f.write_str("keyf"),
			Self::Keyboard { key: rdev::Key::KeyG } => f.write_str("keyg"),
			Self::Keyboard { key: rdev::Key::KeyH } => f.write_str("keyh"),
			Self::Keyboard { key: rdev::Key::KeyJ } => f.write_str("keyj"),
			Self::Keyboard { key: rdev::Key::KeyK } => f.write_str("keyk"),
			Self::Keyboard { key: rdev::Key::KeyL } => f.write_str("keyl"),
			Self::Keyboard { key: rdev::Key::SemiColon } => f.write_str("semicolon"),
			Self::Keyboard { key: rdev::Key::Quote } => f.write_str("quote"),
			Self::Keyboard { key: rdev::Key::BackSlash } => f.write_str("backslash"),
			Self::Keyboard { key: rdev::Key::IntlBackslash } => f.write_str("intlbackslash"),
			Self::Keyboard { key: rdev::Key::KeyZ } => f.write_str("keyz"),
			Self::Keyboard { key: rdev::Key::KeyX } => f.write_str("keyx"),
			Self::Keyboard { key: rdev::Key::KeyC } => f.write_str("keyc"),
			Self::Keyboard { key: rdev::Key::KeyV } => f.write_str("keyv"),
			Self::Keyboard { key: rdev::Key::KeyB } => f.write_str("keyb"),
			Self::Keyboard { key: rdev::Key::KeyN } => f.write_str("keyn"),
			Self::Keyboard { key: rdev::Key::KeyM } => f.write_str("keym"),
			Self::Keyboard { key: rdev::Key::Comma } => f.write_str("comma"),
			Self::Keyboard { key: rdev::Key::Dot } => f.write_str("dot"),
			Self::Keyboard { key: rdev::Key::Slash } => f.write_str("slash"),
			Self::Keyboard { key: rdev::Key::Insert } => f.write_str("insert"),
			Self::Keyboard { key: rdev::Key::KpReturn } => f.write_str("kpreturn"),
			Self::Keyboard { key: rdev::Key::KpMinus } => f.write_str("kpminus"),
			Self::Keyboard { key: rdev::Key::KpPlus } => f.write_str("kpplus"),
			Self::Keyboard { key: rdev::Key::KpMultiply } => f.write_str("kpmultiply"),
			Self::Keyboard { key: rdev::Key::KpDivide } => f.write_str("kpdivide"),
			Self::Keyboard { key: rdev::Key::Kp0 } => f.write_str("kp0"),
			Self::Keyboard { key: rdev::Key::Kp1 } => f.write_str("kp1"),
			Self::Keyboard { key: rdev::Key::Kp2 } => f.write_str("kp2"),
			Self::Keyboard { key: rdev::Key::Kp3 } => f.write_str("kp3"),
			Self::Keyboard { key: rdev::Key::Kp4 } => f.write_str("kp4"),
			Self::Keyboard { key: rdev::Key::Kp5 } => f.write_str("kp5"),
			Self::Keyboard { key: rdev::Key::Kp6 } => f.write_str("kp6"),
			Self::Keyboard { key: rdev::Key::Kp7 } => f.write_str("kp7"),
			Self::Keyboard { key: rdev::Key::Kp8 } => f.write_str("kp8"),
			Self::Keyboard { key: rdev::Key::Kp9 } => f.write_str("kp9"),
			Self::Keyboard { key: rdev::Key::KpDelete } => f.write_str("kpdelete"),
			// Self::Keyboard { key: rdev::Key::Function } => f.write_str("function"),
			Self::Mouse { button: rdev::Button::Left } => f.write_str("mouseleft"),
			Self::Mouse { button: rdev::Button::Middle } => f.write_str("mousemiddle"),
			Self::Mouse { button: rdev::Button::Right } => f.write_str("mouseright"),
			Self::Mouse { button: rdev::Button::Unknown(1) } => f.write_str("mouse4"),
			Self::Mouse { button: rdev::Button::Unknown(2) } => f.write_str("mouse5"),
			_ => f.write_str(""),
		}
	}
}

impl TryFrom<String> for Key
where
    Self: FromStr,
{
    type Error = <Self as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Key::from_str(value.as_str())
    }
}

impl From<Key> for String {
    fn from(value: Key) -> Self {
        value.to_string()
    }
}