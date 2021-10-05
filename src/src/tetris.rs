use crossterm::{
    event::{KeyCode},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Movement {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct KeyboardControl {
    pub start: KeyCode,
    pub pause: KeyCode,
    pub down: KeyCode,
    pub right: KeyCode,
    pub left: KeyCode,
    pub change: KeyCode,
    pub exit: KeyCode,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AppSettings {
    pub gaming_region: Size,
    pub info_region: Size,
    pub total_region: Size,
    pub welcome_region: Size,
    pub gaming_blocks_size: Size,
    pub keyboard_control: KeyboardControl,
}
