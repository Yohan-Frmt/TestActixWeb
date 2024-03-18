use crate::user::model::CreateUser;
use crate::{database::mongodb::Mongo, user::model::User, user::repository::UserRepository};
use actix_web::web::Data;
use argonautica::Hasher;
use std::collections::HashMap;

pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub async fn new(mongodb: Data<Mongo>) -> Self {
        Self {
            repository: UserRepository::new(mongodb).await,
        }
    }

    pub async fn all(&mut self) -> Option<Vec<User>> {
        match self.repository.all().await {
            Ok(users) => Some(users),
            Err(_) => None,
        }
    }

    pub async fn create(&mut self, user: &mut CreateUser) -> Option<String> {
        let mut options = HashMap::<&str, &str>::new();
        options.insert("email", user.email.as_str());
        match self.repository.find(&options).await {
            Ok(_) => None,
            Err(_) => {
                let mut hasher = Hasher::default();
                let hash = hasher
                    .with_secret_key(std::env::var("SECRET").unwrap())
                    .with_password(user.password.as_bytes())
                    .hash();
                match hash {
                    Ok(str) => user.password = str,
                    Err(err) => println!("{err}"),
                }
                let mut usr = User {
                    id: user._id,
                    role: String::from("USER"),
                    password: user.password.to_owned(),
                    email: user.email.to_owned(),
                    username: user.username.to_owned(),
                };
                match self.repository.insert(&mut usr).await {
                    Ok(_) => Some(String::from("Successfully registered user.")),
                    Err(_) => None,
                }
            }
        }
    }
}
