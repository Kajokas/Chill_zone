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
        rocket.attach(Db::init()).mount("/", routes![sign_up])
    })
}
