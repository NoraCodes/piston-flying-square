extern crate piston_window;
extern crate find_folder;
use piston_window::*;
use text::Text;

mod coloredrect;
use coloredrect::ColoredRect;

mod fpscounter;
use fpscounter::FPSCounter;

fn get_glyphs(window: &PistonWindow) -> Glyphs {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("resources").unwrap();
    let ref font = assets.join("Anonymous.ttf");
    let factory = window.factory.clone();
    return Glyphs::new(font, factory).unwrap()
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
    let mut fpscounter = FPSCounter::new();
    let fpsfont = Text::new(10);

    while let Some(e) = window.next() {
        match e {
            Event::Render(r) => {
                window_size = (r.width as f64, r.height as f64);
                window.draw_2d(&e, |c, g| {
                    clear([1.0; 4], g); // Clear to white
                    rectangle(rect.color, // Color
                              rect.position, // Position/size
                              c.transform, g);
                    fpsfont.draw(&format!("{:.0} FPS", fpscounter.get_fps()), 
                        &mut glyphs, &c.draw_state,
                        c.transform.trans(10.0, 12.0), // Set the position of the drawing
                        g);
                });
            }
            Event::Update(u) => {
                rect.update(u.dt, window_size);
                fpscounter.update(u.dt, 0.25);
            }
            Event::Input(i) => {
                match i {
                    Input::Press(b) => {
                       match b {
                           Button::Keyboard(k) => {
                               match k {
                                   Key::W => {
                                       rect.change_velocity(1.1);
                                   }
                                   Key::S => {
                                       rect.change_velocity(0.9);
                                   }
                                   Key::F5 => {
                                       rect = ColoredRect::new();
                                   }
                                   _ => {} // Catch all keys
                               };
                           }
                           _ => {} // Catch non-keyboard buttons
                       };
                    }
                    _ => {} // Catch non-buttons actions
                }
            }
            _ => {} // Catch uninteresting events
        }
    }
}
