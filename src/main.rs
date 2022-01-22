mod async_routes;

#[macro_use]
extern crate rocket;

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
        .mount(
            "/",
            routes![index, async_routes::delay, async_routes::blocking_task],
        )
        .mount("/hello", routes![hello])
        .launch()
        .await;
}
