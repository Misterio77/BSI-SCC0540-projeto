pub mod common;
pub use common::{
    assets, database, error, pagination, template_or_json::TemplateOrJson as Response,
};

pub mod routes;
pub mod schema;
