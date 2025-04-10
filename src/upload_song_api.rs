use std::i64;
use std::path::Path;

use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::{relative, TempFile};
use rocket::http::{ContentType, CookieJar, Status};
use rocket::post;
use rocket::tokio;
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};
use sqlx::Error;
use uuid::Uuid;

use crate::rocket;

#[derive(Database)]
#[database("database")]
struct Db(sqlx::SqlitePool);

#[derive(Debug, FromForm)]
struct Music<'r> {
    #[field(validate = len(..101))]
    title: String,
    #[field(validate = ext(ContentType::JPEG))]
    cover_file: TempFile<'r>,
    #[field(validate = ext(ContentType::MP3))]
    music_file: TempFile<'r>,
}

#[post("/upload", data = "<song>")]
async fn upload_song<'r>(song: Form<Music<'r>>, mut db: Connection<Db>, cookies: &CookieJar<'_>) -> (Status, String) {
    let usr_id = cookies
        .get_private("usr")
        .map(|crumb| format!("{}", crumb.value()));

    if usr_id.is_none() {
        return (Status::Unauthorized, "Can't upload songs if user is not logged in".to_string());
    }

    let (cover_file_name, audio_file_name) = save_files(&song.cover_file, &song.music_file).await;

    let results = sqlx::query!(
        "INSERT INTO songs (Title, CoverArtFile, MusicFile, Artist) VALUES (?, ?, ?, ?) RETURNING id",
        song.title,
        cover_file_name,
        audio_file_name,
        usr_id
    )
    .fetch_one(&mut **db)
    .await;

    //In case of Error it should clean up created files, but im not sure if creating files before SQL insert is a good idea, so until I decide on how it should be done files get created even if no song is created
    match results {
        Err(Error::Database(e)) => {
            if e.is_foreign_key_violation() {
                (Status::Unauthorized, "User is unauthorized to upload".to_string())
            } else {
                (Status::InternalServerError, "Something went wrong".to_string())
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            (Status::InternalServerError, "Something went wrong".to_string())
        }
        Ok(r) => {
            let song_id = r.id;
            (Status::Created, song_id.to_string())
        }
    }
}

async fn save_files<'a>(cover_file: &TempFile<'a>, audio_file: &TempFile<'a>) -> (String, String) {
    let music_dir = Path::new(relative!("music"));

    let song_uuid = Uuid::new_v4();

    let cover_file_name = format!("{}_img", song_uuid);
    let audio_file_name = format!("{}_audio", song_uuid);

    let cover_file_path = music_dir.join(format!("{}", cover_file_name));
    let music_file_path = music_dir.join(format!("{}", audio_file_name));

    let mut cover_file_save = tokio::fs::File::create(cover_file_path).await.unwrap();
    let mut cover_file_stream = cover_file.open().await.unwrap();
    tokio::io::copy(&mut cover_file_stream, &mut cover_file_save).await.expect("Failed to copy cover data");

    let mut music_file_save = tokio::fs::File::create(music_file_path).await.unwrap();
    let mut music_file_stream = audio_file.open().await.unwrap();
    tokio::io::copy(&mut music_file_stream, &mut music_file_save).await.expect("Failed to copy music data");

    (cover_file_name, audio_file_name)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database staged", |rocket| async {
        rocket.attach(Db::init()).mount("/", routes![upload_song])
    })
}
