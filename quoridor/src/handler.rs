use actix_web::{web, Responder};
use crate::game::{Board};

pub async fn get_board() -> impl Responder {
    web::Json(Board::new(3))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App, Error};
    use actix_web::dev::Service;

    #[actix_rt::test]
    async fn test_get_board() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(get_board));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, "{\"size\":3,\"squares\":[{\"column\":0,\"row\":0},{\"column\":1,\"row\":0},{\"column\":2,\"row\":0},{\"column\":0,\"row\":1},{\"column\":1,\"row\":1},{\"column\":2,\"row\":1},{\"column\":0,\"row\":2},{\"column\":1,\"row\":2},{\"column\":2,\"row\":2}]}");

        Ok(())
    }

}