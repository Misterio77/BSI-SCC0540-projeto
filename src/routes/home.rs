/// Página inicial do site
use rocket::{get, request::FlashMessage, routes, Route};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("base", context! {flash})
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
