mod async_routes;
mod segments;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

#[get("/")]
fn index() -> &'static str {
    "Hello, rocket!"
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/public", FileServer::from("static/"))
        .mount(
            "/",
            routes![index, async_routes::delay, async_routes::blocking_task,],
        )
        .mount("/hello", routes![hello])
        .mount("/files", routes![segments::files])
        .launch()
        .await;
}
