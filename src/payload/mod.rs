#[derive(serde::Deserialize)]
pub struct JoinRoomPayload {
    pub username: String,
    pub room_name: String,
}

#[derive(serde::Deserialize)]
pub struct LeaveRoomPayload {
    pub username: String,
    pub room_name: String,
}

#[derive(serde::Deserialize)]
pub struct SendMessagePayload {
    pub username: String,
    pub room_name: String,
    pub content: String,
}

#[derive(serde::Deserialize)]
pub struct GetMessagesQuery {
    pub username: String,
    pub room_name: String,
}