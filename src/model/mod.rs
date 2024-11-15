#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub username: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Message {
    pub username: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct Room {
    pub users: dashmap::DashMap<String, User>,
    pub messages: tokio::sync::RwLock<Vec<Message>>,
}
