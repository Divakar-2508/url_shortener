pub mod db;

use rocket::form::Form;
use rocket::http::Header;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, routes, FromForm, Rocket, State};
use rocket::response::Responder;
use db::DBManager;

#[derive(Responder)]
enum UrlResponse {
    #[response(status = 301)] 
    Found((), Header<'static>),
    #[response(status = 404)] 
    NotFound(()),
}

#[derive(Serialize, Deserialize, FromForm)]
#[serde(crate="rocket::serde")]
struct UrlData {
    short_url: String,
    target_url: String
}

#[get("/<short_url>")]
async fn get_target_url(short_url: &str, db: &State<DBManager>) -> UrlResponse {
    let result = db.get_entry(short_url).await;
    
    if let Err(_) = result {
        return UrlResponse::NotFound(())
    } else {
        return UrlResponse::Found((), Header::new("Location", result.unwrap()))
    }
}

#[post("/add", data="<url_data>")]
async fn register_urls(url_data: Form<UrlData>, db: &State<DBManager>) -> String {
    let data = url_data.into_inner();

    let result = db.add_entry(&data.short_url, &data.target_url).await;

    if let Err(err) = result {
        err
    } else {
        result.unwrap()
    }
}

#[launch]
#[tokio::main]
async fn launch() -> _ {
    let db = DBManager::new().await.unwrap();
  
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    Rocket::build()
        .manage(db)
        .attach(cors)
        .mount("/", routes![get_target_url, register_urls])
}