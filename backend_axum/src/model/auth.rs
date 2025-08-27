use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Audience {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserAuth {
    #[serde(alias = "https://demoapp/email")]
    pub email: String,
    pub sub: String,
    pub iss: String,
    pub aud: Audience,
    pub exp: u64,
    pub iat: u64,
    pub permissions: Vec<String>
}
