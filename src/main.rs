#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate piston_window;
extern crate dotenv;
mod schema;
mod models;
#[cfg(test)]
mod test;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use piston_window::*;
use std::env;
fn main() {
  use schema::notes::dsl::*;
  use models::{Note, NewNote};
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let connection = SqliteConnection::establish(&db_url)
    .expect("failed to connect to database");
  let mut window:PistonWindow = WindowSettings::new("Hello World!",
                                                    [1280, 720])
      .exit_on_esc(true)
      .build()
      .unwrap();
  let factory = window.factory.clone();
  let mut glyphs = Glyphs::new("./fonts/cmunrm.ttf", factory).unwrap();
  window.set_lazy(true);
  while let Some(event) = window.next() {
    window.draw_2d(&event, |ctx, gfx| {
      clear([1.0; 4], gfx);
      rectangle([1.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 100.0, 100.0],
                ctx.transform,
                gfx);
      let text_transform = ctx.transform.trans(10.0, 100.0);
      text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw("Hello World!",
                                                           &mut glyphs,
                                                           &ctx.draw_state,
                                                           text_transform,
                                                           gfx);
    });
  }
}
