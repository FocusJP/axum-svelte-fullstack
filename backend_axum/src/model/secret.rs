use serde::Serialize;

const HIDDEN_SECRET: &str = "***";

#[derive(Clone)]
pub struct SecretString(pub String);

impl Serialize for SecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(HIDDEN_SECRET)
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SecretString").field(&HIDDEN_SECRET).finish()
    }
}

impl<S: Into<String>> From<S> for SecretString {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}
