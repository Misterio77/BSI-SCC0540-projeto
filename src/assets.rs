/// Esse código serve para servir assets estáticos cacheados.
/// É meio baixo nível e irrelevante ao assunto do trabalho,
/// então provavelmente não vai ser interessante ver.
///
/// Provavelmente vou transformar em um biblioteca em algum ponto.
use normpath::PathExt;
use rocket::{
    error,
    fairing::{self, Fairing, Info, Kind},
    fs::NamedFile,
    http::{Method, Status},
    info, info_,
    response::Responder,
    Build, Orbit, Request, Response, Rocket,
};
use std::path::PathBuf;

pub struct Assets;

#[derive(Debug)]
struct AssetsContext {
    path: PathBuf,
    max_age: i32,
}

#[rocket::async_trait]
impl Fairing for Assets {
    fn info(&self) -> Info {
        let kind = Kind::Response | Kind::Ignite | Kind::Liftoff;
        Info {
            kind,
            name: "Static Assets",
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        use rocket::figment::value::magic::RelativePathBuf;

        let configured_dir = rocket
            .figment()
            .extract_inner::<RelativePathBuf>("assets_dir")
            .map(|path| path.relative());

        let relative_path = match configured_dir {
            Ok(dir) => dir,
            Err(e) if e.missing() => "assets/".into(),
            Err(e) => {
                rocket::config::pretty_print_error(e);
                return Err(rocket);
            }
        };

        let path = match relative_path.normalize() {
            Ok(path) => path.into_path_buf(),
            Err(e) => {
                error!("Invalid assets directory '{}': {}.", relative_path.display(), e);
                return Err(rocket);
            }
        };

        let max_age = rocket
            .figment()
            .extract_inner::<i32>("assets_max_age").unwrap_or(86400);

        Ok(rocket.manage(AssetsContext{path, max_age}))
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        use rocket::{figment::Source, log::PaintExt, yansi::Paint};

        let cm = rocket
            .state::<AssetsContext>()
            .expect("Template AssetsContext registered in on_ignite");

        info!("{}{}:", Paint::emoji("📐 "), Paint::magenta("Assets"));
        info_!("directory: {}", Paint::white(Source::from(&*cm.path)));
        info_!("cache max age: {}", Paint::white(cm.max_age));
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.status() != Status::NotFound {
            return;
        }

        let configuration = request.rocket().state::<AssetsContext>().unwrap();
        let mut path = configuration.path.clone();

        let mut segments = request.uri().path().segments();
        let first = segments.next();

        if request.method() == Method::Get && first == Some("assets") {
            let file_path = match segments.to_path_buf(false) {
                Ok(p) => p,
                Err(_) => return,
            };
            path.push(file_path);

            let file_response = match NamedFile::open(path).await {
                Ok(p) => p,
                Err(_) => return,
            };

            let new = Response::build_from(file_response.respond_to(request).unwrap())
                .raw_header("Cache-control", format!("max-age={}", configuration.max_age))
                .finalize();

            let status = new.status();
            response.set_status(status);
            response.merge(new);
        }
    }
}
