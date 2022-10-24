//add postgres
use postgres::{Connection, TlsMode};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;
use rocket::{get, routes};
use std::collections::HashMap;
use std::sync::Mutex;



struct user 
{
    id: i32,
    name: String,
    email: String,
    password: String,
}


impl user 
{
    fn new(id: i32, name: String, email: String, password: String) -> user {
        user 
        {
            id,
            name,
            email,
            password,
        }
    }

    fn add_to_db(&self, conn: &Connection)
    {
        conn.execute(
            "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)",
            &[&self.id, &self.name, &self.email, &self.password],
        )
        .unwrap();
    }
}



struct Dbase 
{
    conn: Connection,
}


impl Dbase 
{
    fn new() -> Dbase 
    {
        let conn = Connection::connect(
            "postgresql://postgres:postgres@localhost:5432/postgres",
            TlsMode::None,
        )
        .unwrap();
        Dbase { conn }
    }

    fn get_user(&self, id: i32) -> user 
    {
        let mut stmt = self.conn.prepare("SELECT * FROM users WHERE id = $1").unwrap();
        let user = stmt
            .query(&[&id])
            .unwrap()
            .iter()
            .map(|row| user::new(row.get(0), row.get(1), row.get(2), row.get(3)))
            .collect::<Vec<user>>();
        user[0].clone()
    }
}