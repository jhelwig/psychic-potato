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
    SetDescription {
        id:          Uuid,
        description: Option<String>,
    },
    SetEndDate {
        id:       Uuid,
        end_date: Option<NaiveDate>,
    },
    SetName {
        id:          Uuid,
        league_name: String,
    },
    SetStartDate {
        id:         Uuid,
        start_date: Option<NaiveDate>,
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

#[remain::sorted]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClassOperation {
    Create {
        name:        String,
        description: Option<String>,
    },
    Delete {
        id: Uuid,
    },
    SetDescription {
        id:          Uuid,
        description: Option<String>,
    },
    SetName {
        id:   Uuid,
        name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SmCsvExportUpload {
    pub filename: String,
    pub content:  String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}
