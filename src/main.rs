pub mod db;

use rocket::form::Form;
use rocket::http::{Header, Status};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, routes, FromForm, Request, Rocket, State};
use rocket::response::{Responder, Response};
use db::DBManager;

enum UrlResponseState {
    Found,
    NotFound
}

struct UrlResponse {
    state: UrlResponseState,
    data: Option<String>
}

#[derive(Serialize, Deserialize, FromForm)]
#[serde(crate="rocket::serde")]
struct UrlData {
    short_url: String,
    target_url: String
}

#[get("/")]
async fn hello_world() -> String {
    "Hello world".to_string()
}

impl<'r> Responder<'r, 'r> for UrlResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'r> {
        match self.state {
            UrlResponseState::Found => {
                Ok(Response::build()
                    .status(Status::new(301))
                    .header(Header::new("Location", self.data.unwrap())) // Convert target_url to a String
                    .finalize()
                )
            },
            UrlResponseState::NotFound => {
                Err(Status::NotFound)
            }
        }
    }
}

#[get("/<short_url>")]
async fn get_target_url(short_url: &str, db: &State<DBManager>) -> UrlResponse {
    let result = db.get_entry(short_url).await;
    
    if let Err(_) = result {
        return UrlResponse {
            state: UrlResponseState::NotFound,
            data: None,
        }
    } else {
        return UrlResponse {
            state: UrlResponseState::Found,
            data: result.ok()
        }
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
        .mount("/", routes![hello_world, get_target_url, register_urls])
}