use crate::tetris::{Point, Size};
use std::slice::Iter;
use crate::gaming_screen::data::bricks::Brick;

pub(crate) mod bricks;
pub(crate) mod game_panel;


pub struct BlocksData {
    pub blocks: Vec<Vec<bool>>,
    block_size: Size,
}

fn create_empty_line(width: u16) -> Vec<bool> {
    [false].repeat(width as usize).iter().map(|x| { *x }).collect()
}

pub struct TryCleanLinesResult {
    pub clear_lines: Vec<u16>,
}

impl BlocksData {
    pub fn new(block_size: Size) -> BlocksData {
        let mut blocks = Vec::new();
        for _line in 0..block_size.height {
            blocks.push(create_empty_line(block_size.width))
        }
        BlocksData {
            blocks,
            block_size,
        }
    }

    pub fn test_points(&self, points: Iter<Point>, flag: bool) -> Result<(), ()> {
        for point in points {
            let line = self.blocks.get(point.y as usize);
            if line.is_none() {
                return Err(());
            }
            let new_position = line.unwrap().get(point.x as usize);
            if new_position.is_none() {
                return Err(());
            }
            if *new_position.unwrap() != flag {
                return Err(());
            }
        }
        Ok(())
    }

    pub fn set_points(&mut self, points: Iter<Point>, flag: bool) {
        for point in points {
            self.blocks[point.y as usize][point.x as usize] = flag;
        }
    }

    pub fn set_brick(&mut self, brick: &Brick, flag: bool) {
        self.set_points(brick.points.iter(), flag)
    }

    pub fn set_region(&mut self, point: Point, size: Size, flag: bool) {
        let vec = &mut self.blocks;
        for y in point.y..point.y + size.height {
            let line = &mut vec[y as usize];
            for x in point.x..point.x + size.width {
                line[x as usize] = flag;
            }
        }
    }

    pub fn try_clean_lines(&mut self) -> Result<TryCleanLinesResult, ()> {
        let mut result = Vec::new();
        for y in 0..self.blocks.len() {
            let line = &self.blocks[y];
            if line.iter().all(|x| { *x }) {
                result.push(y);
            }
        }
        return if result.len() > 0 {
            result.reverse();
            for index in result.iter() {
                self.blocks.remove(*index);
            }
            for _index in result.iter() {
                self.blocks.insert(0, create_empty_line(self.block_size.width));
            }
            Ok(TryCleanLinesResult {
                clear_lines: result.iter().map(|i| { *i as u16 }).collect()
            })
        } else {
            Err(())
        };
    }
}