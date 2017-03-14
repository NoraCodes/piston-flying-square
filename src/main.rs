extern crate piston_window;
use piston_window::*;

struct ColoredRect {
    pub color: [f32; 4],
    pub position: [f64; 4],
    velocity: [f64; 2]
}

impl ColoredRect {
    fn new() -> Self {
        ColoredRect {
            color: [1.0, 0.5, 0.25, 1.0],
            position: [0.0, 0.0, 100.0, 100.0],
            velocity: [0.3, 0.3]
        }
    }
    fn update(&mut self, dt: f64, size: (f64, f64)) {
        self.color[0] = Self::update_color(dt as f32, self.color[0]);
        self.color[1] = Self::update_color(dt as f32, self.color[1]);
        self.color[2] = Self::update_color(dt as f32, self.color[2]);
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
    fn update_color(dt: f32, color: f32)->f32 {
        if color <= 0.0 {
            1.0
        } else {
            color - 0.001 * dt * 120.0
        }
    }
}

fn main() {
    let mut rect = ColoredRect::new();
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();
    while let Some(e) = window.next() {
        match e {
            Event::Render(_) => {
                window.draw_2d(&e, |c, g| {
                    clear([1.0; 4], g); // Clear to white
                    rectangle(rect.color, // Color
                              rect.position, // Position/size
                              c.transform, g);
                });
            }
            Event::Update(u) => {
                rect.update(u.dt, (640.0, 480.0));
            }
            _ => {}
        }
    }
}
