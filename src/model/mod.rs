struct User {
    user_id: i32,
    username: String,
}

struct Message {
    message_id: i32,
    user_id: i32,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

struct Room {
    room_id: i32,
    name: String,

}