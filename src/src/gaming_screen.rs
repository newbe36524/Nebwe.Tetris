mod data;
mod view;

use crate::tetris;
use crate::drawer;
use crate::screens;
use crate::player;

use drawer::{Drawer};
use std::time::{Duration, Instant};
use crossterm::{
    event::{poll, read, Event},
};
use crate::drawer::CommandLineDrawer;
use crate::gaming_screen::data::bricks::{Brick, BrickCollection};
use crate::gaming_screen::data::game_panel::{GamePanel, MoveDownResult, PutNewOneResult};
use crate::gaming_screen::view::blocks_view::{BlocksRenderView, BlocksView};
use crate::gaming_screen::view::game_view::GameView;
use crate::gaming_screen::view::info_view::InfoView;
use crate::player::{Player, SoundTypes};
use crate::screens::NextScreen;
use crate::tetris::{AppSettings, Point, Size};

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, source::Source};

struct GamingScreenCore {
    manager: GamePanel,
    brick_collection: BrickCollection,
    next_brick: Option<Brick>,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum NextResult {
    Success,
    LineClear(u16, Brick),
    NewBrickPutIn(Brick),
    GameOver,
}


impl GamingScreenCore {
    fn next(&mut self) -> NextResult {
        // move
        let down = self.manager.move_down();
        return match down {
            MoveDownResult::NeedNewOne => {
                let line_clear_result = self.manager.try_clean_lines();
                let put_new_one_result = self.manager.put_new_one(&self.next_brick.unwrap());
                return match put_new_one_result {
                    PutNewOneResult::Success => {
                        let new_brick = self.brick_collection.get_rand_one();
                        self.next_brick = Some(new_brick);
                        if line_clear_result.is_ok() {
                            NextResult::LineClear(line_clear_result.unwrap().clear_lines.len() as u16, new_brick)
                        } else {
                            NextResult::NewBrickPutIn(new_brick)
                        }
                    }
                    PutNewOneResult::GameOver => {
                        return NextResult::GameOver;
                    }
                };
            }
            MoveDownResult::Success => {
                NextResult::Success
            }
        };
    }

    fn rotate(&mut self) {
        self.manager.rotate_current_brick()
    }

    fn move_left(&mut self) {
        self.manager.move_current_brick_to_left()
    }

    fn move_right(&mut self) {
        self.manager.move_current_brick_to_right();
    }

    fn move_bottom(&mut self) {
        self.manager.move_current_brick_to_bottom()
    }

    fn render_to(&self, component: &dyn BlocksRenderView) {
        component.render_blocks(&self.manager.blocks)
    }

    fn reset(&mut self) {
        self.manager.reset();
        let brick = self.brick_collection.get_rand_one();
        self.next_brick = Some(brick);
    }
}

pub struct GamingScreen<'a> {
    pub settings: &'a AppSettings,
    drawer: &'a dyn Drawer,
    info_view: InfoView<'a>,
    game_view: GameView<'a>,
    core: GamingScreenCore,
}

impl GamingScreen<'_> {
    pub fn new<'a>(settings: &'a AppSettings, drawer: &'a dyn Drawer) -> GamingScreen<'a> {
        let game_view = GameView::new(settings.gaming_region, drawer);
        let mut game_panel = GamePanel::new(settings.gaming_blocks_size, &settings);
        let collection = data::bricks::BrickCollection::new();
        let mut core = GamingScreenCore {
            manager: game_panel,
            brick_collection: collection,
            next_brick: None,
        };

        core.reset();

        // test code
        // let size = Size {
        //     height: 4,
        //     width: settings.gaming_blocks_size.width,
        // };
        // let point = Point::new(0, settings.gaming_blocks_size.height - 4);
        // core.manager.set_region(point, size, true);

        let info_view = InfoView::new(&settings, drawer);

        GamingScreen {
            settings,
            drawer,
            info_view,
            game_view,
            core,
        }
    }

    fn init(&self) {
        let drawer = self.drawer;
        let window_size = self.settings.total_region;
        drawer.resize(&window_size);
        drawer.draw_region(0, 0, window_size.width, window_size.width, &String::from(" "));
        drawer.draw_frame(0, 0, window_size.width, window_size.width);

        self.info_view.init();
        self.game_view.init();
    }

    fn move_right(&mut self) {
        self.core.move_right();
        self.core.render_to(&self.game_view);
    }

    fn move_left(&mut self) {
        self.core.move_left();
        self.core.render_to(&self.game_view);
    }

    fn move_bottom(&mut self) {
        self.core.move_bottom();
        self.core.render_to(&self.game_view);
    }

    fn rotate(&mut self) {
        self.core.rotate();
        self.core.render_to(&self.game_view);
    }

    fn time_tick(&mut self) -> NextResult {
        let result = self.core.next();
        match result {
            NextResult::Success => {
                self.core.render_to(&self.game_view);
            }
            NextResult::GameOver => {}
            NextResult::LineClear(count, next_brick) => {
                self.core.render_to(&self.game_view);
                self.info_view.add_lines(count);
                self.info_view.update_next_brick(next_brick);
                self.info_view.render_data();
            }
            NextResult::NewBrickPutIn(next_brick) => {
                self.core.render_to(&self.game_view);
                self.info_view.update_next_brick(next_brick);
                self.info_view.render_data();
            }
        };
        result
    }

    fn reset(&mut self) {
        self.core.reset();
        self.core.render_to(&self.game_view);
        let brick = &self.core.next_brick.unwrap();
        self.info_view.reset();
        self.info_view.update_next_brick(brick.clone());
    }
}

