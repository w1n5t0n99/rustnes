use std::time:: Duration;

// typical framerate would average over 1 second
const SAMPLE_SIZE: usize = 60;

#[derive(Debug)]
pub struct AverageDuration {
    samples: Vec<Duration>,
    sample_sum: Duration,
    cur_index: usize,    
}

impl AverageDuration {
    pub fn new() -> AverageDuration {
        AverageDuration {
            samples: vec![Duration::from_secs(0); SAMPLE_SIZE],
            sample_sum: Duration::from_secs(0),
            cur_index: 0,
        }
    }

    pub fn reset(&mut self) {
        for i in self.samples.iter_mut() {
            *i = Duration::from_secs(0);
        }
    }

    pub fn update(&mut self, duration: Duration) {
        self.sample_sum -= self.samples[self.cur_index];
        self.samples[self.cur_index] = duration;
        self.sample_sum += duration;

        self.cur_index = (self.cur_index + 1) % SAMPLE_SIZE;
    }

    pub fn get_average_duration(&self) -> Duration {
        self.sample_sum / (SAMPLE_SIZE as u32)
    }
}
