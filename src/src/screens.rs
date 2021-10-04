pub trait LoadScreen {
    fn load(&self) -> NextScreen;
}

pub trait RenderComponent {
    fn render(&self);
    fn init(&self);
}

pub enum NextScreen {
    Welcome,
    Gaming,
    Pause,
}