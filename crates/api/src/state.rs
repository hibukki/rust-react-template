use crate::config::Config;
use db::DbPool;
use shared::types::WsEvent;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};

pub type Sessions = Arc<RwLock<HashMap<String, i64>>>;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: DbPool,
    pub events_tx: broadcast::Sender<WsEvent>,
    pub sessions: Sessions,
}

impl AppState {
    pub fn new(config: Config, db: DbPool) -> Self {
        let (events_tx, _) = broadcast::channel(100);
        Self {
            config: Arc::new(config),
            db,
            events_tx,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<WsEvent> {
        self.events_tx.subscribe()
    }
}
