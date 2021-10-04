mod tetris;
mod drawer;
mod screens;
mod welcome_screen;
mod pause_screen;
mod gaming_screen;

use crate::gaming_screen::GamingScreen;
use crate::pause_screen::PauseScreen;
use crate::screens::{LoadScreen, NextScreen};
use crate::welcome_screen::{WelcomeScreen};
use crate::tetris::{AppSettings, Size, KeyboardControl};
use crossterm::{
    event::{KeyCode},
};


fn main() {
    let gaming_blocks_size = Size {
        height: 20,
        width: 10,
    };
    let gaming_region = Size {
        height: gaming_blocks_size.height + 2,
        width: gaming_blocks_size.width * 2 + 3, // width of ■ is 2
    };
    let info_region = Size {
        height: gaming_region.height,
        width: 20,
    };
    let total_region = Size {
        height: gaming_region.height,
        width: info_region.width + gaming_region.width,
    };
    let keyboard_control = KeyboardControl {
        exit: KeyCode::Esc,
        change: KeyCode::Up,
        down: KeyCode::Down,
        pause: KeyCode::Char(' '),
        start: KeyCode::Enter,
        left: KeyCode::Left,
        right: KeyCode::Right,
    };
    let settings = &AppSettings {
        gaming_region,
        info_region,
        total_region,
        welcome_region: Size {
            height: 22,
            width: 52,
        },
        gaming_blocks_size,
        keyboard_control,
    };

    let welcome_screen = WelcomeScreen {
        settings
    };
    let pause_screen = PauseScreen {
        settings
    };
    let gaming_screen = GamingScreen {
        settings
    };
    let mut next_screen: NextScreen;
    next_screen = welcome_screen.load();
    loop {
        next_screen = match next_screen {
            NextScreen::Welcome => welcome_screen.load(),
            NextScreen::Gaming => gaming_screen.load(),
            NextScreen::Pause => pause_screen.load()
        }
    }
}
