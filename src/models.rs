use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub real_name: Option<String>,
    pub bio: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
}