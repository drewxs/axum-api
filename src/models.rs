use std::time::SystemTime;

use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
