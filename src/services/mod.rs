use std::error::Error;
use rand::Rng;
use chrono::Utc;
use crate::structs::publisher::Publisher;
mod connection; 

fn raffle() -> (usize, String, String) {
  const ORDER: [&str; 2] = ["ASC", "DESC"];
  const ITEMS: [&str; 3] = ["type", "name", "updated_at"];
  
  let mut rng = rand::thread_rng();
  let item_index: usize = rng.gen_range(0..ITEMS.len());
  let order_index: usize = rng.gen_range(0..ORDER.len());
  
  (item_index, ITEMS[item_index].to_owned(), ORDER[order_index].to_owned())
}

fn get_timestamp() -> String {
  let now = Utc::now();
  now.format("%Y-%m-%d %H:%M:%S").to_string()
}

//Publishers
pub fn create_publisher(publisher:Publisher) -> Result<usize, Box<dyn Error>> {
  let conn = connection::sqlite().unwrap();
  let query = conn.execute(
    "INSERT INTO publishers (name, type, gender) VALUES (?1, ?2, ?3)",
    (publisher.name, publisher.r#type, publisher.gender),
  );

  match query {
    Ok(value) =>
      Ok(value),
    Err(erro) =>
      Err(erro.into()),
  }
}

pub fn update_publisher(id:&str, publisher:Publisher) -> Result<usize, Box<dyn Error>> {
  let conn = connection::sqlite().unwrap();
  let now = get_timestamp();
  let query = conn.execute(
    "UPDATE publishers SET name = ?2, type =?3, gender = ?4, active = ?5, updated_at = ?6 WHERE id = ?1 LIMIT 1",
    (id, publisher.name, publisher.r#type, publisher.gender, publisher.active, now),
  );

  match query {
    Ok(value) =>
      Ok(value),
    Err(erro) =>
      Err(erro.into()),
  }
}

pub fn get_publisher(id:&str) -> Result<String, Box<dyn Error>> {
  let conn = connection::sqlite().unwrap();
  
  let publisher = conn.query_row(
    "SELECT * FROM publishers WHERE id = ? LIMIT 1",
    &[id],
    |row| {
        Ok(Publisher {
          id: row.get(0)?,
          name: row.get(1)?,
          r#type: row.get(2)?,
          gender: row.get_unwrap(3),
          amount: row.get_unwrap(4),
          active: row.get_unwrap(5),
          updated_at: row.get_unwrap(6),
          created_at: row.get_unwrap(7),
      })
    },
  )?;

  match serde_json::to_string(&publisher) {
    Ok(json) =>  Ok(json),
    Err(erro) =>  Err(erro.into()),
  }

}

fn list_raffled_publisher(id: u8) -> (u8, u32) {
  
  let (index, item, order) = raffle();

  let conn = connection::sqlite().unwrap();
  let publishers: Vec<Publisher> = conn
  .prepare("SELECT * FROM publishers WHERE active = true AND NOT id = ?3 ORDER BY ?1 ?2 LIMIT 3").unwrap()
  .query_map([ item, order, id.to_string()], |row| { 
      Ok(Publisher {
          id: row.get(0)?,
          name: row.get(1)?,
          r#type: row.get(2)?,
          gender: row.get_unwrap(3),
          amount: row.get_unwrap(4),
          active: row.get_unwrap(5),
          updated_at: row.get_unwrap(6),
          created_at: row.get_unwrap(7),
      })
  }).unwrap()
  .filter_map(Result::ok)
  .collect();

  (publishers[index].id.unwrap(), publishers[index].amount.unwrap())

}

pub fn list_publisher() -> Result<String, Box<dyn Error>> {
  let conn = connection::sqlite().unwrap();
  let publishers: Vec<Publisher> = conn
  .prepare("SELECT * FROM publishers WHERE active = true ")?
  .query_map([], |row| { 
      Ok(Publisher {
          id: row.get(0)?,
          name: row.get(1)?,
          r#type: row.get(2)?,
          gender: row.get_unwrap(3),
          amount: row.get_unwrap(4),
          active: row.get_unwrap(5),
          updated_at: row.get_unwrap(6),
          created_at: row.get_unwrap(7),
      })
  })?
  .filter_map(Result::ok)
  .collect();

  match serde_json::to_string(&publishers) {
    Ok(json) =>  Ok(json),
    Err(erro) =>  Err(erro.into()),
  }
}

pub fn delete_publisher(id:&str) -> Result<usize, Box<dyn Error>> {
  
  let conn = connection::sqlite().unwrap();
  let query = conn.execute("DELETE FROM publishers WHERE id = ? LIMIT 1", [&id]);

  match query {
    Ok(value) =>
      Ok(value),
    Err(erro) =>
      Err(erro.into()),
  }
}

//Presentations
pub fn create_presentations(length:&str) -> Result<String, Box<dyn Error>> {
  let querys: Vec<Result<usize, String>> = vec![];
  
  for i in 0..length.parse::<u8>().unwrap() {

    let main = list_raffled_publisher(0);
    let helper = list_raffled_publisher(main.0);

    let conn = connection::sqlite().unwrap();
    let query: Result<usize, rusqlite::Error> = conn.execute(
      "INSERT INTO presentation (main, helper) VALUES (?1, ?2)",
      (main.0, helper.0),
    );
  }
  match serde_json::to_string(&querys) {
    Ok(json) =>  Ok(json),
    Err(erro) =>  Err(erro.into()),
  }



}
