use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/types/bindings/")]
pub struct Profile {
    pub id: i64,
    pub user_id: i64,
    pub display_name: String,
    pub bio: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/types/bindings/")]
#[serde(tag = "type", content = "data")]
pub enum WsEvent {
    ProfileUpdated(Profile),
    ProfileCreated(Profile),
}
