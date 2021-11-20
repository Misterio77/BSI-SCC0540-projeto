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
    info, info_,
    outcome::IntoOutcome,
    request::{self, FromRequest, Request},
    response::{self, Responder, Response},
    Build, Orbit, Rocket,
};
use std::io;
use std::path::{Path, PathBuf};

/// Representa o estado de configuração dos assets
#[derive(Debug)]
pub struct Assets {
    path: PathBuf,
    cache_max_age: i32,
}

impl Assets {
    pub fn fairing() -> AssetsFairing {
        AssetsFairing
    }
    pub async fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<Asset> {
        let mut asset_path = self.path.clone();
        asset_path.push(path);
        let file = NamedFile::open(Path::new(&asset_path)).await?;
        let cache_max_age = self.cache_max_age;
        Ok(Asset {
            file,
            cache_max_age,
        })
    }
}

/// Obtém o estado num request
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Assets {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        req.rocket().state::<Assets>().or_forward(())
    }
}

/// Representa um asset a ser servido (criado com Assets::open)
pub struct Asset {
    file: NamedFile,
    cache_max_age: i32,
}

/// Resposta de um request
impl<'r> Responder<'r, 'static> for Asset {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let cache_control = format!("max-age={}", self.cache_max_age);
        Response::build_from(self.file.respond_to(req)?)
            .raw_header("Cache-control", cache_control)
            .ok()
    }
}

/// Fairing para buscar e gerir a configuração
pub struct AssetsFairing;

#[rocket::async_trait]
impl Fairing for AssetsFairing {
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
                error!(
                    "Invalid assets directory '{}': {}.",
                    relative_path.display(),
                    e
                );
                return Err(rocket);
            }
        };

        let cache_max_age = rocket
            .figment()
            .extract_inner::<i32>("assets_max_age")
            .unwrap_or(86400);

        Ok(rocket.manage(Assets {
            path,
            cache_max_age,
        }))
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        use rocket::{figment::Source, log::PaintExt, yansi::Paint};

        let state = rocket
            .state::<Assets>()
            .expect("Template AssetsContext registered in on_ignite");

        info!("{}{}:", Paint::emoji("📐 "), Paint::magenta("Assets"));
        info_!("directory: {}", Paint::white(Source::from(&*state.path)));
        info_!("cache max age: {}", Paint::white(state.cache_max_age));
    }
}
