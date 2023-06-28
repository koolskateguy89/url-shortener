use actix_files::{Directory, Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    web::{self, ServiceConfig},
    HttpRequest,
};

use std::path::PathBuf;

use crate::middleware;
use crate::AppState;

/// Serve static files for Yew frontend under `/yew/{mount_path}`
///
/// Note: this won't work if using `cargo shuttle run` at repo top level.
///
/// https://actix.rs/docs/static-files
///
/// https://yew.rs/docs/more/deployment
fn yew_app(mount_path: &str, serve_from: impl Into<PathBuf>) -> Files {
    // Using a default handler to always show the Yew app
    // see https://yew.rs/docs/more/deployment#serving-indexhtml-as-fallback
    let default_handler = |req: ServiceRequest| async {
        let state: &web::Data<AppState> = req.app_data().expect("App data not set");

        let yew_folder = state.static_folder.join("yew");
        let index_file = yew_folder.join("index.html");

        let index_file = NamedFile::open_async(index_file).await?;

        let res = index_file.into_response(req.request());
        Ok(req.into_response(res))
    };

    // This is only really for home page (/yew/)
    // Use custom files listing renderer to always show the Yew app
    let dir_renderer = |dir: &Directory, req: &HttpRequest| {
        // /yew will show 404 in yew app
        // TODO: if path is /yew, redir to /yew/
        // but unforunately have no way to know if it's /yew,
        // - if url is /yew or /yew/,
        // `req` gives /yew
        // and `dir` doesn't help

        let index_file = NamedFile::open(dir.path.join("index.html"))?;
        let res = index_file.into_response(req);
        Ok(ServiceResponse::new(req.clone(), res))
    };

    Files::new(mount_path, serve_from)
        // .index_file("index.html")
        // .redirect_to_slash_directory()
        .prefer_utf8(true)
        .show_files_listing()
        .files_listing_renderer(dir_renderer)
        .default_handler(fn_service(default_handler))
}

pub trait YewService {
    fn yew_service(&mut self, yew_folder: impl Into<PathBuf>) -> &mut Self;
}
impl YewService for ServiceConfig {
    fn yew_service(&mut self, yew_folder: impl Into<PathBuf>) -> &mut Self {
        self.service(
            web::scope("/yew")
                .wrap(middleware::logger())
                .service(yew_app("/", yew_folder)),
        )
    }
}
