mod model;
mod handler;
mod state;
mod payload;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let state = std::sync::Arc::new(state::AppState::default());

    let router = axum::Router::new()
        .route("/join", axum::routing::post(handler::join_room))
        .route("/leave", axum::routing::post(handler::leave_room))
        .route("/send", axum::routing::post(handler::send_message))
        .route("/messages", axum::routing::get(handler::get_messages))
        .route("/rooms", axum::routing::get(handler::get_rooms))
        .with_state(state.clone());


    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Online-Chat server started at 'http://{}'", addr);
    axum::serve(listener, router).await.unwrap();
}
