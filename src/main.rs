#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
fn login() -> &'static str {
    "Login screen"
}

#[get("/hi/<name>")]
fn hi(name: &str) -> String {
    format!("Hello how are you {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/auth", routes![login])
        .mount("/in", routes![hi])
}
