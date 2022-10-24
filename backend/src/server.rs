



#[get("/")]
pub fn index() -> &'static str 
{
    "Hello, world!"
}


#[get("/user/<name>")]
pub fn username(name : &str) -> String
{
    format!("Hello, {}",name)
    
}




