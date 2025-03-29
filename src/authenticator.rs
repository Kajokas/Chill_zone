use rocket::{fairing::AdHoc, http::CookieJar};

extern crate rocket;

#[get("/getUser")]
fn get_usr(cookies: &CookieJar<'_>) -> Option<String> {
    let id = cookies
        .get_private("usr")
        .map(|crumb| format!("{}", crumb.value()));

    println!("{:?}", id);

    id
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Authenticator online", |rocket| async {
        rocket.mount("/", routes![get_usr])
    })
}
