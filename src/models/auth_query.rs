use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthQuery {
    pub code: String,
}
