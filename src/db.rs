use crate::response::{PreregistrationData, PreregResponse, SinglePreregResponse};
use crate::{
    error::Error::*, model::PreregistrationModel, schema::CreatePreregSchema, Result,
};
use chrono::prelude::*;
use mongodb::bson::{doc, Document};
use mongodb::{bson, options::ClientOptions, Client, Collection};

#[derive(Clone, Debug)]
pub struct DB {
    pub preregistration_collection: Collection<PreregistrationModel>,
    pub collection: Collection<Document>,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name: String =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
        let mongodb_preregistration_collection: String =
            std::env::var("MONGODB_PREREGISTRATION_COLLECTION").expect("MONGODB_PREREGISTRATION_COLLECTION must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let preregistration_collection = database.collection(mongodb_preregistration_collection.as_str());
        let collection = database.collection::<Document>(mongodb_preregistration_collection.as_str());

        println!("✅ Database connected successfully");

        Ok(Self {
            preregistration_collection,
            collection,
        })
    }


    pub async fn create_prereg(&self, body: &CreatePreregSchema) -> Result<Option<SinglePreregResponse>> {
        let serialized_data = bson::to_bson(&body).map_err(MongoSerializeBsonError)?;
        let document = serialized_data.as_document().unwrap();
        let datetime = Utc::now();
        let web3id = body.id.to_owned();

        let mut doc_with_dates = doc! {"createdAt": datetime, "updatedAt": datetime, "web3id": web3id};
        doc_with_dates.extend(document.clone());

        let insert_result = self
            .collection
            .insert_one(&doc_with_dates, None)
            .await
            .map_err(|e| {
                if e.to_string()
                    .contains("E11000 duplicate key error collection")
                {
                    return MongoDuplicateError(e);
                }
                return MongoQueryError(e);
            })?;

        let new_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("issue with new _id");


        let note_doc = self
            .preregistration_collection
            .find_one(doc! {"_id":new_id }, None)
            .await
            .map_err(MongoQueryError)?;

        if note_doc.is_none() {
            return Ok(None);
        }

        let note_response = SinglePreregResponse {
            status: "success".to_string(),
            data: PreregistrationData {
                note: self.doc_to_note(&note_doc.unwrap()).unwrap(),
            },
        };

        Ok(Some(note_response))
    }


    fn doc_to_note(&self, note: &PreregistrationModel) -> Result<PreregResponse> {
        let note_response = PreregResponse {
            id: note.web3id.to_owned(),
            first_name: note.first_name.to_owned(),
            sur_name: note.sur_name.to_owned(),
            product: note.product.to_owned(),
            email: note.email.to_owned(),
            organization: note.organization.to_owned(),
            message: note.message.to_owned(),
            createdAt: note.createdAt,
            updatedAt: note.updatedAt,
        };

        Ok(note_response)
    }
}
