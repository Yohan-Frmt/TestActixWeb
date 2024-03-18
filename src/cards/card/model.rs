use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub serial_number: String,
    pub fr_name: String,
    pub en_name: String,
    pub jp_name: String,
    pub cost: Option<i32>,
    pub power: Option<i32>,
    pub life: Option<i32>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub fr_effect: Option<String>,
    pub en_effect: Option<String>,
    pub fr_trigger_effect: Option<String>,
    pub en_trigger_effect: Option<String>,
    pub counter: Option<i32>,
}

impl Card {
    pub const COLLECTION_NAME: &'static str = "cards";
}

impl Default for Card {
    fn default() -> Self {
        Self {
            id: None,
            serial_number: String::from("SERIAL-001"),
            fr_name: String::from("Nom"),
            en_name: String::from("Name"),
            jp_name: String::from("名字"),
            cost: None,
            power: None,
            life: None,
            prev: None,
            next: None,
            fr_effect: None,
            en_effect: None,
            fr_trigger_effect: None,
            en_trigger_effect: None,
            counter: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCard {
    pub serial_number: String,
    pub fr_name: String,
    pub en_name: String,
    pub jp_name: String,
    pub cost: Option<i32>,
    pub power: Option<i32>,
    pub life: Option<i32>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub fr_effect: Option<String>,
    pub en_effect: Option<String>,
    pub fr_trigger_effect: Option<String>,
    pub en_trigger_effect: Option<String>,
    pub counter: Option<i32>,
}
