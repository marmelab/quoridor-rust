use crate::game::{Game};
use actix_web::{HttpResponse, Responder};

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
    async fn test_new_game() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(new_game));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, "{\"id\":\"anUID\",\"over\":false,\"pawn_turn\":0,\"pawns\":[{\"position\":{\"column\":0,\"row\":2},\"goal\":\"EAST\"},{\"position\":{\"column\":4,\"row\":2},\"goal\":\"WEST\"}],\"fences\":[],\"board\":{\"size\":5,\"squares\":[{\"column\":0,\"row\":0},{\"column\":1,\"row\":0},{\"column\":2,\"row\":0},{\"column\":3,\"row\":0},{\"column\":4,\"row\":0},{\"column\":0,\"row\":1},{\"column\":1,\"row\":1},{\"column\":2,\"row\":1},{\"column\":3,\"row\":1},{\"column\":4,\"row\":1},{\"column\":0,\"row\":2},{\"column\":1,\"row\":2},{\"column\":2,\"row\":2},{\"column\":3,\"row\":2},{\"column\":4,\"row\":2},{\"column\":0,\"row\":3},{\"column\":1,\"row\":3},{\"column\":2,\"row\":3},{\"column\":3,\"row\":3},{\"column\":4,\"row\":3},{\"column\":0,\"row\":4},{\"column\":1,\"row\":4},{\"column\":2,\"row\":4},{\"column\":3,\"row\":4},{\"column\":4,\"row\":4}]}}");

        Ok(())
    }
}
