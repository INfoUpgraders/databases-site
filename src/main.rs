#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the site!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
