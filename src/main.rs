extern crate clap;
#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate piston_window;
extern crate sdl2_window;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use clap::{Arg, App};

use piston_window::*;

mod document;

use document::Document;
use sdl2_window::Sdl2Window;

fn main() {
    let matches = App::new("castel")
        .version("alpha")
        .arg(Arg::with_name("v").short("v").multiple(true))
        .get_matches();
    stderrlog::new()
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(matches.occurrences_of("v") as usize)
        .init()
        .expect("failed to init logger");

    info!("logger on");
    let data = r#"{ "content": ["test", "failure"] }"#;
    info!("deserializing test data");
    let doc: Document = serde_json::from_str(data).unwrap();

    info!("opening window");
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("castel", [200; 2])
        .build()
        .unwrap();

    let font = "./fonts/cmunbmr.ttf";
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    info!("Entering main event loop");
    window.set_lazy(true);
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Q {
                window.set_should_close(true);
            }
        }
        window.draw_2d(&e, |c, g| {
            let view_size = c.get_view_size();
            let width = view_size[0];
            let height = view_size[1];
            let transform = c.transform.trans((width / 2.0), (height / 2.0));

            clear([0.0, 0.0, 0.0, 1.0], g);
            for ref s in &doc.content {
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                    .draw(s, &mut glyphs, &c.draw_state, transform, g);
            }
        });
    }
}
