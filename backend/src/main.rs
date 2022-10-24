#[macro_use] extern crate rocket;
mod server;



#[launch]
fn rocket() -> _ 
{
    rocket::build().mount("/", routes![server::index,server::username])
}