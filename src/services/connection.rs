use rusqlite::Connection;
use dotenv::dotenv;
use std::{ env, error::Error };

pub fn connect_sqlite() -> Result<Connection,  Box<dyn Error>> {
  dotenv().ok();
  let path  = env::var("DB_PATH").expect("DB_PATH not set");
  let conn = Connection::open(path)?;
  Ok(conn)
}

pub fn disconnect_sqlite(conn:Connection) -> Result<(), Box<dyn Error>> {
  let close = conn.close();
  Ok(close.unwrap())
}