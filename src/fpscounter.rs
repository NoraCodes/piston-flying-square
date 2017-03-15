use std::time::{Instant};
pub struct FPSCounter {
    frames_since_clear: u64,
    supposed_time_since_clear: f64,
    real_time_at_last_clear: Instant,
    fps: f64
}

impl FPSCounter {
    pub fn new() -> Self {
        FPSCounter {
            supposed_time_since_clear: 0.0,
            frames_since_clear: 0,
            real_time_at_last_clear: Instant::now(),
            fps: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64, sample_rate: f64) {
        self.supposed_time_since_clear += dt;
        self.frames_since_clear += 1;

        if self.supposed_time_since_clear >= sample_rate {
            let elapsed = self.real_time_at_last_clear.elapsed();
            let real_time_since_clear = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * (1.0/1000000000.0);
            self.fps = self.frames_since_clear as f64 / real_time_since_clear;
            self.supposed_time_since_clear = 0.0;
            self.frames_since_clear = 0;
            self.real_time_at_last_clear = Instant::now();
        }
    }

    pub fn get_fps(&self) -> f64 { self.fps }
}
