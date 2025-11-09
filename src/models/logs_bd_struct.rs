use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord<T, Y> {
    pub id: T,
    pub user_public_key: String,
    pub created_at: Y,
    pub source: String,
    pub error_code: Option<String>,
    pub message: String,
    pub criticality: bool,
    pub context: Option<serde_json::Value>
}

pub type NewLogRecord = LogRecord<Option<i32>, ()>;

pub type StoredLogRecord = LogRecord<i32, NaiveDateTime>;

impl NewLogRecord {
    pub fn with_id(self, id: i32, created_at: NaiveDateTime) -> StoredLogRecord {
        StoredLogRecord {
            id,
            user_public_key: self.user_public_key,
            created_at,
            source: self.source,
            error_code: self.error_code,
            message: self.message,
            criticality: self.criticality,
            context: self.context
        }
    }
}