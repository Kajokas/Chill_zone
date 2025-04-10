use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

pub mod authenticator;
pub mod music_manager;
pub mod upload_song_api;
pub mod user_manager;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./static/front_end/main").rank(0))
        .mount("/loginPage", FileServer::from("./static/front_end/log_in"))
        .mount(
            "/signupPage",
            FileServer::from("./static/front_end/sign_up"),
        )
        .mount("/uploadPage", FileServer::from("./static/front_end/upload"))
        .mount("/songsDir", FileServer::from("./music"))
        .mount("/listen", FileServer::from("./static/front_end/listen"))
        .mount("/img", FileServer::from("./static/Images"))
        .attach(user_manager::stage())
        .attach(music_manager::stage())
        .attach(authenticator::stage())
        .attach(upload_song_api::stage())
}
