pub mod candidaturas;

use crate::error::ServerError;
use rocket::{
    catch,
    fs::NamedFile,
    get,
    http::{uri::Origin, Status},
    request::FlashMessage,
};
use rocket_dyn_templates::{context, Template};
use std::path::Path;

#[get("/")]
pub fn index(flash: Option<FlashMessage<'_>>, origin: &Origin<'_>) -> Template {
    Template::render("base", context! {flash, origin})
}

#[catch(404)]
pub fn not_found() -> ServerError {
    ServerError::builder()
        .code(Status::NotFound)
        .message("Rota não encontrada")
        .build()
}

pub struct CachedFile(NamedFile);

impl<'r> rocket::response::Responder<'r, 'static> for CachedFile {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build_from(self.0.respond_to(req)?)
            .raw_header("Cache-control", "max-age=86400")
            .ok()
    }
}

#[get("/style.css")]
pub async fn css() -> Option<CachedFile> {
    NamedFile::open(Path::new("assets/style.css"))
        .await
        .ok()
        .map(CachedFile)
}
