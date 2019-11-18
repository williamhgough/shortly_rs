use crate::storage::models::{NewRedirect, Redirect};
use diesel::result::Error;
use diesel::{prelude::*, SqliteConnection};

const DB: &str = "shortly.sqlite3";

pub fn establish_connection(db: &str) -> SqliteConnection {
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_redirect(connection: &SqliteConnection, redirect: NewRedirect) -> Option<Redirect> {
    let res = diesel::insert_into(super::schema::redirects::table)
        .values(&redirect)
        .execute(connection)
        .unwrap();
    if res < 1 {
        return None;
    }

    Some(Redirect {
        id: redirect.id.to_owned(),
        original_url: redirect.original_url.to_owned(),
        new_url: redirect.new_url.to_owned(),
    })
}

pub fn get_redirect(connection: &SqliteConnection, pk: String) -> Result<Redirect, Error> {
    let _connection = establish_connection(DB);
    super::schema::redirects::dsl::redirects
        .find(pk)
        .first(connection)
}

pub fn delete_redirect(connection: &SqliteConnection, pk: String) -> usize {
    diesel::delete(super::schema::redirects::dsl::redirects.find(pk))
        .execute(connection)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::storage::models::{NewRedirect, Redirect};
    use crate::storage::sqlite::{
        create_redirect, delete_redirect, establish_connection, get_redirect,
    };

    #[test]
    fn get_works() {
        let conn = establish_connection("test.sqlite3");
        let r = get_redirect(&conn, "abcdef".to_string()).unwrap();
        assert_eq!("abcdef", r.id);
        assert_eq!("https://google.com", r.original_url);
        assert_eq!("https://short.ly/abcdef", r.new_url);
    }

    #[test]
    fn create_works() {
        let conn = establish_connection("test.sqlite3");
        let r = create_redirect(
            &conn,
            NewRedirect {
                id: "123456",
                original_url: "https://yahoo.com",
                new_url: "https://short.ly/123456",
            },
        )
        .unwrap();
        assert_eq!("https://short.ly/123456", r.new_url);
        delete_redirect(&conn, "123456".to_string());
    }
}
