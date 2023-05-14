use actix_files as afs;
use actix_http::Response;
use actix_web::{
    dev::HttpResponseBuilder, dev::ServiceResponse, http::StatusCode, middleware, web, App,
    HttpRequest, HttpServer, Result,
};

use shared::article_list::Articles;
use shared::constants;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const PREVIEW_LINES: i8 = 9;

// TODO: Cache in-memoy articles: Redis? Embedded KV store?

// TODO implement own list_articles so we can do it async and use tokio::fs instead of blocking the
// thread. THIS HERE NOW IS A BAD IDEA!!!!!!
fn list_articles(
    dir: &afs::Directory,
    req: &HttpRequest,
) -> Result<ServiceResponse, std::io::Error> {
    let mut articles: Vec<_> = fs::read_dir(&dir.path)?.collect();
    articles.sort_by(|a, b| {
        a.as_ref()
            .unwrap()
            .file_name()
            .cmp(&b.as_ref().unwrap().file_name())
    });

    let articles: Vec<_> = articles
        .iter()
        .filter(|f| !f.as_ref().unwrap().file_type().unwrap().is_dir())
        .map(|res| res.as_ref().unwrap().file_name().into_string().unwrap())
        .collect();

    Ok(ServiceResponse::new(
        req.clone(),
        HttpResponseBuilder::new(StatusCode::OK).json(Articles { articles }),
    ))
}

async fn preview(req: HttpRequest) -> Result<Response> {
    let base_path = PathBuf::from(format!(
        "{}/{}/",
        constants::STATIC_URL,
        constants::ARTICLES_PATH
    ));
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = base_path.join(path);
    let file = afs::NamedFile::open(path)?;
    let res_buf = BufReader::new(&*file);
    let mut iter = 0;
    let mut buf = "".to_string();

    for line in res_buf.lines() {
        if iter <= PREVIEW_LINES {
            buf += &line?;
            buf += "\n";
            iter += 1;
        }
    }

    let mut res = Response::build_from(file.into_response(&req).unwrap());
    let res = res.body(buf);
    Ok(res)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    main_2().await.unwrap()
    /*let ip = if cfg!(debug_assertions) {
        "127.0.0.1"
    } else {
        "0.0.0.0"
    };
    let binding_ip = format!(
        "{}:{}",
        ip,
        std::env::var("PORT").unwrap_or_else(|_| "8080".to_string())
    );

    println!("Will attemp to listen in http://{}/", binding_ip);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(
                afs::Files::new(
                    constants::ARTICLE_LIST_URI,
                    format!("{}/{}", constants::STATIC_URL, constants::ARTICLES_PATH),
                )
                .files_listing_renderer(list_articles)
                .show_files_listing(),
            )
            .route("/preview/articles/{filename:.*}", web::get().to(preview))
            .service(afs::Files::new("/", constants::STATIC_URL).index_file("index.html"))
    })
    .bind(binding_ip)?
    .run()
    .await
    */
}

async fn list_files(
    path: impl AsRef<std::path::Path>,
) -> Result<tokio::fs::ReadDir, std::io::Error> {
    tokio::fs::read_dir(path).await
}

use axum::{route, routing::RoutingDsl, service::ServiceExt};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
async fn main_2() -> hyper::Result<()> {
    tracing_subscriber::fmt::init();
    let app = route(constants::ARTICLE_LIST_URI)
        .route(
            "/",
            axum::service::get(
                ServeDir::new(constants::STATIC_URL)
                    .append_index_html_on_directories(true)
                    .handle_error(|error: std::io::Error| {
                        Ok::<_, std::convert::Infallible>((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }),
            ),
        )
        .layer(TraceLayer::new_for_http());
    // DEBUG!
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::server::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}
