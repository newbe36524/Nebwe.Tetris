mod data;
mod view;

use crate::tetris;
use crate::drawer;
use crate::screens;

use drawer::{Drawer};
use std::time::{Duration};
use tetris::{Size, AppSettings, Point};
use screens::{RenderComponent};
use crossterm::{
    event::{poll, read, Event},
};
use rand::Rng;
use crate::gaming_screen::data::game_panel::{GamePanel, MoveDownResult, PutNewOneResult};
use crate::screens::NextScreen;


pub struct GamingScreen<'a> {
    pub settings: &'a tetris::AppSettings,
}


impl screens::LoadScreen for GamingScreen<'_> {
    fn load(&self) -> NextScreen {
        let drawer = drawer::CommandLineDrawer::new();
        let window_size = tetris::Size {
            width: self.settings.total_region.width,
            height: self.settings.total_region.height,
        };
        drawer.resize(&window_size);
        drawer.draw_region(0, 0, window_size.width, window_size.width, &String::from(" "));
        drawer.draw_frame(0, 0, window_size.width, window_size.width);

        let component = view::info::InfoComponent::new(self.settings, &drawer);
        component.init();

        let mut manager = GamePanel::new(self.settings.gaming_blocks_size,self.settings);

        let brick_collection = data::bricks::BrickCollection::new();

        let game_panel = view::game::GameComponent::new(self.settings, &drawer);
        game_panel.init();
        game_panel.render_blocks(&manager);


        let mut is_gaming = true;

        let next_screen = loop {
            if poll(Duration::from_millis(500)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => {
                        if event.code == self.settings.keyboard_control.exit {
                            break NextScreen::Welcome;
                        }
                        if event.code == self.settings.keyboard_control.pause {
                            break NextScreen::Pause;
                        }
                        if event.code == self.settings.keyboard_control.change {
                            manager.rotate_current_brick();
                            game_panel.render();
                        }
                    }
                    _ => {}
                }
            } else {
                if is_gaming {
                    // move
                    let down = &manager.move_down();
                    match down {
                        MoveDownResult::NeedNewOne => {
                            let new_brick = brick_collection.get_rand_one();
                            let put_new_one_result = &manager.put_new_one(&new_brick);
                            match put_new_one_result {
                                PutNewOneResult::Success => {}
                                PutNewOneResult::GameOver => {
                                    break NextScreen::Welcome;
                                }
                            }
                        }
                        MoveDownResult::GameOver => {
                            break NextScreen::Welcome;
                        }
                        MoveDownResult::Success => {}
                    }
                } else {}
            }
        };
        next_screen
    }
}
