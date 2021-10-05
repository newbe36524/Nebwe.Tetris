use crate::drawer::Drawer;
use crate::gaming_screen::data::BlocksData;
use crate::gaming_screen::data::bricks::Brick;
use crate::gaming_screen::view::blocks_view::{BlocksRenderView, BlocksView};
use crate::tetris::{AppSettings, Point, Size};

static NEXT_BRICK_REGION_SIZE: Size = Size {
    height: 4,
    width: 4,
};

pub struct InfoView<'a> {
    start_point: Point,
    score_title_point: Point,
    score_text_point: Point,
    lines_count_title_point: Point,
    lines_count_text_point: Point,
    next_brick: Option<Brick>,
    next_brick_title_point: Point,
    next_brick_view_point: Point,
    size: Size,
    drawer: &'a dyn Drawer,
    lines_count: u32,
    score: f64,
    blocks: BlocksData,
    blocks_view: BlocksView<'a>,
}

impl InfoView<'_> {
    pub fn render_data(&self) {
        self.drawer.draw_string_on_point(self.score_text_point, &self.score.to_string(), None);
        self.drawer.draw_string_on_point(self.lines_count_text_point, &self.lines_count.to_string(), None);
        if self.next_brick.is_some() {
            self.blocks_view.render_blocks(&self.blocks);
        }
    }

    pub fn init(&self) {
        let drawer = self.drawer;
        drawer.draw_frame(self.start_point.x, self.start_point.y, self.size.width, self.size.height);
        drawer.draw_string_on_point(self.next_brick_title_point, &String::from("Next"), None);
        drawer.draw_string_on_point(self.score_title_point, &String::from("Scores"), None);
        drawer.draw_string_on_point(self.lines_count_title_point, &String::from("Lines"), None);
        self.render_data();
    }

    pub fn reset(&mut self) {
        self.score = 0f64;
        self.lines_count = 0;
        self.blocks.set_region(Point::new(0, 0), NEXT_BRICK_REGION_SIZE, false);
        self.next_brick = None;
        self.render_data();
    }
}

impl InfoView<'_> {
    pub fn new<'a>(settings: &'a AppSettings, drawer: &'a dyn Drawer) -> InfoView<'a> {
        let start_point = Point {
            x: settings.gaming_region.width,
            y: 0,
        };
        let next_brick_title_point = Point {
            x: start_point.x + 6,
            y: start_point.y + 2,
        };
        let next_brick_view_point = Point {
            y: next_brick_title_point.y + 1,
            ..next_brick_title_point
        };
        let score_title_point = Point {
            y: next_brick_view_point.y + 5,
            ..next_brick_view_point
        };
        let score_text_point = Point {
            y: score_title_point.y + 1,
            ..score_title_point
        };
        let lines_count_title_point = Point {
            y: score_text_point.y + 2,
            ..score_text_point
        };
        let lines_count_text_point = Point {
            y: lines_count_title_point.y + 1,
            ..lines_count_title_point
        };
        let view = BlocksView::new(next_brick_view_point, drawer);
        let component = InfoView {
            size: settings.info_region,
            start_point,
            score_title_point,
            score_text_point,
            lines_count_title_point,
            lines_count_text_point,
            lines_count: 0,
            score: 0f64,
            drawer,
            next_brick: None,
            next_brick_title_point,
            next_brick_view_point,
            blocks: BlocksData::new(NEXT_BRICK_REGION_SIZE),
            blocks_view: view,
        };
        component
    }

    pub fn add_lines(&mut self, count: u16) {
        self.lines_count = self.lines_count + count as u32;
        let score = match count {
            0 => 0,
            1 => 100,
            2 => 300,
            3 => 700,
            4 => 1500,
            _ => 10000
        } as f64;
        self.score = self.score + score;
    }

    pub fn update_next_brick(&mut self, brick: Brick) {
        if self.next_brick.is_some() {
            self.blocks.set_brick(&self.next_brick.unwrap(), false);
        }
        self.next_brick = Some(brick);
        self.blocks.set_brick(&self.next_brick.unwrap(), true);
    }
}