use std::error::Error;

#[macro_use] extern crate rocket;
use rocket::http::{Status};

#[get("/assistant")]
async fn get_assistant() -> Result<String, Status> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://west-api.vapi.ai/assistant")
        .header("accept", "application/json")
        .header("Authorization", "Bearer 2a911e1c-8e20-40c9-b953-e8b23aa5b030")
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|_| Status::InternalServerError)?;
        Ok(body)
    } else {
        Err(Status::from_code(response.status().as_u16()).unwrap_or(Status::InternalServerError))
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build().mount("/", routes![get_assistant]).launch().await?;
    Ok(())
}
