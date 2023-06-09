use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePreregSchema {
    pub id: String,
    pub first_name: String,
    pub sur_name: String,
    pub product: String,
    pub organization: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
