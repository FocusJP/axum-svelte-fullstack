use serde::Serialize;
use utoipa::ToSchema;

pub mod request;
pub mod response;

#[derive(Debug, Serialize, ToSchema)]
pub struct NameStat {
    pub name: String,
    pub sex: char,
    pub year: i32,
    pub count: i64,
}
