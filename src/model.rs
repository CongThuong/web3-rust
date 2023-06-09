use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreregistrationModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub web3id: String,
    pub first_name: String,
    pub sur_name: String,
    pub product: String,
    pub email: String,
    pub organization: String,
    pub message: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}
