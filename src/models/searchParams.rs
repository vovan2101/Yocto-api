use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchParams {
    pub name: String,
}
