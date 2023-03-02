#[macro_use] extern crate rocket;

use tokio::task;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    task::spawn(async {
        println!("spawn")
    });

    "event"
}