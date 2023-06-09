#[macro_use]
extern crate rocket;

use std::error::Error;
use std::str::FromStr;

use crate::core::{hooks_core, hooks_verify};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins};

mod core;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods: AllowedMethods = ["Get", "Post"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .attach(cors)
        .mount("/", routes![hooks_verify, hooks_core])
        .launch()
        .await?;
    Ok(())
}
