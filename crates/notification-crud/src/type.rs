use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    #[default]
    New,
    Open,
    Resolved,
    Suppressed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            Status::New => "NEW",
            Status::Open => "OPEN",
            Status::Resolved => "RESOLVED",
            Status::Suppressed => "SUPPRESSED",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub enum Severity {
    #[default]
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity_str = match self {
            Severity::Info => "INFO",
            Severity::Warn => "WARN",
            Severity::Error => "ERROR",
        };
        write!(f, "{}", severity_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    #[serde(rename = "PK")]
    pub pk: String,
    pub title: String,
    pub details: Option<String>,
    pub status: Status,
    pub severity: Severity,
    pub notified_at: String,
    pub url: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Input {
//     Create(CreateParameter),
//     Read(ReadParameter),
//     Update(UpdateParameter),
//     Delete(DeleteParameter),
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreateParameter {
//     pub title: String,
//     pub details: Option<String>,
//     pub status: Option<Status>,
//     pub severity: Option<Severity>,
//     pub url: Option<String>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ReadParameter {
//     pub status_query: Vec<Status>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateParameter {
//     #[serde(rename = "PK")]
//     pub pk: String,
//     pub title: Option<String>,
//     pub details: Option<String>,
//     pub status: Option<Status>,
//     pub severity: Option<Severity>,
//     pub url: Option<String>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct DeleteParameter {
//     #[serde(rename = "PK")]
//     pub pk: String,
// }
