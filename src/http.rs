use crate::db::Db;
use crate::geojson;
use crate::gpx;
use crate::owntracks::Message;
use actix_cors::Cors;
use actix_web::{
    error, get, middleware, middleware::Logger, post, route, web, App, HttpResponse, HttpServer,
    Responder,
};
use actix_web_rust_embed_responder::{EmbedResponse, EmbedableFileResponse, IntoResponse};
use rust_embed_for_web::RustEmbed;
use serde::Deserialize;

#[derive(Deserialize)]
struct OtParams {
    u: Option<String>,
    d: Option<String>,
}

/// OwnTracks endpoint for storing locations
#[post("/owntracks")]
async fn owntracks(
    db: web::Data<Db>,
    msg: web::Json<Message>,
    params: web::Query<OtParams>,
) -> actix_web::Result<impl Responder> {
    log::debug!("{msg:?}");
    if let Message::Location(loc) = msg.into_inner() {
        let user = params.u.clone().unwrap_or("".to_string());
        let device = params.d.clone().unwrap_or("".to_string());
        // TODO: read user/device from msg.topic and/or from X-Limit-U + X-Limit-D headers
        if let Err(e) = db.insert_location(&user, &device, &loc).await {
            log::error!("{e}");
        }
    }
    Ok(web::Json::<Vec<Message>>(Vec::new()))
}

#[derive(Deserialize)]
struct TracksParams {
    date: String,
}

#[get("/trackinfos")]
async fn trackinfos(
    db: web::Data<Db>,
    params: web::Query<TracksParams>,
) -> actix_web::Result<impl Responder> {
    match db.query_tracks_info(&params.date).await {
        Ok(track_infos) => Ok(web::Json(track_infos)),
        Err(e) => {
            log::error!("{e}");
            Err(error::ErrorInternalServerError(
                "Failed to fetch track infos",
            ))
        }
    }
}

/// Get GPX tracks
#[get("/gpxtracks")]
async fn gpxtracks(
    db: web::Data<Db>,
    params: web::Query<TracksParams>,
) -> actix_web::Result<String> {
    let tracks_ = db.query_tracks(&params.date).await.unwrap();
    match gpx::query_tracks(&tracks_) {
        Ok(gpx) => Ok(gpx),
        Err(e) => {
            log::error!("{e}");
            Err(error::ErrorInternalServerError("Failed to fetch tracks"))
        }
    }
}

/// Get GeoJSON tracks
#[get("/tracks")]
async fn tracks(db: web::Data<Db>, params: web::Query<TracksParams>) -> HttpResponse {
    let tracks_ = db.query_tracks(&params.date).await.unwrap();
    let json = match geojson::query_tracks(&tracks_) {
        Ok(json) => json,
        Err(e) => {
            log::error!("{e}");
            return HttpResponse::InternalServerError()
                .reason("Failed to fetch tracks")
                .finish();
        }
    };
    HttpResponse::Ok()
        .content_type("application/geo+json")
        .body(json)
}

#[derive(RustEmbed)]
#[folder = "./static/"]
struct Embed;

// This responder implements both GET and HEAD
#[route("/{path:.*}", method = "GET", method = "HEAD")]
async fn serve_assets(path: web::Path<String>) -> EmbedResponse<EmbedableFileResponse> {
    let path = if path.is_empty() {
        "index.html"
    } else {
        path.as_str()
    };
    Embed::get(path).into_response()
}

pub async fn webserver(db: Db) -> std::io::Result<()> {
    let bind_addr = dotenvy::var("HTTP_LISTEN").unwrap_or("127.0.0.1:8083".to_string());
    log::info!("Listening on http://{bind_addr}/");
    HttpServer::new(move || {
        let cors = if cfg!(debug_assertions) {
            Cors::permissive()
        } else {
            Cors::default()
        };
        App::new()
            .wrap(Logger::default().log_target("owntracks_duck_tape::http"))
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(owntracks)
            .service(trackinfos)
            .service(gpxtracks)
            .service(tracks)
            .service(serve_assets)
    })
    .bind(bind_addr)?
    .run()
    .await
}
