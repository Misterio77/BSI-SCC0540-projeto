/// Esse código serve para reescrever requests POST /blabla/delete
/// em DELETE /blabla. Útil para formulários (que só suportam GET e POST)
///
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Method,
    Data, Request,
};

pub struct PostAsDelete;

#[rocket::async_trait]
impl Fairing for PostAsDelete {
    fn info(&self) -> Info {
        let kind = Kind::Request;
        Info {
            kind,
            name: "POST as DELETE",
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let last = request.uri().path().segments().last();
        if request.method() == Method::Post && last == Some("delete") {
            request.set_method(Method::Delete);
            request.set_uri(
                request
                    .uri()
                    .map_path(|p| p.strip_suffix("/delete").unwrap_or(p))
                    .unwrap(),
            );
        }
    }
}
