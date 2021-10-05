use crate::tetris;
use crate::drawer;
use crate::screens;

use drawer::Drawer;
use std::time::{Duration};
use screens::{NextScreen};

use crossterm::{
    event::{poll, read, Event},
};

pub struct PauseScreen<'a> {
    pub settings: &'a tetris::AppSettings,
}

impl screens::LoadScreen for PauseScreen<'_> {
    fn load(&mut self) -> NextScreen {
        let drawer = drawer::CommandLineDrawer::new();
        let window_size = tetris::Size {
            width: self.settings.welcome_region.width,
            height: self.settings.welcome_region.height,
        };
        drawer.draw_region(0, 0, window_size.width, window_size.width, &String::from(" "));
        drawer.draw_frame(0, 0, window_size.width, window_size.width);

        let title_content = [
            &String::from("██████  ██████  ██  ██  ██████  ██████"),
            &String::from("██  ██  ██  ██  ██  ██  ██      ██    "),
            &String::from("██████  ██████  ██  ██  ██████  ██████"),
            &String::from("██      ██  ██  ██  ██      ██  ██    "),
            &String::from("██      ██  ██  ██████  ██████  ██████"),
        ];

        // width of title text in utf8
        let title_width = 40;
        let title_x = (window_size.width - 6 - title_width) / 2;
        let title_y = (window_size.height / 2 - 5) as usize;
        for i in 0..title_content.len() {
            drawer.draw_string(title_x, (i + title_y) as u16, title_content[i], None);
        }


        let next_screen = loop {
            if poll(Duration::from_millis(500)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => {
                        if event.code == self.settings.keyboard_control.pause {
                            break NextScreen::Gaming;
                        }
                    }
                    _ => {}
                }
            } else {}
        };
        next_screen
    }
}
