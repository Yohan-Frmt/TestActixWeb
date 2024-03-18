use crate::cards::card::model::UpdateCard;
use crate::{cards::card::model::Card, database::mongodb::Mongo, error::AppError, Result};
use actix_web::web::Data;
use futures::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Collection, Cursor};
use std::collections::HashMap;

pub struct CardRepository {
    collection: Collection<Card>,
}

impl CardRepository {
    pub async fn new(mongodb: Data<Mongo>) -> Self {
        Self {
            collection: mongodb.collection(Card::COLLECTION_NAME),
        }
    }

    pub async fn all(&mut self) -> Result<Vec<Card>> {
        let mut card_cursor: Cursor<Card> = self
            .collection
            .find(None, None)
            .await
            .map_err(AppError::from)?;
        let mut card_result: Vec<Card> = Vec::new();
        while let Some(card) = card_cursor.next().await {
            card_result.push(card?);
        }
        Ok(card_result)
    }

    pub async fn find(&mut self, options: &HashMap<&str, &str>) -> Result<Vec<Card>> {
        let mut doc: Document = doc! {};
        for (key, value) in options {
            doc.extend(doc! {
                *key: *value
            });
        }
        let mut user_cursor = self
            .collection
            .find(doc, None)
            .await
            .map_err(AppError::from)?;
        let mut user_result: Vec<Card> = Vec::new();
        while let Some(user) = user_cursor.next().await {
            user_result.push(user?);
        }
        if user_result.is_empty() {
            return Err(AppError::new());
        }
        Ok(user_result)
    }

    pub async fn insert(&self, card: &mut Card) -> Result<InsertOneResult> {
        let card = self
            .collection
            .insert_one(card, None)
            .await
            .map_err(AppError::from)?;
        Ok(card)
    }

    pub async fn update(&self, serial: &str, card: &mut UpdateCard) -> Result<UpdateResult> {
        let card = self
            .collection
            .update_one(
                doc! {
                    "serial_number": serial
                },
                doc! {
                    "$set":{
                        ..card
                        },
                },
                None,
            )
            .await
            .map_err(AppError::from)?;
        Ok(card)
    }
}
