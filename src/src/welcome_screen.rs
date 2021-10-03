use crate::tetris;
use crate::drawer;
use crate::screens;

use drawer::Drawer;
use std::time::{Duration};

use std::io::{stdout};

use crossterm::{
    execute,
    style::{Color},
    event::{poll, read, Event, KeyCode},
    terminal::{SetTitle},
};

pub struct WelcomeScreen<'a> {
    pub settings: &'a tetris::AppSettings,
}

impl screens::LoadScreen for WelcomeScreen<'_> {
    fn load(&self) {
        let drawer = drawer::CommandLineDrawer::new();
        let window_size = tetris::Size {
            width: self.settings.welcome_region.width,
            height: self.settings.welcome_region.height,
        };
        drawer.resize(&window_size);
        drawer.draw_region(0, 0, window_size.width, window_size.width, &String::from(" "));
        drawer.draw_frame(0, 0, window_size.width, window_size.width);

        let mut stdout = stdout();
        execute!(stdout,SetTitle("Newbe.Tetris by Justin Yu")).unwrap();

        let title_content = [
            &String::from("██████  ██████  ██████  ██████  ██████  ██████"),
            &String::from("  ██    ██        ██    ██  ██    ██    ██"),
            &String::from("  ██    ██████    ██    ██████    ██    ██████"),
            &String::from("  ██    ██        ██    ██ ██     ██        ██"),
            &String::from("  ██    ██████    ██    ██  ██  ██████  ██████"),
        ];
        let colors = [
            Color::Blue,
            Color::Green,
            Color::Red
        ];
        let mut first_color = 0;

        let start_text = &String::from("Press Enter to start game!");
        let start_text_x = (window_size.width - start_text.len() as u16) / 2;
        let start_text_y = window_size.height / 2 + 5;

        drawer.draw_string(start_text_x, start_text_y, start_text, None);

        // width of title text in utf8
        let title_width = 46;
        let title_x = (window_size.width - title_width) / 2;
        let title_y = (window_size.height / 2 - 10) as usize;
        'outer: loop {
            if poll(Duration::from_millis(500)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => {
                        if event.code == KeyCode::Char(' ') {
                            break 'outer;
                        }
                    }
                    _ => {}
                }
            } else {
                first_color = (first_color + 1) % colors.len();
                let mut color_index = first_color;
                for i in 0..title_content.len() {
                    color_index = (color_index + 1) % colors.len();
                    let color = colors[color_index];
                    drawer.draw_string(title_x, (i + title_y) as u16, title_content[i], Some(color));
                }
            }
        }
    }
}
