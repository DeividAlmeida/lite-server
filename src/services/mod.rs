use std::{error::Error, thread};
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

fn sum_puplisher_amount (publisher:Publisher) -> u32{
  let new_amout = publisher.amount.unwrap() + 1;
  new_amout
}

//Publishers
pub fn create_publisher(publisher:Publisher) -> Result<usize, Box<dyn Error>> {
  let conn = connection::connect_sqlite().unwrap();
  let now = get_timestamp();
  let query = conn.execute(
    "INSERT INTO publishers (name, type, gender, updated_at, created_at) VALUES (?1, ?2, ?3, ?4, ?4)",
    (publisher.name, publisher.r#type, publisher.gender, now),
  );

  connection::disconnect_sqlite(conn).unwrap();

  match query {
    Ok(value) =>
      Ok(value),
    Err(erro) =>
      Err(erro.into()),
  }
}

pub fn update_publisher(id:&str, publisher:Publisher) -> Result<usize, Box<dyn Error>> {
  let conn = connection::connect_sqlite().unwrap();
  let now = get_timestamp();
  let query = conn.execute(
    "UPDATE publishers SET name = ?2, type =?3, gender = ?4, active = ?5, updated_at = ?6 WHERE id = ?1",
    (id, publisher.name, publisher.r#type, publisher.gender, publisher.active, now),
  );

  connection::disconnect_sqlite(conn).unwrap();
  
  match query {
    Ok(value) =>
      Ok(value),
    Err(erro) =>
      Err(erro.into()),
  }
}

pub fn get_publisher(id:&str) -> Result<String, Box<dyn Error>> {
  let conn = connection::connect_sqlite().unwrap();
  
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

  connection::disconnect_sqlite(conn).unwrap();

  match serde_json::to_string(&publisher) {
    Ok(json) =>  Ok(json),
    Err(erro) =>  Err(erro.into()),
  }

}

fn list_raffled_publisher(id: u8, operator:String, gender:String) -> Option<Publisher> {
  
  let (index, item, order) = raffle();

  let conn = connection::connect_sqlite().unwrap();
  let query = format!("SELECT * FROM publishers WHERE NOT id = ?3 AND type {} 2  AND gender = ?4  ORDER BY amount ASC, ?1, ?2 LIMIT 3", operator);
  let publishers: Vec<Publisher> = conn
  .prepare(&query).unwrap()
  .query_map([item, order, id.to_string(), gender], |row| { 
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
  }).unwrap().filter_map(Result::ok)
  .collect();

  connection::disconnect_sqlite(conn).unwrap();
  
  match &publishers.len() > &index && publishers.len() > 0 {
    true => Some(publishers[index].clone()),
    false => None,
  }
  
  // Ok(Publisher { id: Some(5), name: "Teste".to_string(), r#type: 1, gender: "male".to_string(), amount: Some(2), active: Some(true), updated_at: None, created_at: None })
  
}

pub fn list_publisher() -> Result<String, Box<dyn Error>> {
  let conn = connection::connect_sqlite().unwrap();
  let publishers: Vec<Publisher> = conn
  .prepare("SELECT * FROM publishers")?
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

  connection::disconnect_sqlite(conn).unwrap();

  match serde_json::to_string(&publishers) {
    Ok(json) =>  Ok(json),
    Err(erro) =>  Err(erro.into()),
  }
}

pub fn delete_publisher(id:&str) -> Result<usize, Box<dyn Error>> {
  
  let conn = connection::connect_sqlite().unwrap();
  let query = conn.execute("DELETE FROM publishers WHERE id = ?", [&id]);
  
  connection::disconnect_sqlite(conn).unwrap();

  match query {
    Ok(value) =>
      Ok(value),
    Err(erro) =>
      Err(erro.into()),
  }
}

fn update_publisher_amount(id:&str, amount:u32) -> Result<usize, Box<dyn Error>> {
  let conn = connection::connect_sqlite().unwrap();
  let now = get_timestamp();
  let query = conn.execute(
    "UPDATE publishers SET  amount = ?2, updated_at = ?3 WHERE id = ?1",
    (id, amount, now),
  );
  
  match query {
    Ok(value) =>
      Ok(value),
    Err(erro) =>
      Err(erro.into()),
  }
}

//Presentations
pub fn create_presentations(mut length:u8, gender:String) -> Result<String, Box<dyn Error>> {
  let mut presentations : Vec<(Publisher, Publisher)> = vec![];
  
  while 0 < length {

    let main = match list_raffled_publisher(0, ">=".to_owned(), gender.clone()) {
      Some(publisher) => publisher.clone(),
      None => break,
    };

    let helper =  match list_raffled_publisher(main.id.unwrap(), "<=".to_owned(), main.gender.clone().to_owned()) {
        Some(publisher) => publisher.clone(),
        None => break,
    };

    presentations.push(
      (main.clone(), helper.clone())
    );
    
    thread::spawn( move || {
      let new_main_amount = sum_puplisher_amount(main.clone());
      let _ = update_publisher_amount(&main.id.unwrap().to_string(), new_main_amount);
    });
    
    thread::spawn( move || {
      let new_helper_amount = sum_puplisher_amount(helper.clone());
      let _ = update_publisher_amount(&helper.id.unwrap().to_string(), new_helper_amount);
    });

    length -= 1;

  }

  match serde_json::to_string(&presentations) {
    Ok(json) =>  Ok(json),
    Err(erro) =>  Err(erro.into()),
  }

}
