extern crate piston_window;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use piston_window::*;

mod document;

use document::Document;

fn main() {
    let doc:Document = serde_json::from_str("['test','two']").unwrap();

    let mut window: PistonWindow = WindowSettings::new("piston: hello_world", [200, 200])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let font = "./fonts/cmunbmr.ttf";
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Q {
                window.set_should_close(true);
            }
        }
        window.draw_2d(&e, |c, g| {
            let transform = c.transform.trans(10.0, 100.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            for ref s in &doc.content {
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                    .draw(s, &mut glyphs, &c.draw_state, transform, g);
            }
        });
    }
}
