use std::time::{Instant, Duration};

#[derive(Debug)]
pub struct FrameLimiter {
    frame_duration: Duration,
    start_instant: Instant,
}

impl FrameLimiter {
    pub fn new(fps: u32) -> FrameLimiter {
        let mut f = FrameLimiter {
            frame_duration: Duration::from_secs(0),
            start_instant: Instant::now(),
        };

        f.set_rate(fps);
        f
    }

    pub fn set_rate(&mut self, fps: u32) {
        self.frame_duration = Duration::from_secs(1) / fps;
    }

    // must be called at beggining of frame or the frame may be too long
    pub fn start(&mut self) {
        self.start_instant = Instant::now();
    }

    // this is blocking so make sure last thing called in frame
    pub fn wait(&mut self) {
        while (Instant::now() - self.start_instant) < self.frame_duration {
            //wait
        }
    }
}

