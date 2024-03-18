use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
