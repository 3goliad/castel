#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate glutin;
extern crate libc;
extern crate freetype;
mod gl {
  #![allow(non_upper_case_globals)]
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
mod schema;
mod models;
#[cfg(test)]
mod test;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
fn main() {
  use schema::notes::dsl::*;
  use models::{Note, NewNote};
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let connection = SqliteConnection::establish(&db_url)
    .expect("failed to connect to database");
  let ftlib = freetype::Library::init().unwrap();
  let face = lib.new_face("./fonts/cmusr.ttf", 0).unwrap();
  let face.set_char_size(40 * 64, 0, 50, 0).unwrap();
  face.load_char('A').unwrap();
  let glyph = face.glyph();
  let events_loop = glutin::EventsLoop::new();
  let window = glutin::Window::new(&events_loop);
  unsafe { window.make_current() }.unwrap();
  unsafe {
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 1.0, 0.0, 1.0);
  }
  loop {
    let mut exited = false;
    events_loop.poll_events(|event| match event {
                              glutin::Event::WindowEvent {
                                event: glutin::WindowEvent::Closed, ..
                              } => {
                                exited = true;
                              },
                              _ => (),
                            });
    if exited {
      break;
    }
    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    window.swap_buffers().unwrap();
  }
}
