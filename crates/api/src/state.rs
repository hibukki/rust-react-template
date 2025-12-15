use crate::config::Config;
use db::DbPool;
use shared::types::WsEvent;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: DbPool,
    pub events_tx: broadcast::Sender<WsEvent>,
}

impl AppState {
    pub fn new(config: Config, db: DbPool) -> Self {
        let (events_tx, _) = broadcast::channel(100);
        Self {
            config: Arc::new(config),
            db,
            events_tx,
        }
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<WsEvent> {
        self.events_tx.subscribe()
    }
}
