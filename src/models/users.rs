use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub username: String,
    pub cred: String,
}

#[async_graphql::Object]
impl User {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn email(&self) -> &str {
        self.email.as_str()
    }

    pub async fn username(&self) -> &str {
        self.username.as_str()
    }
}
#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub cred: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInfo {
    pub email: String,
    pub username: String,
    pub token: String,
}

#[async_graphql::Object]
impl SignInfo {
    pub async fn email(&self) -> &str {
        self.email.as_str()
    }

    pub async fn username(&self) -> &str {
        self.username.as_str()
    }

    pub async fn token(&self) -> &str {
        self.token.as_str()
    }
}