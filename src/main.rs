use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

pub mod user_manager;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./static/front_end/main").rank(0))
        .mount("/login", FileServer::from("./static/front_end/log_in"))
        .mount("/signup", FileServer::from("./static/front_end/sign_up"))
        .attach(user_manager::stage())
}
