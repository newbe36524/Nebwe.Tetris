use crate::drawer::Drawer;
use crate::gaming_screen::data::BlocksData;
use crate::gaming_screen::data::game_panel::{GamePanel};
use crate::gaming_screen::view::blocks_view::{BlocksRenderView, BlocksView};
use crate::tetris::{AppSettings, Point, Size};

pub(crate) struct GameView<'a> {
    start_point: Point,
    size: Size,
    drawer: &'a dyn Drawer,
    pub blocks_view: BlocksView<'a>,
}

impl GameView<'_> {
    pub fn init(&self) {
        self.drawer.draw_frame(self.start_point.x, self.start_point.y, self.size.width, self.size.height);
    }
}

impl BlocksRenderView for GameView<'_> {
    fn render_blocks(&self, data: &BlocksData) {
        self.blocks_view.render_blocks(data)
    }
}

impl GameView<'_> {
    pub(crate) fn new<'a>(region_size: Size, drawer: &'a dyn Drawer) -> GameView<'a> {
        let start_point = Point {
            x: 0,
            y: 0,
        };
        let blocks_start_point = Point {
            x: start_point.x + 2,
            y: start_point.y + 1,
        };

        let blocks_view = BlocksView::new(blocks_start_point, drawer);

        let component = GameView {
            size: region_size,
            start_point,
            drawer,
            blocks_view,
        };
        component
    }
}

