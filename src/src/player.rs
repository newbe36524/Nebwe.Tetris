use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};
use rodio::{Decoder, OutputStream, OutputStreamHandle, source::Source};

pub enum SoundTypes {
    Welcome,
    Gaming,
    Pause,
    Change,
    LineClean(u16),
}

pub struct Player {
    pub stream_handle: Option<OutputStreamHandle>,
    pub stream: Option<OutputStream>,
    duration_limit: Duration,
    last_time: Instant,
}

impl Player {
    pub fn new() -> Player {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Player {
            stream_handle: Some(stream_handle),
            stream: Some(stream),
            duration_limit: Duration::from_millis(500),
            last_time: Instant::now(),
        }
    }
    pub fn play(&mut self, t: SoundTypes) {
        if self.check_limit().is_err() {
            ()
        }
        // Load a sound from a file, using a path relative to Cargo.toml
        let result = File::open(self.get_filename(t));
        if result.is_ok() {
            let file = BufReader::new(result.unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            if self.stream_handle.is_some() {
                let handle = self.stream_handle.as_ref().unwrap();
                handle.play_raw(source.convert_samples());
            }
        }
    }

    pub fn play_repeat(&mut self, t: SoundTypes) {
        if self.check_limit().is_err() {
            ()
        }
        let result = File::open(self.get_filename(t));
        if result.is_ok() {
            let file = BufReader::new(result.unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            let source = source.repeat_infinite();
            if self.stream_handle.is_some() {
                let handle = self.stream_handle.as_ref().unwrap();
                handle.play_raw(source.convert_samples());
            }
        }
    }

    pub fn stop(&mut self, t: SoundTypes) {
        self.stream = None;
        self.stream_handle = None;
    }

    fn check_limit(&mut self) -> Result<(), ()> {
        let now = Instant::now();
        return if (now - self.last_time) < self.duration_limit {
            Err(())
        } else {
            self.last_time = now;
            Ok(())
        };
    }

    fn get_filename(&self, t: SoundTypes) -> &str {
        match t {
            SoundTypes::Welcome => {
                "asset/welcome.mp3"
            }
            SoundTypes::Gaming => {
                "asset/game.mp3"
            }
            SoundTypes::Pause => {
                "asset/pause.mp3"
            }
            SoundTypes::Change => {
                "asset/change.mp3"
            }
            SoundTypes::LineClean(count) => {
                match count {
                    1 => "asset/line_clean_1.mp3",
                    2 => "asset/line_clean_2.mp3",
                    3 => "asset/line_clean_3.mp3",
                    4 => "asset/line_clean_4.mp3",
                    _ => {
                        "asset/line_clean_4.mp3"
                    }
                }
            }
        }
    }
}
