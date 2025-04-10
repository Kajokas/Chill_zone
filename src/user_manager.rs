use std::i64;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket::fairing::AdHoc;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};
use serde::Serialize;
use sqlx::Error;
use validator::Validate;

#[derive(Database)]
#[database("database")]
struct Db(sqlx::SqlitePool);

#[derive(Deserialize, Debug, Serialize, Validate)]
struct User {
    id: Option<i64>,
    #[validate(length(max = 30))]
    username: String,
    #[validate(email)]
    #[validate(length(max = 30))]
    email: String,
    #[validate(length(max = 30))]
    psw: String,
}

#[derive(Deserialize, Validate)]
struct LogIn {
    #[validate(length(max = 30))]
    login: String,
    #[validate(length(max = 30))]
    psw: String,
}

#[derive(Responder)]
enum SignUpResponse {
    Response(Json<User>),
    ErrorMessage(String),
}

#[get("/logout")]
async fn log_out(cookies: &CookieJar<'_>) -> Status {
    cookies.remove_private("usr");
    Status::Ok
}

#[post("/login", format = "json", data = "<login>")]
async fn log_in(
    login: Json<LogIn>,
    mut db: Connection<Db>,
    cookies: &CookieJar<'_>,
) -> (Status, String) {
    if login.validate().is_err() {
        return (Status::BadRequest, "Invalid data".to_string());
    }

    let results = sqlx::query!(
        r#"SELECT * FROM USER WHERE (username = $1 OR email = $1)"#,
        login.login
    )
    .fetch_one(&mut **db)
    .await;

    match results {
        Err(Error::RowNotFound) => (Status::Unauthorized, String::from("No such user")),
        Err(e) => {
            eprintln!("Error: {}", e);
            (
                Status::InternalServerError,
                String::from("Something went wrong"),
            )
        }
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.psw).unwrap();

            if Argon2::default()
                .verify_password(login.psw.as_bytes(), &parsed_hash)
                .is_err()
            {
                return (Status::Unauthorized, "Wrong password".to_string());
            }

            let usr_id = user.id.to_string();

            println!("User {} has logged in", usr_id);

            let cookie = Cookie::build(("usr", usr_id.clone()))
                .path("/")
                .http_only(true)
                .secure(true);

            cookies.add_private(cookie);

            (Status::Ok, usr_id)
        }
    }
}

#[post("/signup", format = "json", data = "<user>")]
async fn sign_up(
    mut user: Json<User>,
    mut db: Connection<Db>,
    cookies: &CookieJar<'_>,
) -> (Status, SignUpResponse) {
    if user.validate().is_err() {
        return (
            Status::BadRequest,
            SignUpResponse::ErrorMessage("Invalid data!".to_string()),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_psw = argon2
        .hash_password(user.psw.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let results = sqlx::query!(
        "INSERT INTO user (username, email, psw) VALUES (?, ?, ?) RETURNING id",
        user.username,
        user.email,
        hashed_psw
    )
    .fetch_one(&mut **db)
    .await;

    match results {
        Err(Error::Database(e)) => {
            if e.is_unique_violation() {
                (
                    Status::BadRequest,
                    SignUpResponse::ErrorMessage(String::from("User already exist!")),
                )
            } else {
                eprintln!("Error: {}", e);
                (
                    Status::InternalServerError,
                    SignUpResponse::ErrorMessage(String::from("Something went wrong")),
                )
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            (
                Status::InternalServerError,
                SignUpResponse::ErrorMessage(String::from("Something went wrong")),
            )
        }
        Ok(u) => {
            println!("User created");
            user.id = Some(u.id);
            let usr_id = user.id.map(|v| v.to_string()).unwrap_or("".to_string());

            let cookie = Cookie::build(("usr", usr_id))
                .path("/")
                .http_only(true)
                .secure(true);

            cookies.add_private(cookie);

            (Status::Created, SignUpResponse::Response(user))
        }
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database staged", |rocket| async {
        rocket
            .attach(Db::init())
            .mount("/", routes![sign_up, log_in, log_out])
    })
}
