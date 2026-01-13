use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub from: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

