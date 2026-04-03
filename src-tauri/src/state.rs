use std::sync::{Arc, Mutex};
use crate::domain::entities::TypingStats;
use crate::infrastructure::persistence::Database;

pub struct AppState {
    pub stats: Arc<Mutex<TypingStats>>,
    pub db: Arc<Mutex<Option<Database>>>,
    pub is_tracking: Arc<Mutex<bool>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(TypingStats::new())),
            db: Arc::new(Mutex::new(None)),
            is_tracking: Arc::new(Mutex::new(false)),
        }
    }
}
