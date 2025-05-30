use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StageName {
    #[serde(rename = "dev")]
    Development,

    #[serde(rename = "stg")]
    Staging,

    #[serde(rename = "prod")]
    Production,
}

impl std::fmt::Display for StageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                StageName::Development => "dev",
                StageName::Staging => "stg",
                StageName::Production => "prod",
            })
        )
    }
}

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
                Status::Resolved => "RESOLVED",
                Status::Suppressed => "SUPPRESSED",
            })
        )
    }
}

impl<T> From<T> for Status
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        match value.as_ref() {
            "NEW" => Self::New,
            "OPEN" => Self::Open,
            "RESOLVE" => Self::Resolved,
            "SUPPRESSED" => Self::Suppressed,
            _ => Self::New,
        }
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

impl<T> From<T> for Severity
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        match value.as_ref() {
            "INFO" => Self::Info,
            "WARN" => Self::Warn,
            "ERROR" => Self::Error,
            _ => Self::Info,
        }
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
    pub title: Option<String>,
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
