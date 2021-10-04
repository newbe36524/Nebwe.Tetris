use crate::drawer::Drawer;
use crate::gaming_screen::data::game_panel::GamePanel;
use crate::screens::RenderComponent;
use crate::tetris::{AppSettings, Point, Size};

pub(crate) struct GameComponent<'a> {
    start_point: Point,
    blocks_start_point: Point,
    size: Size,
    drawer: &'a dyn Drawer,
}

impl GameComponent<'_> {
    pub(crate) fn render_blocks(&self, manager: &GamePanel) {
        let blocks = &manager.blocks;
        let block_text = String::from("■");
        for y in 0..blocks.len() {
            let line = &blocks[y];
            for x in 0..line.len() {
                let point = Point {
                    x: self.blocks_start_point.x + ((x * 2) as u16),
                    y: self.blocks_start_point.y + y as u16,
                };
                if blocks[x][y] {
                    self.drawer.draw_string_on_point(point, &block_text, None);
                }
            }
        }
    }
}

impl RenderComponent for GameComponent<'_> {
    fn render(&self) {
        todo!()
    }
    fn init(&self) {
        self.drawer.draw_frame(self.start_point.x, self.start_point.y, self.size.width, self.size.height);
    }
}

impl GameComponent<'_> {
    pub(crate) fn new<'a>(settings: &'a AppSettings, drawer: &'a dyn Drawer) -> GameComponent<'a> {
        let mut blocks = Vec::new();
        for _line in 0..settings.gaming_blocks_size.height {
            let mut vec = Vec::new();
            for _i in 0..settings.gaming_blocks_size.width {
                vec.push(true);
            }
            blocks.push(vec)
        }
        let start_point = Point {
            x: 0,
            y: 0,
        };
        let blocks_start_point = Point {
            x: start_point.x + 2,
            y: start_point.y + 1,
        };

        let component = GameComponent {
            size: settings.gaming_region,
            start_point,
            blocks_start_point,
            drawer,
        };
        component
    }
}