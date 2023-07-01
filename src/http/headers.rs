use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, Header};

pub struct Headers {
    max_age: chrono::Duration
}

impl Headers {
    pub fn new( duration: chrono::Duration ) -> Headers {
        Headers { max_age: duration }
    }
}

#[rocket::async_trait]
impl Fairing for Headers {
    fn info(&self) -> Info {
        Info {
            name: "headers",
            kind: Kind::Response
        }
    }

    
    async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {

        // only cors makes OPTION calls therefore I can do this. I am a genious. My brain is large. I hav big iq
        if req.method() != Method::Get {
            return
        }

        response.set_header(Header::new("Cache-Control", format!("max-age={}", self.max_age.num_seconds())));
    }
}