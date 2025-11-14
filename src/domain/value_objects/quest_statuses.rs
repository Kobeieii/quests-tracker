use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum QuestStatuses {
    #[default]
    Open,
    Injourney,
    Completed,
    Failed
}

impl fmt::Display for QuestStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuestStatuses::Open => write!(f, "Open"),
            QuestStatuses::Injourney => write!(f, "Injourney"),
            QuestStatuses::Completed => write!(f, "Completed"),
            QuestStatuses::Failed => write!(f, "Failed")
        }
    }
}