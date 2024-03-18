use crate::{
    database::mongodb::Mongo, error::AppError, user::model::User,
    utils::cursor_to_vec::CursorIntoVec, Result,
};
use actix_web::web::Data;
use mongodb::{
    bson::{doc, Document},
    results::InsertOneResult,
    Collection, Cursor,
};
use std::collections::HashMap;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub async fn new(mongodb: Data<Mongo>) -> Self {
        Self {
            collection: mongodb.collection(User::COLLECTION_NAME),
        }
    }

    pub async fn all(&mut self) -> Result<Vec<User>> {
        let user_cursor: Cursor<User> = self
            .collection
            .find(None, None)
            .await
            .map_err(AppError::from)?;
        let user_result: Vec<User> = user_cursor.into_vec().await;
        Ok(user_result)
    }

    pub async fn find(&mut self, options: &HashMap<&str, &str>) -> Result<Vec<User>> {
        let mut doc: Document = doc! {};
        for (key, value) in options {
            doc.extend(doc! {
                *key: *value
            });
        }
        let user_cursor = self
            .collection
            .find(doc, None)
            .await
            .map_err(AppError::from)?;
        let user_result: Vec<User> = user_cursor.into_vec().await;
        if Vec::len(&user_result) == 0 {
            return Err(AppError::new());
        } else {
        }
        Ok(user_result)
    }

    pub async fn insert(&self, user: &mut User) -> Result<InsertOneResult> {
        let insert: InsertOneResult = self
            .collection
            .insert_one(user, None)
            .await
            .map_err(AppError::from)?;
        Ok(insert)
    }
}
