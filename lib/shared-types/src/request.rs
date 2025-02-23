use chrono::NaiveDate;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[remain::sorted]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchOperation {
    Create {
        name:       String,
        event_date: NaiveDate,
    },
    Delete {
        id: Uuid,
    },
    SetDate {
        id:         Uuid,
        event_date: NaiveDate,
    },
    SetName {
        id:   Uuid,
        name: String,
    },
}
