use int_enum::IntEnum;
use std::fmt::{self, Display, Formatter};
use enum_iterator::IntoEnumIterator;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    let result = match ResistorColor::from_int(value) {
        // Ok(color) => color.to_string(),
        Ok(color) => format!("{:?}", color),
        Err(_) => "value out of range".to_string(),
    };
    result
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
