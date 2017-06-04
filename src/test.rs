#[test]
fn database_connects_and_responds() {
  use diesel;
  use diesel::prelude::*;
  use diesel::sqlite::SqliteConnection;
  use dotenv::dotenv;
  use std::env;
  use super::schema::notes::dsl::*;
  use super::models::{Note, NewNote};
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let connection = SqliteConnection::establish(&db_url)
    .expect("failed to connect to database");

  let new_note = NewNote { body:"testing", };

  assert_eq!(diesel::insert(&new_note)
               .into(notes)
               .execute(&connection)
               .expect("error saving note"),
             1);
  let retrieved_note:Note = notes
    .get_result::<Note>(&connection)
    .expect("error loading notes");
  assert_eq!(retrieved_note.body, "testing");
  diesel::delete(notes.filter(body.eq("testing")))
    .execute(&connection)
    .expect("error deleting note");
  assert_eq!(notes.count().get_result(&connection), Ok(0));
}
