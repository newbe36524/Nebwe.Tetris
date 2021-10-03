use crate::tetris::Size;
use std::io::{stdout};

use crossterm::{
    ExecutableCommand,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{enable_raw_mode, Clear, ClearType, SetSize},
    cursor::{MoveTo, Hide},
};


pub trait Drawer {
    fn resize(&self, size: &Size);
    fn draw_string(&self, x: u16, y: u16, content: &String, color: Option<Color>);
    fn draw_region(&self, x: u16, y: u16, w: u16, h: u16, filler: &String);
    fn draw_frame(&self, x: u16, y: u16, w: u16, h: u16);
}

pub struct CommandLineDrawer {}


impl CommandLineDrawer {
    pub fn new() -> CommandLineDrawer {
        CommandLineDrawer {}
    }
}

impl Drawer for CommandLineDrawer {
    fn resize(&self, size: &Size) {
        let mut stdout = stdout();
        //going into raw mode
        enable_raw_mode().unwrap();

        stdout
            .execute(Clear(ClearType::All)).unwrap()
            .execute(SetSize(size.width, size.height)).unwrap()
            .execute(Hide).unwrap();
    }

    fn draw_string(&self, x: u16, y: u16, content: &String, color: Option<Color>) {
        let mut stdout = stdout();
        stdout
            .execute(MoveTo(x, y)).unwrap();

        if color.is_none() {
            stdout
                .execute(Print(content)).unwrap();
        } else {
            stdout
                .execute(SetForegroundColor(color.unwrap())).unwrap()
                .execute(Print(content)).unwrap()
                .execute(ResetColor).unwrap();
        }
    }

    fn draw_region(&self, x: u16, y: u16, w: u16, h: u16, filler: &String) {
        let line = String::from(filler.repeat(w as usize));
        for i in y..y + h {
            self.draw_string(x, i, &line, None)
        }
    }

    fn draw_frame(&self, x: u16, y: u16, w: u16, h: u16) {
        let bottom = y + h;
        let right = x + w;
        let bottom_index = bottom - 1;
        let right_index = right - 1;
        for i in x..right {
            for j in y..bottom {
                let content =
                    if i == x && j == y {
                        Some(String::from("┏"))
                    } else if i == right_index && j == y {
                        Some(String::from("┓"))
                    } else if i == x && j == bottom_index {
                        Some(String::from("┗"))
                    } else if i == right_index && j == bottom_index {
                        Some(String::from("┛"))
                    } else if i == x || i == right_index {
                        Some(String::from("┃"))
                    } else if j == y || j == bottom_index {
                        Some(String::from("━"))
                    } else {
                        None
                    };
                if content.is_some() {
                    self.draw_string(i, j, &content.unwrap(), None);
                }
            }
        }
    }
}