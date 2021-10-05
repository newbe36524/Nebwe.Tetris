use crate::drawer::Drawer;
use crate::gaming_screen::data::BlocksData;
use crate::tetris::Point;

pub(crate) struct BlocksView<'a> {
    blocks_start_point: Point,
    drawer: &'a dyn Drawer,
    block_text: String,
    empty_text: String,
}

pub trait BlocksRenderView {
    fn render_blocks(&self, data: &BlocksData);
}

impl BlocksRenderView for BlocksView<'_> {
    fn render_blocks(&self, data: &BlocksData) {
        let blocks = &data.blocks;
        for y in 0..blocks.len() {
            let line = &blocks[y];
            for x in 0..line.len() {
                let point = Point {
                    x: self.blocks_start_point.x + ((x * 2) as u16),
                    y: self.blocks_start_point.y + y as u16,
                };
                let text = if blocks[y][x] { &self.block_text } else {
                    &self.empty_text
                };
                self.drawer.draw_string_on_point(point, text, None);
            }
        }
    }
}

impl BlocksView<'_> {
    pub(crate) fn new<'a>(blocks_start_point: Point, drawer: &'a dyn Drawer) -> BlocksView<'a> {
        let block_text = String::from("■");
        let empty_text = String::from("  ");

        let component = BlocksView {
            blocks_start_point,
            drawer,
            block_text,
            empty_text,
        };
        component
    }
}