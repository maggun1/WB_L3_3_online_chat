use axum::{
    extract::{State, Query},
    http::StatusCode,
    Json,
};

use crate::model::{
    User,
    Message,
    Room,
};

use crate::payload::{
    JoinRoomPayload,
    LeaveRoomPayload,
    SendMessagePayload,
    GetMessagesQuery
};

use crate::state::AppState;

use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::RwLock;


pub async fn join_room(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JoinRoomPayload>)
    -> StatusCode {
    let room = state.rooms
        .entry(payload.room_name.clone())
        .or_insert_with(|| {
            tracing::info!("Room '{}' created.", payload.room_name);
            Arc::new(Room {
                users: DashMap::new(),
                messages: RwLock::new(Vec::new()),
            })
        })
        .clone();

    let user = User {
        username: payload.username.clone(),
    };

    if room.users.contains_key(&user.username) {
        tracing::error!("User '{}' already in room '{}'.", payload.username, payload.room_name);
        return StatusCode::CONFLICT;
    }

    room.users.insert(user.username.clone(), user.clone());
    state.user_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    tracing::info!("User '{}' joined room '{}'.", payload.username, payload.room_name);
    tracing::info!("{} users online.", state.user_count.load(std::sync::atomic::Ordering::Relaxed));
    StatusCode::OK
}

pub async fn leave_room(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LeaveRoomPayload>)
    -> StatusCode {
    if let Some(room) = state.rooms.get(&payload.room_name) {
        if !room.users.contains_key(&payload.username) {
            tracing::error!("User '{}' is not in room '{}'.", payload.username, payload.room_name);
            return StatusCode::CONFLICT;
        }

        room.users.remove(&payload.username);
        state.user_count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        tracing::info!("User '{}' left room '{}'.", payload.username, payload.room_name);
        tracing::info!("{} users online.", state.user_count.load(std::sync::atomic::Ordering::Relaxed));

        if room.users.is_empty() {
            room.users.remove(&payload.room_name);
            tracing::info!("Room '{}' deleted.", payload.room_name);
        }

        StatusCode::OK
    } else {
        tracing::error!("Room '{}' not found.", payload.room_name);
        StatusCode::NOT_FOUND
    }

}

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SendMessagePayload>)
    -> StatusCode {
    if let Some(room) = state.rooms.get(&payload.room_name) {
        if !room.users.contains_key(&payload.username) {
            tracing::error!("User '{}' is not in room '{}'.", payload.username, payload.room_name);
            return StatusCode::CONFLICT;
        }

        let message = Message {
            username: payload.username.clone(),
            content: payload.content.clone(),
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        room.messages.write().await.push(message);
        tracing::info!("Message sent by user '{}' in room '{}'.", payload.username, payload.room_name);
        StatusCode::OK
    } else {
        tracing::error!("Room '{}' not found.", payload.room_name);
        StatusCode::NOT_FOUND
    }
}

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GetMessagesQuery>)
    -> Result<Json<Vec<Message>>, StatusCode> {
    if let Some(room) = state.rooms.get(&query.room_name) {
        if !room.users.contains_key(&query.username) {
            tracing::error!("User '{}' is not in room '{}'.", query.username, query.room_name);
            return Err(StatusCode::CONFLICT);
        }

        let messages = room.messages.read().await.clone();
        tracing::info!("Messages received from room '{}'.", query.room_name);
        Ok(Json(messages))
    } else {
        tracing::error!("Room '{}' not found.", query.room_name);
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_rooms(State(state): State<Arc<AppState>>)
    -> Json<Vec<String>> {
    state.user_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut room_names = vec![];
    for room in state.rooms.iter() {
        room_names.push(room.key().clone());
    }
    tracing::info!("Rooms received.");
    Json(room_names)
}
