use bson::{doc, oid::ObjectId};
use chrono::{Utc, DateTime};
use mongodb::{Collection, options::{ClientOptions, ResolverConfig}, Client, results::{InsertOneResult, UpdateResult, DeleteResult}, Cursor};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub data: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_update: DateTime<Utc>
}
pub struct DB {
    collection: Collection<File>,
}

impl DB {
    pub async fn new(client_uri: String, database: &str, collection: &str) -> DB {
        let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await.expect("Error create options");

        let client = Client::with_options(options).expect("Error create client");

        DB {
            collection: client.database(database).collection::<File>(collection)
        }
    }

    pub async fn insert_one(&self, title: &str, data: &str) -> InsertOneResult {
        let file = File {
            id: None,
            title: title.to_string(),
            data: data.to_string(),
            last_update: Utc::now(),
        };

        self.collection.insert_one(file, None).await.expect("Error insert")
    }

    pub async fn update_one(&self, id: ObjectId, data: &str) -> UpdateResult {
        self.collection.update_one( doc! {
               "_id": id
            }, doc! {
               "$set": { "data": data, "last_update": Utc::now() }
            }, None).await.expect("Error update document")
    }

    pub async fn delete_one(&self, id: ObjectId) -> DeleteResult {
        self.collection.delete_one( doc! {
            "_id": id
        }, None).await.expect("Error delete document")
    }

    pub async fn get_all(&self) -> Cursor<File> {
        self.collection.find(None, None).await.expect("Error get documents")
    }
}