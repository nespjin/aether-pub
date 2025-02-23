use log::info;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use std::time::Instant;

pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        info!("Incoming request: {} {}", request.method(), request.uri());
        info!("Headers: {:#?}", request.headers());
        info!("Request time: {}", Instant::now().elapsed().as_secs());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        info!(
            "Response: {} {} -> {}",
            request.method(),
            request.uri(),
            response.status()
        );
        info!("Response headers: {:#?}", response.headers());
        info!("Response time: {}", Instant::now().elapsed().as_secs());
        // if response.status().code == 200 {
        //     let body_string = &mut String::new();
        //     let _ = response.body().clone().read_to_string(body_string).await.unwrap();
        //     info!("Response body: {}", body_string);
        // }
    }
}
