/// Página inicial do site
use rocket::{get, request::FlashMessage};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("base", context! {flash})
}
