
use actix_web::{middleware, web, App, HttpServer};

mod error;
mod handler;
mod game;

static PORT: i32 = 8383;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("Listening on {:?}", PORT);
    let api_port = format!("api:{}", PORT);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/games/{gameId}").to(handler::new_game))
            .service(web::resource("/games/{gameId}/add-fence").route(web::post().to(handler::add_fence)))
    })
    .bind(api_port)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App, body::{Body, ResponseBody}};

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for ResponseBody<Body> {
        fn as_str(&self) -> &str {
            match self {
                ResponseBody::Body(ref b) => match b {
                    Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                    _ => panic!(),
                },
                ResponseBody::Other(ref b) => match b {
                    Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                    _ => panic!(),
                },
            }
        }
    }

    #[actix_rt::test]
    async fn test_new_game() {
        let mut app = test::init_service(App::new().route("/", web::get().to(handler::new_game))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.response().body().as_str(), "{\"id\":\"anUID\",\"over\":false,\"pawn_turn\":0,\"pawns\":[{\"position\":{\"column\":0,\"row\":2},\"goal\":\"EAST\"},{\"position\":{\"column\":4,\"row\":2},\"goal\":\"WEST\"}],\"fences\":[],\"board\":{\"size\":5,\"squares\":[{\"column\":0,\"row\":0},{\"column\":1,\"row\":0},{\"column\":2,\"row\":0},{\"column\":3,\"row\":0},{\"column\":4,\"row\":0},{\"column\":0,\"row\":1},{\"column\":1,\"row\":1},{\"column\":2,\"row\":1},{\"column\":3,\"row\":1},{\"column\":4,\"row\":1},{\"column\":0,\"row\":2},{\"column\":1,\"row\":2},{\"column\":2,\"row\":2},{\"column\":3,\"row\":2},{\"column\":4,\"row\":2},{\"column\":0,\"row\":3},{\"column\":1,\"row\":3},{\"column\":2,\"row\":3},{\"column\":3,\"row\":3},{\"column\":4,\"row\":3},{\"column\":0,\"row\":4},{\"column\":1,\"row\":4},{\"column\":2,\"row\":4},{\"column\":3,\"row\":4},{\"column\":4,\"row\":4}]}}");
    }

}
