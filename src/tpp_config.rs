use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TPPConfig {
    pub username: String,
    pub oauth_token: String,
}

// Uhh maybe something?
