use std::{io, time::Duration};

use rocket::{
    http::Status,
    tokio::{task::spawn_blocking, time::sleep},
    Request,
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is all it can do for now."
}

#[get("/num")]
fn num() -> &'static str {
    "5"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

// Rust's Futures are a form of cooperative multitasking.
#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    // This function basically reads this file which is in the root directory and then downloads it.

    Ok(vec)
}

/*
Any number of dynamic path segments are allowed.
A path segment can be of any type, including your own, as long as the type
implements the FromParam trait. We call these types parameter guards.
https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromParam.html
*/
#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

// Catches all route errors
#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

/**
// To catch specific status codes, refer function below:
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, {} does not exist.", req.uri())
}
*/

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, num, delay, blocking_task, hello])
        .register("/", catchers![default])
}
