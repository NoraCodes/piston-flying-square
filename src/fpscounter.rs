use std::collections::VecDeque;

pub struct FPSCounter {
    queue: VecDeque<f64>,
    fps: f64
}
impl FPSCounter {
    pub fn new() -> Self {
        FPSCounter {
            queue: VecDeque::<f64>::with_capacity(120),
            fps: 0.0
        }
    }
    pub fn update(&mut self, dt: f64, sample_rate: f64) {
        self.queue.push_back(dt);
        let time_since_clear = self.queue.iter().fold(0.0, |a, i| a + i);
        if time_since_clear >= sample_rate {
            self.fps = self.queue.len() as f64 / time_since_clear;
            self.queue = VecDeque::with_capacity(120);
        }
    }
    pub fn get_fps(&self) -> f64 { self.fps }
}