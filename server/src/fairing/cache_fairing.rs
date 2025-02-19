use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub(crate) struct StaticCacheFairing;

#[rocket::async_trait]
impl Fairing for StaticCacheFairing {
    fn info(&self) -> Info {
        Info {
            name: "Static Cache Headers",
            kind: Kind::Response,
        }
    }
     async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>)  {
        if req.uri().path().starts_with("/static/") {
            res.set_raw_header("Cache-Control", "public, max-age=86400");
        }
    }
}
