use std::slice::Iter;
use crate::gaming_screen::data::{BlocksData, TryCleanLinesResult};
use crate::gaming_screen::data::bricks::{Brick};
use crate::tetris::{AppSettings, Movement, Point, Size};


#[derive(Eq, PartialEq, Debug)]
pub enum MoveDownResult {
    Success,
    NeedNewOne,
}

#[derive(Eq, PartialEq, Debug)]
pub enum PutNewOneResult {
    Success,
    GameOver,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LiveBrick {
    pub brick: Brick,
    pub position: Point,
}

impl LiveBrick {
    fn get_projected_brick(&self) -> Brick {
        self.brick.clone().project_to_new_position(self.position)
    }
}

pub struct GamePanel {
    pub blocks: BlocksData,
    pub current_brick: Option<LiveBrick>,
    block_size: Size,
}

impl GamePanel {
    pub fn new(block_size: Size, settings: &AppSettings) -> GamePanel {
        let data = BlocksData::new(block_size);
        GamePanel {
            blocks: data,
            current_brick: None,
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
            self.set_current_brick(true);
            PutNewOneResult::Success
        } else {
            PutNewOneResult::GameOver
        }
    }

    pub fn move_down(&mut self) -> MoveDownResult {
        return if self.move_current_brick(Movement {
            y: 1,
            x: 0,
        }).is_ok() {
            MoveDownResult::Success
        } else {
            self.current_brick = None;
            MoveDownResult::NeedNewOne
        };
    }

    pub fn move_current_brick(&mut self, movement: Movement) -> Result<(), ()> {
        let current_brick = &self.current_brick;
        return if current_brick.is_none() {
            Err(())
        } else {
            let old_brick = &current_brick.unwrap();
            let new_y = old_brick.position.y as i16 + movement.y;
            let new_x = old_brick.position.x as i16 + movement.x;
            if new_x < 0 || new_y < 0 {
                Err(())
            } else {
                let new_brick = LiveBrick {
                    brick: old_brick.brick.clone(),
                    position: Point {
                        y: (new_y) as u16,
                        x: (new_x) as u16,
                    },
                };
                self.set_current_brick(false);
                if self.test_put_brick(&new_brick).is_ok() {
                    self.current_brick = Some(new_brick);
                    self.set_current_brick(true);
                    Ok(())
                } else {
                    self.set_current_brick(true);
                    Err(())
                }
            }
        };
    }

    pub fn try_clean_lines(&mut self) -> Result<TryCleanLinesResult, ()> {
        self.blocks.try_clean_lines()
    }

    pub fn rotate_current_brick(&mut self) {
        if self.current_brick.is_some() {
            self.set_current_brick(false);
            let current_brick = &self.current_brick.unwrap();

            // try to rotate
            let mut brick = current_brick.brick.clone();
            brick.rotate();
            let new_position = LiveBrick {
                brick,
                position: current_brick.position,
            };
            let result = self.test_put_brick(&new_position);
            if result.is_ok() {
                let current_brick = self.current_brick.as_mut().unwrap();
                current_brick.brick.rotate();
            }
            self.set_current_brick(true);
        }
    }

    pub fn move_current_brick_to_right(&mut self) {
        let _ = self.move_current_brick(Movement {
            x: 1,
            y: 0,
        });
    }

    pub fn move_current_brick_to_left(&mut self) {
        let _ = self.move_current_brick(Movement {
            x: -1,
            y: 0,
        });
    }

    pub fn move_current_brick_to_bottom(&mut self) {
        let movement = Movement {
            x: 0,
            y: 1,
        };
        loop {
            let result = self.move_current_brick(movement);
            if result.is_err() { break; }
        }
    }

    fn test_put_brick(&self, current_brick: &LiveBrick) -> Result<(), ()> {
        let new_position_brick = current_brick.get_projected_brick();
        self.test_points(new_position_brick.points.iter(), false)
    }

    pub fn set_region(&mut self, point: Point, size: Size, flag: bool) {
        self.blocks.set_region(point, size, flag)
    }

    fn set_current_brick(&mut self, flag: bool) {
        if self.current_brick.is_some() {
            let live_brick = &self.current_brick.unwrap();
            let new_brick = live_brick.brick.project_to_new_position(live_brick.position);
            self.set_points(new_brick.points.iter(), flag);
        }
    }

    fn test_points(&self, points: Iter<Point>, flag: bool) -> Result<(), ()> {
        self.blocks.test_points(points, flag)
    }

    fn set_points(&mut self, points: Iter<Point>, flag: bool) {
        self.blocks.set_points(points, flag)
    }

    pub(crate) fn reset(&mut self) {
        self.set_region(Point::new(0, 0), self.block_size, false);
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;
    use crate::gaming_screen::data::bricks::*;
    use crate::gaming_screen::data::game_panel::{GamePanel, LiveBrick, MoveDownResult, PutNewOneResult};
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

        // act
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

    #[test]
    fn move_down_success() {
        let mut panel = GamePanel::new(TEST_SIZE, &TEST_APP_SETTINGS);
        let new_brick = Brick::new(Z_BRICK_POINTS);
        let result = panel.put_new_one(&new_brick);
        let current_brick = &panel.current_brick.unwrap();
        let old_brick_points = current_brick.get_projected_brick();
        let old_position = current_brick.position.clone();
        assert_eq!(result, PutNewOneResult::Success);

        // act
        let down_result = panel.move_down();

        assert_eq!(down_result, MoveDownResult::Success);
        let current_brick = &panel.current_brick.unwrap();
        let new_brick_points = current_brick.get_projected_brick();
        assert_eq!(current_brick.brick, new_brick);
        assert_eq!(current_brick.position.y, old_position.y + 1);
        assert_eq!(current_brick.position.x, old_position.x);
    }

    #[test]
    fn move_down_need_new_one() {
        let mut panel = GamePanel::new(TEST_SIZE, &TEST_APP_SETTINGS);
        let new_brick = Brick::new(Z_BRICK_POINTS);
        let result = panel.put_new_one(&new_brick);
        let current_brick = &panel.current_brick.unwrap();
        let old_position = current_brick.position.clone();
        assert_eq!(result, PutNewOneResult::Success);

        let fill_size = Size {
            height: TEST_SIZE.height - 2,
            width: TEST_SIZE.width,
        };
        panel.set_region(Point::new(0, 2), fill_size, true);

        // act
        let down_result = panel.move_down();
        assert_eq!(down_result, MoveDownResult::NeedNewOne);
        assert_eq!(panel.current_brick.is_none(), true);
    }

    #[test]
    fn try_clean_lines() {
        let mut panel = GamePanel::new(TEST_SIZE, &TEST_APP_SETTINGS);
        // fill all blank with tags
        panel.set_region(Point::new(0, 0), TEST_SIZE, true);

        // act
        let result = panel.try_clean_lines();
        assert_eq!(result.is_ok(), true);
        let lines_result = result.unwrap();
        assert_eq!(lines_result.clear_lines.len() as u16, TEST_SIZE.height);

        assert_eq!(panel.blocks.blocks.len() as u16, TEST_SIZE.height);
        // all clear
        for vec in panel.blocks.blocks.iter() {
            assert_eq!(vec.iter().all(|x| { !*x }), true);
        }
    }
}
