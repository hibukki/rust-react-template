use crate::config::Config;
use db::DbPool;
use domain::ProfileService;
use shared::types::WsEvent;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};

pub type Sessions = Arc<RwLock<HashMap<String, i64>>>;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: DbPool,
    pub profile_service: ProfileService,
    pub sessions: Sessions,
    events_tx: broadcast::Sender<WsEvent>,
}

impl AppState {
    pub fn new(config: Config, db: DbPool) -> Self {
        let (events_tx, _) = broadcast::channel(100);
        let profile_service = ProfileService::new(db.clone(), events_tx.clone());
        Self {
            config: Arc::new(config),
            db,
            profile_service,
            events_tx,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<WsEvent> {
        self.events_tx.subscribe()
    }
}
