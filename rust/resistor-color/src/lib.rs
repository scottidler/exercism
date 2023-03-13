use int_enum::IntEnum;
use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
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

impl From<u32> for ResistorColor {
    fn from(value: u32) -> Self {
        match value {
            0 => ResistorColor::Black,
            1 => ResistorColor::Brown,
            2 => ResistorColor::Red,
            3 => ResistorColor::Orange,
            4 => ResistorColor::Yellow,
            5 => ResistorColor::Green,
            6 => ResistorColor::Blue,
            7 => ResistorColor::Violet,
            8 => ResistorColor::Grey,
            9 => ResistorColor::White,
            _ => panic!("value out of range"),
        }
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0..=9 => format!("{:?}", ResistorColor::from(value)),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
