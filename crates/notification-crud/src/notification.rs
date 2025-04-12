use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum Status {
    #[default]
    #[serde(rename = "NEW")]
    New,

    #[serde(rename = "OPEN")]
    Open,

    #[serde(rename = "RESOLVED")]
    Resolved,

    #[serde(rename = "SUPPRESSED")]
    Suppressed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                Status::New => "NEW",
                Status::Open => "OPEN",
                Status::Resolved => "RESOLVE",
                Status::Suppressed => "SUPPRESSED",
            })
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum Severity {
    #[default]
    #[serde(rename = "INFO")]
    Info,

    #[serde(rename = "WARN")]
    Warn,

    #[serde(rename = "ERROR")]
    Error,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                Severity::Info => "INFO",
                Severity::Warn => "WARN",
                Severity::Error => "ERROR",
            })
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Notification {
    pub pk: String,
    pub title: String,
    pub details: Option<String>,
    pub severity: Severity,
    pub status: Status,
    pub url: Option<String>,
    pub notified_at: String,
}

// Request

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    #[serde(rename = "put")]
    Put(PutCommand),

    #[serde(rename = "list")]
    List(ListCommand),

    #[serde(rename = "delete")]
    Delete(DeleteCommand),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PutCommand {
    pub pk: Option<String>,
    pub title: String,
    pub details: Option<String>,
    pub severity: Option<Severity>,
    pub status: Option<Status>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ListCommand {
    pub status: Vec<Status>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeleteCommand {
    pub pk: String,
}

// Response

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Response {
    pub results: Vec<Notification>,
}
