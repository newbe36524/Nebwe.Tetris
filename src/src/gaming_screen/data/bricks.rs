use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::iter::Map;
use rand::Rng;
use crate::tetris::Point;

const BRICK_TYPE_COUNT: usize = 7;
const BLOCK_COUNT_OF_BRICK: usize = 4;

type TupleBrickPoints = [(u16, u16); BLOCK_COUNT_OF_BRICK];

#[derive(Eq, PartialEq, Hash)]
enum BrickType {
    O = 0x0,
    Z = 0x1,
    T = 0x2,
    S = 0x3,
    L = 0x4,
    RL = 0x5,
    I = 0x6,
}

pub static O_BRICK_POINTS: TupleBrickPoints = [(0, 0), (0, 1), (1, 0), (1, 1)];
pub static Z_BRICK_POINTS: TupleBrickPoints = [(0, 0), (1, 0), (1, 1), (2, 1)];
pub static T_BRICK_POINTS: TupleBrickPoints = [(1, 0), (0, 1), (1, 1), (2, 1)];
pub static S_BRICK_POINTS: TupleBrickPoints = [(1, 0), (2, 0), (0, 1), (1, 1)];
pub static L_BRICK_POINTS: TupleBrickPoints = [(0, 0), (0, 1), (1, 1), (2, 1)];
pub static RL_BRICK_POINTS: TupleBrickPoints = [(2, 0), (0, 1), (1, 1), (2, 1)];
pub static I_BRICK_POINTS: TupleBrickPoints = [(0, 0), (1, 0), (2, 0), (3, 0)];

impl BrickType {
    fn from_usize(num: usize) -> BrickType {
        match num {
            0 => BrickType::O,
            1 => BrickType::Z,
            2 => BrickType::T,
            3 => BrickType::S,
            4 => BrickType::L,
            5 => BrickType::RL,
            6 => BrickType::I,
            _ => BrickType::O
        }
    }
}

pub struct BrickCollection {
    all_bricks: HashMap<BrickType, Brick>,
}

impl BrickCollection {
    pub fn new() -> BrickCollection {
        let all_bricks = create_all_brick();
        BrickCollection {
            all_bricks,
        }
    }

    pub fn get_rand_one(&self) -> Brick {
        let mut rng = rand::thread_rng();
        let index = BrickType::from_usize(rng.gen_range(0..BRICK_TYPE_COUNT));
        let rotate_times = rng.gen_range(0..4);
        self.get_new_one(index, rotate_times)
    }

    fn get_new_one(&self, brick_type: BrickType, rotate_time: usize) -> Brick {
        let mut new_block = self.all_bricks[&brick_type].clone();
        for i in 0..rotate_time {
            new_block.rotate();
        }
        new_block
    }
}

type BrickPoints = [Point; BLOCK_COUNT_OF_BRICK];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Brick {
    pub points: BrickPoints,
}

impl Brick {
    pub fn new(points: TupleBrickPoints) -> Brick {
        Brick {
            points: create_points(points)
        }
    }
    pub fn rotate(&mut self) {
        let max_x = *self.points.map(|t| t.x).iter().max().unwrap();
        for index in 0..self.points.len() {
            let point = &mut self.points[index];
            let old_x: u16 = point.x;
            let old_y: u16 = point.y;
            let new_x = old_y;
            let new_y = if max_x == 0 { 0u16 } else { max_x - old_x };
            point.x = new_x;
            point.y = new_y;
        }
    }
}


fn create_points(points: TupleBrickPoints) -> BrickPoints {
    let mut vec = Vec::new();
    for index in 0..points.len() {
        let point = points[index];
        vec.push(Point::new(point.0, point.1));
    }
    vec.try_into().unwrap()
}


fn create_all_brick() -> HashMap<BrickType, Brick> {
    let mut map = HashMap::new();
    map.insert(BrickType::O, Brick { points: create_points(O_BRICK_POINTS) });
    map.insert(BrickType::Z, Brick { points: create_points(Z_BRICK_POINTS) });
    map.insert(BrickType::T, Brick { points: create_points(T_BRICK_POINTS) });
    map.insert(BrickType::S, Brick { points: create_points(S_BRICK_POINTS) });
    map.insert(BrickType::L, Brick { points: create_points(L_BRICK_POINTS) });
    map.insert(BrickType::RL, Brick { points: create_points(RL_BRICK_POINTS) });
    map.insert(BrickType::I, Brick { points: create_points(I_BRICK_POINTS) });
    map
}


#[cfg(test)]
mod tests {
    use crate::gaming_screen::data::bricks::*;
    use crate::tetris::Point;


    #[test]
    fn rotate_o() {
        let source = O_BRICK_POINTS;
        let expected = [(0, 1), (1, 1), (0, 0), (1, 0)];
        test_rotate(source, expected);
    }

    #[test]
    fn rotate_i() {
        let source = I_BRICK_POINTS;
        let expected = [(0, 3), (0, 2), (0, 1), (0, 0)];
        test_rotate(source, expected);
    }

    #[test]
    fn rotate_z() {
        let source = Z_BRICK_POINTS;
        let expected = [(0, 2), (0, 1), (1, 1), (1, 0)];
        test_rotate(source, expected);
    }

    #[test]
    fn rotate_z2() {
        let source = [(0, 2), (0, 1), (1, 1), (1, 0)];
        let expected = [(2, 1), (1, 1), (1, 0), (0, 0)];
        test_rotate(source, expected);
    }

    fn test_rotate(source: TupleBrickPoints, expected: TupleBrickPoints) {
        let source_points = create_points(source);
        let expected_points = create_points(expected);
        let mut brick = Brick {
            points: source_points
        };
        brick.rotate();
        println!("{:?}", source_points);
        println!("{:?}", brick.points);
        for i in 0..expected_points.len() {
            assert_eq!(brick.points[i], expected_points[i]);
        }
    }

    #[test]
    fn get_from_brick_collection() {
        let collection = BrickCollection::new();
        let mut brick = collection.get_new_one(BrickType::O, 0);
        brick.rotate();
        let expected_points = create_points([(0, 1), (1, 1), (0, 0), (1, 0)]);
        for i in 0..expected_points.len() {
            assert_eq!(brick.points[i], expected_points[i]);
        }

        let mut brick = collection.get_new_one(BrickType::O, 1);
        brick.rotate();
        let expected_points = create_points([(1, 1), (1, 0), (0, 1), (0, 0)]);
        for i in 0..expected_points.len() {
            assert_eq!(brick.points[i], expected_points[i]);
        }
    }
}

