use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Status{
    pub status:String
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}