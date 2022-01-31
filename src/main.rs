#[macro_use]
extern crate rocket;

mod routes;

use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(hello::stage())
        .attach(file_server::stage())
}
