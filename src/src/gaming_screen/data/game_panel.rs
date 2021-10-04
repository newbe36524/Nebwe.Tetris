use rand::Rng;
use crate::gaming_screen::data::bricks::Brick;
use crate::tetris::{AppSettings, Point, Size};


pub enum MoveDownResult {
    Success,
    NeedNewOne,
    GameOver,
}

pub enum PutNewOneResult {
    Success,
    GameOver,
}

pub struct LiveBrick {
    brick: Brick,
    position: Point,
}

pub struct GamePanel<'a> {
    pub blocks: Vec<Vec<bool>>,
    current_brick: Option<LiveBrick>,
    settings: &'a AppSettings,
    block_size: Size,
}

impl GamePanel<'_> {
    pub fn new(block_size: Size, settings: &AppSettings) -> GamePanel<'_> {
        let mut blocks = Vec::new();
        for _line in 0..block_size.height {
            let mut vec = Vec::new();
            for _i in 0..block_size.width {
                vec.push(false);
            }
            blocks.push(vec)
        }
        GamePanel {
            blocks,
            current_brick: None,
            settings,
            block_size,
        }
    }
    pub fn put_new_one(&mut self, new_brick: &Brick) -> PutNewOneResult {
        let live_brick = LiveBrick {
            brick: new_brick.clone(),
            position: Point::new(self.block_size.width / 2 - 2, 0),
        };
        let result = self.test_put_brick(&live_brick);
        if result.is_ok() {
            self.current_brick = Some(live_brick);
            PutNewOneResult::Success
        } else {
            PutNewOneResult::GameOver
        }
    }
    pub fn move_down(&self) -> MoveDownResult {
        if self.current_brick.is_none() {
            MoveDownResult::NeedNewOne
        } else {
            MoveDownResult::Success
        }
    }
    pub fn rotate_current_brick(&mut self) {
        if self.current_brick.is_some() {
            let current_brick = self.current_brick.as_mut().unwrap();
            current_brick.brick.rotate();
        }
    }

    fn test_put_brick(&self, current_brick: &LiveBrick) -> Result<(), ()> {
        for point in current_brick.brick.points.iter() {
            let new_point = Point {
                x: point.x + current_brick.position.x,
                y: point.y + current_brick.position.y,
            };
            if self.blocks[new_point.x as usize][new_point.y as usize] {
                return Err(());
            }
        }
        Ok(())
    }

    fn set_region(&mut self, point: Point, size: Size, flag: bool) {
        let mut vec = &mut self.blocks;
        for y in point.y..point.y + size.height {
            let mut line = &mut vec[y as usize];
            for x in point.x..point.x + size.width {
                line[x as usize] = flag;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;
    use crate::gaming_screen::data::bricks::*;
    use crate::gaming_screen::data::game_panel::{GamePanel, LiveBrick};
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
    fn put_new_one() {
        let mut panel = GamePanel::new(TEST_SIZE, &TEST_APP_SETTINGS);
        let new_brick = Brick::new(Z_BRICK_POINTS);
        panel.put_new_one(&new_brick);
        let brick = panel.current_brick.as_ref().unwrap();
        assert_eq!(brick.brick, new_brick);
        assert_eq!(brick.position, Point::new(TEST_SIZE.width / 2 - 2, 0));
    }

    #[test]
    fn test_put_brick() {
        let mut panel = GamePanel::new(TEST_SIZE, &TEST_APP_SETTINGS);
        let new_brick = Brick::new(Z_BRICK_POINTS);
        let live_brick = LiveBrick {
            brick: new_brick,
            position: Point {
                x: 0,
                y: 0,
            },
        };
        assert_eq!(panel.test_put_brick(&live_brick).is_ok(), true);

        // fill all blank with tags

        panel.set_region(Point::new(0, 0), TEST_SIZE, true);

        assert_eq!(panel.test_put_brick(&live_brick).is_ok(), false);
    }
}
