use std::i64;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};
use serde::Serialize;
use sqlx::Error;

use crate::rocket;

#[derive(Database)]
#[database("database")]
struct Db(sqlx::SqlitePool);

static LOAD_SONG_LIMIT: i64 = 16;

#[derive(Serialize)]
struct RetMusicData {
    id: i64,
    title: String,
    artist: String,
    thumbnail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    song: Option<String>,
}

#[derive(Responder)]
enum SongResponse {
    ReturnDataArray(Json<Vec<RetMusicData>>),
    ReturnData(Json<RetMusicData>),
    ErrorMessage(String),
}

#[get("/song?<l>")]
async fn load_song(mut db: Connection<Db>, l: i64) -> (Status, SongResponse) {
    let results = sqlx::query!("SELECT s.id as id, s.title as title, u.username as artist, s.CoverArtFile as thumbnail, s.MusicFile as music FROM user u, songs s where s.artist = u.id and s.id = ?", l)
    .fetch_one(&mut **db)
    .await;

    let good_result;

    match results {
        Err(Error::RowNotFound) => {
            eprintln!("No such song");
            return (
                Status::NotFound,
                SongResponse::ErrorMessage("Song not found".to_string()),
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return (
                Status::InternalServerError,
                SongResponse::ErrorMessage("Something went wrong".to_string()),
            );
        }
        Ok(song) => {
            good_result = song;
        }
    }

    let data = RetMusicData {
        id: good_result.id,
        title: good_result.title,
        artist: good_result.artist,
        thumbnail: good_result.thumbnail,
        song: Some(good_result.music),
    };

    (Status::Ok, SongResponse::ReturnData(Json(data)))
}

#[get("/loadMainPageSongs?<f>")]
async fn load_main_songs(mut db: Connection<Db>, mut f: i64) -> (Status, SongResponse) {
    f = f * LOAD_SONG_LIMIT;
    let results = sqlx::query!("SELECT s.id as id, s.title as title, u.username as artist, s.CoverArtFile as thumbnail FROM user u, songs s where s.artist = u.id AND s.id > ? LIMIT?", f, LOAD_SONG_LIMIT)
    .fetch_all(&mut **db)
    .await;

    let good_results;

    match results {
        Err(e) => {
            eprintln!("Error: {}", e);
            return (
                Status::InternalServerError,
                SongResponse::ErrorMessage("Something went wrong".to_string()),
            );
        }
        Ok(songs) => {
            good_results = songs;
        }
    }

    let mut song_vec = Vec::new();

    for r in good_results {
        let id = r.id;
        let title = r.title;
        let artist = r.artist;
        let thumb = r.thumbnail;

        let song_data = RetMusicData {
            id,
            title,
            artist,
            thumbnail: thumb,
            song: None,
        };

        song_vec.push(song_data);
    }

    (Status::Ok, SongResponse::ReturnDataArray(Json(song_vec)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database staged", |rocket| async {
        rocket
            .attach(Db::init())
            .mount("/", routes![load_main_songs, load_song])
    })
}
