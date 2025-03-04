use chrono::NaiveDate;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[remain::sorted]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LeagueOperation {
    Create {
        league_name: String,
    },
    Delete {
        id: Uuid,
    },
    SetName {
        id:          Uuid,
        league_name: String,
    },
}

#[remain::sorted]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl std::default::Default for MatchOperation {
    fn default() -> Self {
        MatchOperation::Create {
            name:       String::new(),
            event_date: NaiveDate::default(),
        }
    }
}
