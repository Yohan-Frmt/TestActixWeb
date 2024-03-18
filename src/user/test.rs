#[cfg(test)]
mod user_test {
    use crate::{database::mongodb::Mongo, user, user::model::CreateUser};
    use actix_http::Request;
    use actix_web::{
        dev::{Service, ServiceResponse},
        http::header::ContentType,
        test::{self, TestRequest},
        web::{self, Data, Json},
        App, Error,
    };
    use std::sync::Arc;
    use user::controller::init_routes;

    pub async fn init() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        let mongodb = Data::from(Arc::new(Mongo::init(Some("user_test")).await));
        test::init_service(
            App::new()
                .app_data(mongodb.clone())
                .service(web::scope("/api/user").configure(init_routes)),
        )
        .await
    }

    #[actix_web::test]
    async fn should_read_users() {
        let app = init().await;
        let res = TestRequest::get().uri("/api/user").send_request(&app).await;
        assert!(res.status().is_success(), "Something went wrong");
    }

    #[actix_web::test]
    async fn should_create_user() {
        let req_body = Json(CreateUser {
            _id: None,
            email: String::from("Rybard"),
            username: String::from("fremontyohan@gmail.com"),
            password: String::from("tagada"),
        });
        let app = init().await;
        let res = TestRequest::post()
            .uri("/api/user")
            .insert_header(ContentType::json())
            .set_json(req_body)
            .send_request(&app)
            .await;
        println!("{:?}", res.response().body());
        assert!(res.status().is_success());
    }
}
