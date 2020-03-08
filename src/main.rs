#![feature(proc_macro_hygiene, decl_macro)] // language features needed by Rocket

// Import the rocket macros
#[macro_use]
extern crate rocket;

// Create route / that returns "Hello, world!"
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
