#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[get("/")]
fn index()-> &'static str{
    "Hello, World!"
}

#[get("/user")]
fn user()-> Json<User> {
    Json(User {name:"Juciano"})
}


fn main(){
    rocket::ignite().mount("/", routes![index, user]).launch();
}