use utoipa::OpenApi;

pub mod ssa;

#[derive(OpenApi)]
#[openapi(paths(ssa::get_name_stats))]
pub struct ApiDoc;
