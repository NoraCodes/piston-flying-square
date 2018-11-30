extern crate piston_window;
extern crate find_folder;
extern crate update_rate;
use piston_window::*;
use text::Text;
use update_rate::{RateCounter, RollingRateCounter};

mod coloredrect;
use coloredrect::ColoredRect;

fn get_glyphs(window: &PistonWindow) -> Glyphs {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("resources").unwrap();
    let ref font = assets.join("Anonymous.ttf");
    let factory = window.factory.clone();
    return Glyphs::new(font, factory, TextureSettings::new()).unwrap()
}

fn main() {
    let mut rect = ColoredRect::new();
    let mut window: PistonWindow =
        WindowSettings::new("Flying Square", [640, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut glyphs = get_glyphs(&window);

    let mut window_size: (f64, f64) = (0.0, 0.0);
    let mut fpscounter = RollingRateCounter::new(60);
    let fpsfont = Text::new(10);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            window_size = (r.width as f64, r.height as f64);
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g); // Clear to white
                rectangle(rect.color, // Color
                          rect.position, // Position/size
                          c.transform, g);
                fpsfont.draw(&format!("{:.0} FPS", fpscounter.rate()), 
                    &mut glyphs, &c.draw_state,
                    c.transform.trans(10.0, 12.0), // Set the position of the drawing
                    g).unwrap();
            });
        }

        if let Some(u) = e.update_args() {
            fpscounter.update();
            rect.update(u.dt, window_size);
        }

        if let Some(Button::Keyboard(k)) = e.press_args() {
            match k {
                Key::W => {
                    rect.change_velocity(1.1);
                },
                Key::S => {
                    rect.change_velocity(0.9);
                },
                Key::F5 => {
                    rect = ColoredRect::new();
                },
                _ => {}, // Catch all keys
            }
        }
    }
}
