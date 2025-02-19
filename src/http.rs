use crate::db::Db;
use crate::owntracks::Message;
use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    u: Option<String>,
    d: Option<String>,
}

#[post("/owntracks")]
async fn owntracks(
    db: web::Data<Db>,
    msg: web::Json<Message>,
    params: web::Query<Params>,
) -> actix_web::Result<impl Responder> {
    log::debug!("{msg:?}");
    if let Message::Location(loc) = msg.into_inner() {
        let _user = &params.u;
        let _device = &params.d;
        // TODO: read X-Limit-U + X-Limit-D headers
        if let Err(e) = db.insert_location(&loc) {
            log::error!("{e}");
        }
    }
    Ok(web::Json::<Vec<Message>>(Vec::new()))
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
    })
    .bind(bind_addr)?
    .run()
    .await
}
