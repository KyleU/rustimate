use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::{Data, Path};

use rustimate_core::ResponseMessage;
use rustimate_service::AppConfig;

/// Available at `/admin`
pub fn list(session: Session, cfg: Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    rustimate_templates::admin::list(&ctx, router)
  })
}

/// Available at `/admin/conn`
pub fn connections(session: Session, cfg: Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    let conn = ctx.app().connections().read().unwrap();
    rustimate_templates::admin::connections(&ctx, router, conn.conn_list(), conn.channel_list())
  })
}

/// Available at `/admin/conn/{id}`
pub fn connection_detail(session: Session, cfg: Data<AppConfig>, id: Path<uuid::Uuid>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    rustimate_templates::admin::connection_detail(&ctx, router, *id)
  })
}

/// Available by posting to `/admin/conn/{id}`
pub fn connection_send(session: Session, cfg: Data<AppConfig>, id: Path<uuid::Uuid>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    let conn = ctx.app().connections().read().unwrap();
    conn.send_session(&id, ResponseMessage::ServerError { reason: "broadcast".into(), content: "TODO".into()});
    rustimate_templates::admin::connection_detail(&ctx, router, *id)
  })
}

/// Available at `/admin/settings`
pub fn settings(session: Session, cfg: Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| rustimate_templates::settings::settings(&ctx, router))
}

/// Available by posting to `/admin/settings`
pub fn settings_post(session: Session, cfg: Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| rustimate_templates::settings::settings(&ctx, router))
}
