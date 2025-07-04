use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::Deserialize;
use argon2::{self, Config};
use rand::Rng;
use crate::schema::users;
use crate::models::{User, NewUser};

mod schema;
mod models;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    password: String,
}

fn hash_password(password: &str) -> String {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

async fn fetch_location_from_ip(ip: &str) -> Option<(f32, f32)> {
    let url = format!("http://ip-api.com/json/{}", ip);
    let response = reqwest::get(&url).await.ok()?;
    let json: serde_json::Value = response.json().await.ok()?;
    let lat = json["lat"].as_f64()? as f32;
    let lon = json["lon"].as_f64()? as f32;
    Some((lat, lon))
}

async fn register(
    pool: web::Data<DbPool>,
    form: web::Json<RegisterForm>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().map_err(|_| {
        actix_web::error::ErrorInternalServerError("Could not get db connection")
    })?;

    // Check if username exists
    let user_count = users::table
        .filter(users::username.eq(&form.username))
        .count()
        .get_result::<i64>(&conn)
        .unwrap_or(0);
    if user_count > 0 {
        return Ok(HttpResponse::BadRequest().body("Username already taken"));
    }

    // Get IP-based location
    let ip = req.connection_info().realip_remote_addr().unwrap_or("0.0.0.0");
    let location = fetch_location_from_ip(ip).await;
    let (latitude, longitude) = location.unwrap_or((0.0, 0.0));

    let new_user = NewUser {
        username: &form.username,
        password_hash: &hash_password(&form.password),
        latitude: Some(latitude),
        longitude: Some(longitude),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not insert user"))?;

    Ok(HttpResponse::Ok().body("User registered"))
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, onlyoomfs!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(hello))
            .route("/register", web::post().to(register))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}