use serde::Serialize;
use utoipa::ToSchema;

use super::NameStat;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetNameStatsResponse(pub Vec<NameStat>);
