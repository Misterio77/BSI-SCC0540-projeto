/// Servir assets na página. No caso, CSS.
use std::path::Path;
use rocket::{get, fs::NamedFile};

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
