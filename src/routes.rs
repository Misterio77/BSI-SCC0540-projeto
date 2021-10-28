use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    let ctx = context! {};
    Template::render("base", ctx)
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
