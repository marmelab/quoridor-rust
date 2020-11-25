
use actix_web::{middleware, web, App, HttpServer};
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(handler::hello))
    })
    .bind("api:8383")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use actix_web::body::{Body, ResponseBody};

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
    async fn test_get_hello_world() {
        let mut app = test::init_service(App::new().route("/", web::get().to(handler::hello))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        
        let resp = test::call_service(&mut app, req).await;
        
        assert!(resp.status().is_success());
        assert_eq!(resp.response().body().as_str(), "Hello world!");
    }

}
