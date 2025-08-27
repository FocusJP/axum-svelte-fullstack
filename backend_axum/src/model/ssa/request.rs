use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetNameStatsQuery {
    pub names: Vec<String>,
    pub start_year: i32,
    pub end_year: i32,
}
