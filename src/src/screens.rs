pub trait LoadScreen {
    fn load(&mut self) -> NextScreen;
}

pub enum NextScreen {
    Welcome,
    Gaming,
    Pause,
}