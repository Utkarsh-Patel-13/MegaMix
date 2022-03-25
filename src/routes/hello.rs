use std::fmt::Debug;

use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct HelloResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct HelloName {
    name: String,
    age: u32,
}

#[get("/")]
pub fn hello_world() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, world!".to_string(),
    })
}

#[get("/<name>")]
pub fn hello_name(name: String) -> Json<HelloResponse> {
    Json(HelloResponse {
        message: format!("Hello, {}!", name),
    })
}

#[get("/?<name>")]
pub fn hello_name_query(name: String) -> Json<HelloResponse> {
    Json(HelloResponse {
        message: format!("Hello, {}!", name),
    })
}

#[post("/", format = "json", data = "<user>")]
pub fn hello_name_json_post(user: Json<HelloName>) -> status::Created<Json<HelloResponse>> {
    let res = Json(HelloResponse {
        message: format!("{} stored!!", user.name),
    });
    status::Created::new("local").body(res)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Hello route ignited", |rocket| async {
        rocket.mount(
            "/hello",
            routes![
                hello_world,
                hello_name,
                hello_name_query,
                hello_name_json_post
            ],
        )
    })
}
