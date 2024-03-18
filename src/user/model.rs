use crate::utils::deserialize_object_id;
use argonautica::Hasher;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: String,
}

impl User {
    pub const COLLECTION_NAME: &'static str = "users";

    pub fn role_from_str(&self) -> UserRole {
        match self.role.clone().as_str() {
            "ADMIN" => UserRole::Admin,
            "USER" => UserRole::User,
            "VIEWER" => UserRole::Viewer,
            &_ => UserRole::None,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        let mut hasher = Hasher::default();
        let password = hasher
            .with_secret_key(std::env::var("SECRET").unwrap())
            .with_password("password".as_bytes())
            .hash()
            .expect("Error");
        Self {
            id: Some(ObjectId::new()),
            email: String::from("johndoe@doe.com"),
            username: String::from("John"),
            password,
            role: String::from("User"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRole {
    Admin,
    User,
    Viewer,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    #[serde(deserialize_with = "deserialize_object_id", default)]
    pub _id: Option<ObjectId>,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl Default for CreateUser {
    fn default() -> Self {
        Self {
            _id: None,
            email: String::from("johndoe@doe.com"),
            username: String::from("John"),
            password: String::from("password"),
        }
    }
}
