//add postgres
use postgres::{Connection, TlsMode};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;
use rocket::{get, routes};
use std::collections::HashMap;
use std::sync::Mutex;



struct user {
    id: i32,
    name: String,
    email: String,
    password: String,
}


impl user {
    fn new(id: i32, name: String, email: String, password: String) -> user {
        user {
            id,
            name,
            email,
            password,
        }
    }

    fn add_to_db(&self, conn: &Connection) {
        conn.execute(
            "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)",
            &[&self.id, &self.name, &self.email, &self.password],
        )
        .unwrap();
    }
}
