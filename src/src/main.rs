mod tetris;
mod drawer;
mod screens;
mod welcome_screen;
mod pause_screen;

use crate::pause_screen::PauseScreen;
use crate::screens::{LoadScreen};
use crate::welcome_screen::{WelcomeScreen};
use crate::tetris::{AppSettings, Size};


fn main() {
    let settings = &AppSettings {
        gaming_region: Size {
            height: 20,
            width: 10,
        },
        info_region: Size {
            height: 20,
            width: 5,
        },
        total_region: Size {
            height: 22,
            width: 19,
        },
        welcome_region: Size {
            height: 50,
            width: 50,
        },
    };

    let screen = WelcomeScreen {
        settings
    };
    let pause_screen = PauseScreen {
        settings
    };
    loop {
        screen.load();
        pause_screen.load();
    }
}
