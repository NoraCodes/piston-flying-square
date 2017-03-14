pub struct ColoredRect {
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2]
}

impl ColoredRect {
    pub fn new() -> Self {
        ColoredRect {
            color: [1.0, 1.0, 1.0, 1.0],
            position: [0.0, 0.0, 100.0, 100.0],
            velocity: [0.3, 0.3]
        }
    }
    pub fn update(&mut self, dt: f64, size: (f64, f64)) {
        self.color[0] = Self::update_color(dt as f32, self.color[0], 0.001);
        self.color[1] = Self::update_color(dt as f32, self.color[1], 0.002);
        self.color[2] = Self::update_color(dt as f32, self.color[2], 0.003);
        // X updates
        if self.position[0] + self.position[2] >= size.0 ||
            self.position[0] < 0.0 {
            self.velocity[0] = -self.velocity[0];
        }
        self.position[0] += self.velocity[0] * dt * 120.0;

        // Y updates
        if self.position[1] + self.position[3] >= size.1 || 
            self.position[1] < 0.0 {
            self.velocity[1] = -self.velocity[1];
        } 
        self.position[1] += self.velocity[1] * dt * 120.0;
    }
    fn update_color(dt: f32, color: f32, change: f32)->f32 {
        if color <= 0.0 {
            1.0
        } else {
            color - change * dt * 120.0
        }
    }
    pub fn change_velocity(&mut self, factor: f64) {
        self.velocity[0] *= factor;
        self.velocity[1] *= factor;
    }
}