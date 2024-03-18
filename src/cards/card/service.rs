use crate::cards::card::model::UpdateCard;
use crate::{
    cards::card::model::Card, cards::card::repository::CardRepository, database::mongodb::Mongo,
};
use actix_web::web::{Data, Json};
use std::collections::HashMap;

pub struct CardService {
    pub repository: CardRepository,
}

impl CardService {
    pub async fn new(mongodb: Data<Mongo>) -> Self {
        Self {
            repository: CardRepository::new(mongodb).await,
        }
    }
    pub async fn all(&mut self) -> Option<Vec<Card>> {
        match self.repository.all().await {
            Ok(cards) => Some(cards),
            Err(err) => None,
        }
    }

    pub async fn find_by_serial(&mut self, serial: &str) -> Option<Card> {
        let mut options = HashMap::<&str, &str>::new();
        HashMap::insert(&mut options, "serial_number", serial);
        match self.repository.find(&options).await {
            Ok(cards) => {
                let card = match cards.as_slice() {
                    [card] => card,
                    _ => panic!("expected single element"),
                };
                Some(card.clone())
            }
            Err(_) => None,
        }
    }

    pub async fn create(&mut self, card: &mut Card) -> Option<String> {
        let card = card.clone();
        let mut options = HashMap::<&str, &str>::new();
        HashMap::insert(&mut options, "serial_number", card.serial_number.as_str());
        match self.repository.find(&options).await {
            Ok(_) => None,
            Err(_) => {
                let mut card = Card {
                    id: None,
                    serial_number: card.serial_number,
                    fr_name: card.fr_name,
                    en_name: card.en_name,
                    jp_name: card.jp_name,
                    cost: card.cost,
                    power: card.power,
                    life: card.life,
                    prev: None,
                    next: None,
                    fr_effect: card.fr_effect,
                    en_effect: card.en_effect,
                    fr_trigger_effect: card.fr_trigger_effect,
                    en_trigger_effect: card.en_trigger_effect,
                    counter: card.counter,
                };
                match self.repository.insert(&mut card).await {
                    Ok(_) => Some(String::from("Successfully created card.")),
                    Err(_) => None,
                }
            }
        }
    }

    pub async fn update(&mut self, serial: &str, data: Json<UpdateCard>) -> Option<Card> {
        let mut card = data.into_inner();
        let c = self.repository.update(serial, &mut card).await;
        None
    }
}
