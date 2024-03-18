use crate::cards::card::model::UpdateCard;
use crate::{
    cards::card::{model::Card, service::CardService},
    database::mongodb::Mongo,
};
use actix_web::{
    get,
    http::header::ContentType,
    patch, post,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use futures::executor::block_on;
use std::sync::Arc;

#[post("")]
async fn create_card(mongodb: Data<Mongo>, crd: Json<Card>) -> impl Responder {
    let mut card = crd.into_inner();
    let mut card_service = CardService::new(mongodb.clone()).await;
    let response = card_service.create(&mut card).await;
    if response.is_none() {
        return HttpResponse::Conflict().json("Could not create card.");
    }
    HttpResponse::Ok().json(response.unwrap())
}

#[get("")]
async fn read_cards(mongodb: Data<Mongo>) -> impl Responder {
    let mut card_service = CardService::new(mongodb.clone()).await;
    let cards = card_service.all().await;
    if cards.is_none() {
        return HttpResponse::Ok().json("[]");
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(cards.unwrap())
}

#[get("/{serial}")]
async fn read_card_by_serial(mongodb: Data<Mongo>, path: Path<String>) -> impl Responder {
    let serial = path.into_inner();
    let mut card_service = CardService::new(mongodb.clone()).await;
    let card = card_service.find_by_serial(&serial).await;
    if card.is_none() {
        return HttpResponse::Ok().json("[]");
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(card.unwrap())
}

#[patch("/{serial}")]
async fn update_card(
    mongodb: Data<Mongo>,
    path: Path<String>,
    body: Json<UpdateCard>,
) -> impl Responder {
    let serial = path.into_inner();
    let mut card_service = CardService::new(mongodb.clone()).await;
    let card = card_service.update(&serial, body).await;
    if card.is_none() {
        return HttpResponse::Ok().json("[]");
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(card.unwrap())
}

async fn init_database() {
    let mongodb = Data::from(Arc::new(Mongo::init(None).await));
    let mut card_service = CardService::new(mongodb.clone()).await;
    let mut card = Card::default();
    card_service
        .create(&mut card)
        .await
        .unwrap_or("".to_string());
}

pub fn init_routes(config: &mut ServiceConfig) {
    block_on(async {
        init_database().await;
    });
    config.service(read_cards);
    config.service(read_card_by_serial);
    config.service(create_card);
}
