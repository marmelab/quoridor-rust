use crate::error::AppError;
use crate::game::{Board, Game};
use actix_web::{HttpResponse, Responder};

pub async fn get_board() -> Result<impl Responder, AppError> {
    let result = Board::new(3);
    result.map(|board| HttpResponse::Ok().json(board))
}

pub async fn new_game() -> impl Responder {
    let result = Game::new(5);
    result.map(|game| HttpResponse::Ok().json(game))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

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
