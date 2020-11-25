use actix_web::{HttpRequest};

pub async fn hello(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App, Error};
    use actix_web::dev::Service;

    #[actix_rt::test]
    async fn test_hello_handler() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(hello));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello world!"##);

        Ok(())
    }

}
