use crate::drawer::Drawer;
use crate::screens::RenderComponent;
use crate::tetris::{AppSettings, Point, Size};

pub struct InfoComponent<'a> {
    start_point: Point,
    score_title_point: Point,
    score_text_point: Point,
    lines_count_title_point: Point,
    lines_count_text_point: Point,
    size: Size,
    drawer: &'a dyn Drawer,
    lines_count: u32,
    score: f64,
}

impl RenderComponent for InfoComponent<'_> {
    fn render(&self) {
        todo!()
    }
    fn init(&self) {
        let drawer = self.drawer;
        drawer.draw_frame(self.start_point.x, self.start_point.y, self.size.width, self.size.height);
        drawer.draw_string_on_point(self.score_title_point, &String::from("Scores"), None);
        drawer.draw_string_on_point(self.score_text_point, &self.score.to_string(), None);
        drawer.draw_string_on_point(self.lines_count_title_point, &String::from("Lines"), None);
        drawer.draw_string_on_point(self.lines_count_text_point, &self.lines_count.to_string(), None);
    }
}

impl InfoComponent<'_> {
    pub fn new<'a>(settings: &'a AppSettings, drawer: &'a dyn Drawer) -> InfoComponent<'a> {
        let start_point = Point {
            x: settings.gaming_region.width,
            y: 0,
        };
        let score_title_point = Point {
            x: start_point.x + 6,
            y: start_point.y + 2,
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
        let component = InfoComponent {
            size: settings.info_region,
            start_point,
            score_title_point,
            score_text_point,
            lines_count_title_point,
            lines_count_text_point,
            lines_count: 0,
            score: 0f64,
            drawer,
        };
        component
    }
}