/// Mais um código meio baixo nível
///
/// Aqui eu extraio a página da query string, para poder retornar as URIs bonitinho
/// apontando pra próxima e pra anterior, mantendo os argumentos do filtro sem usar
/// javascript.
use rocket::{http, outcome, request};
use serde::Serialize;
use std::{cmp, collections::HashMap};

#[derive(Serialize)]
pub struct Pages {
    pub current: u16,
    pub prev: u16,
    pub next: u16,
    pub prev_url: String,
    pub next_url: String,
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for Pages {
    type Error = std::convert::Infallible;
    async fn from_request(
        req: &'r request::Request<'_>,
    ) -> outcome::Outcome<Self, (http::Status, Self::Error), ()> {
        let uri = req.uri();

        let query = qstring::QString::from(uri.query().map(|q| q.as_str()).unwrap_or_default());

        let current = get_page(&query);

        let prev = current - 1;
        let next = current + 1;

        let prev_query = set_page(query.clone(), prev);
        let next_query = set_page(query, next);

        let prev_url = format!("{}?{}", uri.path(), prev_query);
        let next_url = format!("{}?{}", uri.path(), next_query);

        outcome::Outcome::Success(Pages {
            current,
            prev,
            next,
            prev_url,
            next_url,
        })
    }
}

fn get_page(query: &qstring::QString) -> u16 {
    let map: HashMap<&str, &str> = query.to_pairs().into_iter().collect();

    let value = map.get("pagina");
    value
        .and_then(|q| q.parse().ok())
        .map(|p| cmp::max(1, p))
        .unwrap_or(1)
}

fn set_page(query: qstring::QString, page: u16) -> qstring::QString {
    let mut map: HashMap<String, String> = query.into_pairs().into_iter().collect();

    *map.entry("pagina".into())
        .or_insert_with(|| page.to_string()) = page.to_string();

    qstring::QString::new(map.into_iter().collect())
}
