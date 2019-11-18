use super::schema::redirects;

#[derive(Insertable)]
#[table_name = "redirects"]
pub struct NewRedirect<'a> {
    pub id: &'a str,
    pub original_url: &'a str,
    pub new_url: &'a str,
}

#[derive(Queryable, Serialize, Debug, PartialEq)]
pub struct Redirect {
    pub id: String,
    pub original_url: String,
    pub new_url: String,
}
