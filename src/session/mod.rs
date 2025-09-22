use chrono::{DateTime, Utc};
use serenity::all::{ChannelId, UserId};
use uuid::Uuid;

pub mod manager;
pub use manager::SessionManager;

#[derive(Debug, Clone)]
pub struct MathSession {
    pub id: Uuid,
    pub user_id: UserId,
    pub channel_id: ChannelId,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub conversation_history: Vec<String>,
}

impl MathSession {
    pub fn new(user_id: UserId, channel_id: ChannelId) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            channel_id,
            created_at: now,
            last_activity: now,
            conversation_history: Vec::new(),
        }
    }

    pub fn add_message(&mut self, content: String) {
        self.conversation_history.push(content);
        self.last_activity = Utc::now();
    }

    pub fn is_expired(&self, timeout_minutes: i64) -> bool {
        let timeout_duration = chrono::Duration::minutes(timeout_minutes);
        Utc::now() - self.last_activity > timeout_duration
    }
}