use rocket::serde::json::Json;
use rocket::{http, request, response};
use rocket_dyn_templates::Template;
use serde::Serialize;

use std::borrow::Cow;

#[derive(Debug)]
pub struct TemplateOrJson<D>(Template, Json<D>);

impl<D> TemplateOrJson<D> {
    pub fn new<S, C>(json_data: D, template_name: S, template_ctx: C) -> Self
    where
        S: Into<Cow<'static, str>>,
        C: Serialize,
    {
        Self(
            Template::render(template_name, template_ctx),
            Json(json_data),
        )
    }
}

impl<'r, T: Serialize> response::Responder<'r, 'static> for TemplateOrJson<T> {
    fn respond_to(self, req: &'r request::Request<'_>) -> response::Result<'static> {
        let media_type = req.accept().map(|a| a.preferred().media_type());

        if media_type == Some(&http::MediaType::JSON) {
            self.1.respond_to(req)
        } else {
            self.0.respond_to(req)
        }
    }
}
