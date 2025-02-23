use crate::db::Db;
use crate::gpx;
use crate::owntracks::Message;
use actix_web::middleware::Logger;
use actix_web::{error, get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct OtParams {
    u: Option<String>,
    d: Option<String>,
}

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
        if let Err(e) = db.insert_location(&user, &device, &loc) {
            log::error!("{e}");
        }
    }
    Ok(web::Json::<Vec<Message>>(Vec::new()))
}

#[derive(Deserialize)]
struct TracksParams {
    date: String,
}

#[get("/tracks")]
async fn tracks(db: web::Data<Db>, params: web::Query<TracksParams>) -> actix_web::Result<String> {
    match gpx::query_tracks(&db, &params.date) {
        Ok(gpx) => Ok(gpx),
        Err(e) => {
            log::error!("{e}");
            Err(error::ErrorBadRequest("DB error"))
        }
    }
}

#[actix_web::main]
pub async fn webserver(db: Db) -> std::io::Result<()> {
    let bind_addr = dotenvy::var("HTTP_LISTEN").unwrap_or("127.0.0.1:8083".to_string());
    log::info!("Listening on http://{bind_addr}/owntracks");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(owntracks)
            .service(tracks)
    })
    .bind(bind_addr)?
    .run()
    .await
}
