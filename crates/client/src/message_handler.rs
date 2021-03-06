use crate::ctx::ClientContext;
use crate::session_ctx::SessionContext;

use anyhow::Result;
use maud::html;
use rustimate_core::profile::UserProfile;
use rustimate_core::session::EstimateSession;
use rustimate_core::ResponseMessage;
use std::sync::RwLock;
use uuid::Uuid;

pub(crate) struct MessageHandler {}

impl MessageHandler {
  pub(crate) fn handle(ctx: &RwLock<ClientContext>, msg: ResponseMessage) -> Result<()> {
    match msg {
      ResponseMessage::Connected {
        connection_id,
        user_id,
        u,
        b
      } => on_connected(ctx, connection_id, user_id, &u, b),
      ResponseMessage::Pong { v } => on_pong(ctx, v),
      ResponseMessage::Notification { level, content } => crate::logging::notify(&level, &content),

      // Custom Messages
      ResponseMessage::SessionJoined {
        session,
        members,
        connected,
        polls,
        votes
      } => on_session_joined(ctx, SessionContext::new(*session, &members, &connected, &polls, &votes)),

      ResponseMessage::UpdateMember { member } => crate::members::on_update_member(ctx, member),
      ResponseMessage::UpdateSession { session } => on_update_session(ctx, session),
      ResponseMessage::UpdatePoll { poll } => crate::polls::on_update_poll(ctx, poll),
      ResponseMessage::UpdateVote { vote } => crate::votes::on_update_vote(ctx, vote),

      _ => {
        warn!("Unhandled ResponseMessage [{:?}]", msg);
        Ok(())
      }
    }
  }
}

fn on_connected(ctx: &RwLock<ClientContext>, connection_id: Uuid, user_id: Uuid, u: &UserProfile, b: bool) -> Result<()> {
  ctx
    .write()
    .expect("Cannot lock ClientContext for write")
    .on_connected(connection_id, user_id, u.clone(), b);
  let c = ctx.read().expect("Cannot lock ClientContext for read");
  let _ = c.append_template(
    "socket-results",
    "div",
    html!((format!("Connect message received, {} connection", if b { "binary" } else { "text" })))
  )?;
  Ok(())
}

fn on_pong(ctx: &RwLock<ClientContext>, v: i64) -> Result<()> {
  let now = js_sys::Date::now() as i64;
  let msg = format!("Pong: [{}ms] elapsed", now - v);
  let _ = ctx
    .read()
    .expect("Cannot lock ClientContext for read")
    .append_template("socket-results", "div", html!((msg)))?;
  info!("{}", msg);
  Ok(())
}

fn on_session_joined(ctx: &RwLock<ClientContext>, session: SessionContext) -> Result<()> {
  {
    let svc = ctx.read().expect("Cannot lock ClientContext for read");
    crate::members::render_members(&svc, session.members_sorted(), session.connected())?;
    crate::polls::render_polls(&svc, session.polls())?;
  }
  {
    let mut svc = ctx.write().expect("Cannot lock ClientContext for write");
    svc.on_session_joined(session);
  }
  Ok(())
}

fn on_update_session(ctx: &RwLock<ClientContext>, session: EstimateSession) -> Result<()> {
  {
    let svc = ctx.read().expect("Cannot lock ClientContext for read");
    svc.replace_template("session-name-label", html!((session.title())))?;
  }
  {
    let mut svc = ctx.write().expect("Cannot lock ClientContext for write");
    if let Some(ref mut x) = svc.session_ctx_mut() {
      x.set_session(session);
    }
  }
  Ok(())
}
