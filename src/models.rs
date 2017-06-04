use super::schema::notes;

#[derive(Queryable)]
pub struct Note {
  pub id:Option<i32>,
  pub body:String,
}

#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote<'a> {
  pub body:&'a str,
}
