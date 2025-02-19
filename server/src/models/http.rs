use crate::schema::package::star;
use rocket::http::{Header, Status};
use rocket::response::Responder;
use rocket::yansi::Paint;

pub struct ServerHeader {
    pub content_type: String,
}

impl From<ServerHeader> for Header<'static> {
    fn from(value: ServerHeader) -> Self {
        Header::new("Content-Type", value.content_type)
    }
}

pub struct ServerJsonResponder {
    pub body: String,
}

impl ServerJsonResponder {
    pub fn new(body: &str) -> Self {
        Self {
            body: body.to_string(),
        }
    }
}
#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ServerJsonResponder {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::Response::build();
        response.header(Header::new("Content-Type", "application/vnd.pub.v2+json"));
        response.sized_body(self.body.len(), std::io::Cursor::new(self.body));
        response.ok()
    }
}

pub struct ServerNoContentResponder {
    pub location: String,
}

impl ServerNoContentResponder {
    pub fn new(location: &str) -> Self {
        Self {
            location: location.to_string(),
        }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ServerNoContentResponder {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut response = rocket::Response::build();
        response.status(rocket::http::Status::NoContent);
        response.header(Header::new("Location", self.location));
        response.status(Status::new(204)).ok()
    }
}
