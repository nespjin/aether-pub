use log::error;
use aether_pub_server;
use aether_pub_server::rocket;

#[rocket::main]
async fn main() {
    aether_pub_server::config::check();
    if let Err(e) = rocket().launch().await {
        error!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}
