use rocket::{
    fairing::AdHoc,
    http::{CookieJar, Status},
};

extern crate rocket;

#[get("/getUser")]
fn get_usr(cookies: &CookieJar<'_>) -> (Status, String) {
    let id = cookies
        .get_private("usr")
        .map(|crumb| format!("{}", crumb.value()));

    match id {
        Some(id) => (Status::Ok, id),
        None => (Status::Unauthorized, String::from("User is unauthorized")),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Authenticator online", |rocket| async {
        rocket.mount("/", routes![get_usr])
    })
}
