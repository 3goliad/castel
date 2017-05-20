#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate glutin;
extern crate libc;

mod gl {
    #![allow(non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

mod models {
    use super::schema::notes;

    #[derive(Queryable)]
    pub struct Note {
        pub id: Option<i32>,
        pub body: String,
    }

    #[derive(Insertable)]
    #[table_name="notes"]
    pub struct NewNote<'a> {
        pub body: &'a str,
    }
}

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

fn main() {
    use schema::notes::dsl::*;
    use models::{Note, NewNote};

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&db_url).expect("failed to connect to database");

    let new_note = NewNote {
        body: "testing",
    };

    assert_eq!(diesel::insert(&new_note).into(notes).execute(&connection).expect("error saving note"), 1);
    let retrieved_note: Note = notes.get_result::<Note>(&connection).expect("error loading notes");
    assert_eq!(retrieved_note.body, "testing");
    diesel::delete(notes.filter(body.eq("testing"))).execute(&connection).expect("error deleting note");
    assert_eq!(notes.count().get_result(&connection), Ok(0));

    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!".to_string())
        .with_dimensions(1024, 768)
        .with_vsync()
        .build(&events_loop)
        .unwrap();

    unsafe { window.make_current() }.unwrap();

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
                                    glutin::Event::WindowEvent {
                                        event: glutin::WindowEvent::Closed, ..
                                    } => {
                                        running = false;
                                    }
                                    _ => (),
                                });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers().unwrap();
    }
}
