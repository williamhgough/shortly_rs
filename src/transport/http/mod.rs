use crate::hashing::{HashIDHasher, Hasher};
use crate::storage::models::{NewRedirect, Redirect as RD};
use crate::storage::sqlite::{
    create_redirect, delete_redirect, establish_connection, get_redirect,
};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

const DB: &str = "shortly.sqlite3";

#[derive(Deserialize)]
pub struct Payload {
    original_url: String,
}

#[get("/<id>")]
pub fn redirect(id: String) -> Redirect {
    let conn = establish_connection(DB);
    let rd = get_redirect(&conn, id).unwrap();
    Redirect::to(rd.original_url)
}

#[post("/api/v1/shorten", data = "<payload>")]
pub fn shorten(payload: Json<Payload>) -> Json<RD> {
    let conn = establish_connection(DB);
    let hash = HashIDHasher::generate(&payload.original_url, &time::now().tm_nsec.to_string());
    let rd = create_redirect(
        &conn,
        NewRedirect {
            id: &hash,
            original_url: &payload.original_url,
            new_url: &format!("http://short.ly/{}", hash),
        },
    )
    .unwrap();

    Json(rd)
}

#[post("/api/v1/delete?<id>")]
pub fn delete(id: String) -> Status {
    let conn = establish_connection(DB);
    let u = delete_redirect(&conn, id);
    if u < 1 {
        return Status::BadRequest;
    }
    Status::Ok
}
