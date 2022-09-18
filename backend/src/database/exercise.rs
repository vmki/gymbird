use super::ID;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: ID,
    pub name: String,
    pub image_name: String,
}

impl Exercise {
    pub fn new<T>(name: T, image_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.as_ref().to_string(),
            image_name: image_name.as_ref().to_string(),
        }
    }
}

impl From<&tokio_postgres::Row> for Exercise {
    fn from(data: &tokio_postgres::Row) -> Self {
        Self {
            id: data.get("id"),
            name: data.get("name"),
            image_name: data.get("image_name"),
        }
    }
}
