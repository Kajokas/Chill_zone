use std::i64;

use rocket::fairing::AdHoc;
use rocket::futures::TryStreamExt;
use rocket::http::CookieJar;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};
use serde::Serialize;

#[derive(Database)]
#[database("database")]
struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Deserialize, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: Option<i64>,
    username: String,
    email: String,
    psw: String,
}

#[derive(Deserialize)]
struct LogIn {
    login: String,
    psw: String,
}

#[get("/logout")]
async fn log_out(cookies: &CookieJar<'_>) -> &'static str {
    cookies.remove("usr");
    "its done"
}

#[post("/login", format = "json", data = "<login>")]
async fn log_in(
    login: Json<LogIn>,
    mut db: Connection<Db>,
    cookies: &CookieJar<'_>,
) -> Result<Json<User>> {
    let results = sqlx::query!(
        r#"SELECT id FROM USER WHERE (username = $1 OR email = $1) AND psw = $2"#,
        login.login,
        login.psw
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    let temp_id = Some(results.first().expect("No such user").id);

    if temp_id.is_some() {
        let id = temp_id.map(|i| i.to_string()).unwrap();

        println!("{}", id);

        if cookies.get("usr").is_none() {
            cookies.add(("usr", id));
        } else {
            cookies.remove("usr");
            cookies.add(("usr", id));
        }
    }

    Ok(Json(User {
        id: temp_id,
        username: String::from("NO"),
        email: String::from("NO"),
        psw: String::from("NO"),
    }))
}

#[post("/signup", format = "json", data = "<user>")]
async fn sign_up(
    mut user: Json<User>,
    mut db: Connection<Db>,
    cookies: &CookieJar<'_>,
) -> Result<Created<Json<User>>> {
    //println!("{0}, {1}, {2}", user.username, user.email, user.psw);

    let results = sqlx::query!(
        "INSERT INTO user (username, email, psw) VALUES (?, ?, ?) RETURNING id",
        user.username,
        user.email,
        user.psw
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    user.id = Some(results.first().expect("Returning results").id);
    let usr_id = user.id.map(|v| v.to_string()).unwrap_or("".to_string());

    if cookies.get("usr").is_none() {
        cookies.add(("usr", usr_id));
    } else {
        cookies.remove("usr");
        cookies.add(("usr", usr_id));
    }

    Ok(Created::new("/").body(user))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database staged", |rocket| async {
        rocket
            .attach(Db::init())
            .mount("/", routes![sign_up, log_in, log_out])
    })
}
