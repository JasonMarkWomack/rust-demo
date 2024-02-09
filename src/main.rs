#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::url::Orgin;
use rocket::serde::json::(json, Value);
#[get("/<name>/<age>")]
fn index() -> Redirect {
    Redirect::to(uri!(call_api()))
}

//https://west-api.vapi.ai/assistant?limit=5
#[get("https://west-api.vapi.ai/assistant")]
fn call_api(_platfrom: &str){
    Status::NoContent
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/hello", routes![hello])
    .mount("https://west-api.vapi.ai/assistant?limit=5", routes![call_api])
}
