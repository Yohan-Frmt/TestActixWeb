extern crate dotenv;
use dotenv::dotenv;
use mongodb::{Client, Collection, Database};

pub struct Mongo {
    pub database: Database,
}

impl Mongo {
    pub async fn init(db: Option<&str>) -> Self {
        dotenv().ok();
        let mongo_uri: String = std::env::var("MONGO_URI").expect("MONGO_URI env may not be set");
        let client: Client = Client::with_uri_str(&mongo_uri)
            .await
            .expect("Error with Mongo Client");
        let db_name =
            db.unwrap_or_else(|| &std::env::var("DB_NAME").expect("DB_NAME env may not be set"));
        let database: Database = client.database(db_name);
        Self { database }
    }

    pub fn collection<T>(&self, col: &str) -> Collection<T> {
        self.database.collection::<T>(col)
    }
}
