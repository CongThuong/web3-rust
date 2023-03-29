use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PreregResponse {
    pub id: String,
    pub first_name: String,
    pub sur_name: String,
    pub product: String,
    pub email: String,
    pub organization: String,
    pub message: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct PreregistrationData {
    pub note: PreregResponse,
}

#[derive(Serialize, Debug)]
pub struct SinglePreregResponse {
    pub status: String,
    pub data: PreregistrationData,
}

