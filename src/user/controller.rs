use crate::user::model::CreateUser;
use crate::{database::mongodb::Mongo, user::service::UserService};
use actix_web::{
    get,
    http::header::ContentType,
    post,
    web::Json,
    web::{Data, ServiceConfig},
    HttpResponse,
};
use futures::executor::block_on;
use std::sync::Arc;

#[get("")]
pub async fn read_users(mongodb: Data<Mongo>) -> HttpResponse {
    let mut user_service = UserService::new(mongodb.clone()).await;
    let users = user_service.all().await;
    if users.is_none() {
        return HttpResponse::Ok().json("[]");
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(users.unwrap())
}

#[post("")]
pub async fn create_user(mongodb: Data<Mongo>, usr: Json<CreateUser>) -> HttpResponse {
    let mut user = usr.into_inner();
    let mut user_service = UserService::new(mongodb.clone()).await;
    let response = user_service.create(&mut user).await;
    if response.is_none() {
        return HttpResponse::Conflict().json("Could not create user.");
    }
    HttpResponse::Ok().json(response.unwrap())
}

async fn init_database() {
    let mongodb = Data::from(Arc::new(Mongo::init(None).await));
    let mut user_service = UserService::new(mongodb.clone()).await;
    let mut user = CreateUser::default();
    user_service
        .create(&mut user)
        .await
        .unwrap_or("".to_string());
}

pub fn init_routes(config: &mut ServiceConfig) {
    block_on(async {
        init_database().await;
    });
    config.service(read_users);
    config.service(create_user);
}