impl screens::LoadScreen for GamingScreen<'_> {
    fn load(&mut self) -> NextScreen {
        self.init();
        let mut player = Player::new();
        player.play_repeat(SoundTypes::Gaming);

        let mut last_tick_time = Instant::now();
        let mut last_action_time = Instant::now();
        let next_screen = loop {
            let now = Instant::now();
            if poll(Duration::from_millis(100)).unwrap() {
                if now - last_action_time > Duration::from_millis(100) {
                    last_action_time = now;
                    match read().unwrap() {
                        Event::Key(event) => {
                            if event.code == self.settings.keyboard_control.exit {
                                break NextScreen::Welcome;
                            }
                            if event.code == self.settings.keyboard_control.pause {
                                break NextScreen::Pause;
                            }
                            if event.code == self.settings.keyboard_control.right {
                                self.move_right();
                            }
                            if event.code == self.settings.keyboard_control.left {
                                self.move_left();
                            }
                            if event.code == self.settings.keyboard_control.down {
                                self.move_bottom();
                            }
                            if event.code == self.settings.keyboard_control.change {
                                self.rotate();
                                player.play(SoundTypes::Change);
                            }
                        }
                        _ => {}
                    }
                }
            }
            if now - last_tick_time > Duration::from_millis(500) {
                last_tick_time = now;
                let next_result = self.time_tick();
                match next_result {
                    NextResult::GameOver => {
                        self.reset();
                    }
                    NextResult::LineClear(count, _) => {
                        player.play(SoundTypes::LineClean(count));
                    }
                    _ => {}
                }
            }
        };
        next_screen
    }
}


#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;
    use crate::{drawer, tetris};
    use crate::drawer::{Drawer, NothingDrawer};
    use crate::gaming_screen::data::bricks::*;
    use crate::gaming_screen::data::game_panel::{GamePanel, LiveBrick, MoveDownResult, PutNewOneResult};
    use crate::gaming_screen::{data, GamingScreenCore, NextResult, view};
    use crate::tetris::*;

    static TEST_SIZE: Size = Size {
        width: 10,
        height: 20,
    };


    static TEST_APP_SETTINGS: AppSettings = AppSettings {
        gaming_region: Size {
            height: 20,
            width: 10,
        },
        info_region: Size {
            height: 20,
            width: 20,
        },
        total_region: Size {
            height: 20,
            width: 30,
        },
        welcome_region: Size {
            height: 22,
            width: 52,
        },
        gaming_blocks_size: TEST_SIZE,
        keyboard_control: KeyboardControl {
            exit: KeyCode::Esc,
            change: KeyCode::Up,
            down: KeyCode::Down,
            pause: KeyCode::Char(' '),
            start: KeyCode::Enter,
            left: KeyCode::Left,
            right: KeyCode::Right,
        },
    };

    #[test]
    fn next() {
        let drawer = NothingDrawer::new();
        let component = view::info_view::InfoView::new(&TEST_APP_SETTINGS, &drawer);
        component.init();

        let game_panel = view::game_view::GameView::new(TEST_APP_SETTINGS.gaming_region, &drawer);
        game_panel.init();

        let collection = data::bricks::BrickCollection::new();
        let next_brick = collection.get_rand_one();
        let mut core = GamingScreenCore {
            manager: GamePanel::new(TEST_APP_SETTINGS.gaming_blocks_size, &TEST_APP_SETTINGS),
            brick_collection: collection,
            next_brick: Some(next_brick),
        };

        let result = core.next();
        match result {
            NextResult::NewBrickPutIn(_) => { assert!(true) }
            _ => { assert!(false) }
        }

        let panel = &core.manager;
        let brick = &panel.current_brick.unwrap();
        assert_eq!(brick.position.y, 0);
        assert_ne!(brick.position.x, 0);
    }
}
