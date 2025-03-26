use std::i64;
use std::path::Path;

use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::{relative, TempFile};
use rocket::futures::TryStreamExt;
use rocket::http::CookieJar;
use rocket::outcome::IntoOutcome;
use rocket::tokio;
use rocket::{post, tokio::fs::File, tokio::io::AsyncWriteExt};
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};

use crate::rocket;

#[derive(Database)]
#[database("database")]
struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, FromForm)]
struct Music<'r> {
    id: Option<i64>,
    Title: String,
    CoverArtFile: String,
    MusicFile: String,
    coverFile: TempFile<'r>,
    musicFile: TempFile<'r>,
}

#[post("/upload", data = "<song>")]
async fn upload_song<'r>(
    mut song: Form<Music<'r>>,
    mut db: Connection<Db>,
    cookies: &CookieJar<'_>,
) -> Result<String> {
    let usr_id = cookies.get("usr").map(|crumb| crumb.value());

    let results = sqlx::query!(
        "INSERT INTO songs (Title, CoverArtFile, MusicFile, Artist) VALUES (?, ?, ?, ?) RETURNING id",
        song.Title,
        song.CoverArtFile,
        song.MusicFile,
        usr_id
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    song.id = Some(results.first().expect("Returning results").id);
    let song_id = song.id.map(|v| v.to_string()).unwrap_or("".to_string());

    let music_dir = Path::new(relative!("music"));

    let cover_file_path = music_dir.join(format!("{}", song.CoverArtFile));
    let music_file_path = music_dir.join(format!("{}", song.MusicFile));

    tokio::fs::create_dir_all(music_dir).await.unwrap();

    let mut cover_file = tokio::fs::File::create(cover_file_path).await.unwrap();
    let mut cover_file_stream = song.coverFile.open().await.unwrap();
    tokio::io::copy(&mut cover_file_stream, &mut cover_file)
        .await
        .unwrap();

    let mut music_file = tokio::fs::File::create(music_file_path).await.unwrap();
    let mut music_file_stream = song.musicFile.open().await.unwrap();
    tokio::io::copy(&mut music_file_stream, &mut music_file)
        .await
        .unwrap();

    Ok(song_id)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database staged", |rocket| async {
        rocket.attach(Db::init()).mount("/", routes![upload_song])
    })
}
