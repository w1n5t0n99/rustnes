use std::time::{Instant, Duration};
use std::collections::VecDeque;

const MAX_SAMPLES: usize = 100;
const US_PER_SEC: u64 = 1000000;

#[derive(Debug, Clone)]
pub struct AvgDuration {
    samples: VecDeque<u64>,
    sample_sum: u64,
    cur_sample: u64,
    now: Instant,
}

impl AvgDuration {
    pub fn new() -> Self {
        let mut ad = AvgDuration {
            samples: VecDeque::with_capacity(MAX_SAMPLES),
            sample_sum: 0,
            cur_sample: 0,
            now: Instant::now(),
        };

        for _i in 0..MAX_SAMPLES {
            ad.samples.push_front(0);
        }

        ad
    }

    pub fn begin(&mut self) {
        self.now = Instant::now();
    }

    pub fn end(&mut self) {
        let dur = self.now.elapsed();
        self.cur_sample = dur.as_micros() as u64;

        self.sample_sum = self.sample_sum.saturating_sub(self.samples.pop_back().unwrap());
        self.sample_sum += self.cur_sample;
        self.samples.push_front(self.cur_sample);
    }

    pub fn reset(&mut self) {
        for s in self.samples.iter_mut() {
            *s = 0;
        }

        self.sample_sum = 0;
        self.cur_sample = 0;
        self.now = Instant::now();
    }

    pub fn get_average_duration(&self) -> Duration {
        Duration::from_micros(self.sample_sum / (MAX_SAMPLES as u64))
    }

    pub fn get_current_duration(&self) -> Duration {
        Duration::from_micros(self.cur_sample)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FrameLimit {
    target_fps: u64,
    target_frame_us: u64,
}

impl FrameLimit {
    pub fn new(fps: u64) -> Self {
        FrameLimit {
            target_fps: fps,
            target_frame_us: US_PER_SEC / fps,
        }
    }

    pub fn end_of_frame(&self, frame_duration: Duration) {
        let frame_us = frame_duration.as_micros() as u64;
        let rem_us = self.target_frame_us.saturating_sub(frame_us);
        let mut wait_us = 0;

        let now = Instant::now();
        while wait_us < rem_us {
            wait_us = now.elapsed().as_micros() as u64;
        }
    }
}