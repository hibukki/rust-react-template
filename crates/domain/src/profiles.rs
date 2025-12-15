use db::DbPool;
use shared::types::{Profile, WsEvent};
use tokio::sync::broadcast;

/// ProfileService centralizes all profile mutations.
/// Broadcasts happen automatically after successful DB writes.
#[derive(Clone)]
pub struct ProfileService {
    db: DbPool,
    events_tx: broadcast::Sender<WsEvent>,
}

impl ProfileService {
    pub fn new(db: DbPool, events_tx: broadcast::Sender<WsEvent>) -> Self {
        Self { db, events_tx }
    }

    /// Create a new profile for a user (called during registration)
    pub async fn create_profile(
        &self,
        user_id: i64,
        display_name: &str,
    ) -> Result<Profile, sqlx::Error> {
        let profile = db::create_profile(&self.db, user_id, display_name).await?;
        let _ = self.events_tx.send(WsEvent::Profile(profile.clone()));
        Ok(profile)
    }

    /// Update an existing profile
    pub async fn update_profile(
        &self,
        id: i64,
        display_name: Option<&str>,
        bio: Option<&str>,
    ) -> Result<Option<Profile>, sqlx::Error> {
        let profile = db::update_profile(&self.db, id, display_name, bio).await?;
        if let Some(ref p) = profile {
            let _ = self.events_tx.send(WsEvent::Profile(p.clone()));
        }
        Ok(profile)
    }

    /// Get a single profile by ID (no broadcast)
    pub async fn get_profile_by_id(&self, id: i64) -> Result<Option<Profile>, sqlx::Error> {
        db::get_profile_by_id(&self.db, id).await
    }

    /// Get a profile by user ID (no broadcast)
    pub async fn get_profile_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Option<Profile>, sqlx::Error> {
        db::get_profile_by_user_id(&self.db, user_id).await
    }

    /// Get all profiles (no broadcast - used for initial state)
    pub async fn list_profiles(&self) -> Result<Vec<Profile>, sqlx::Error> {
        db::list_profiles(&self.db).await
    }
}
