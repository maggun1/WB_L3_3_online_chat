#[derive(Debug, Default)]
pub struct AppState {
    pub rooms: dashmap::DashMap<String, std::sync::Arc<crate::model::Room>>,
    pub user_count: std::sync::atomic::AtomicUsize,
}