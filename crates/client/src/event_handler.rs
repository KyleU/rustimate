use crate::ctx::ClientContext;
use rustimate_core::{RequestMessage, Result};

pub(crate) struct EventHandler {}

impl EventHandler {
  pub(crate) fn handle(ctx: &ClientContext, t: &str, k: &str, v: &str) -> Result<()> {
    match t {
      "send-ping" => ctx.send(RequestMessage::Ping {
        v: js_sys::Date::now() as i64
      }),

      "add-poll" if v.is_empty() => crate::js::notify("warn", "Enter a question next time"),
      "add-poll" => ctx.send(RequestMessage::AddPoll { q: v.into() }),

      _ => warn!("Unhandled event [{}] with [k:{}], [v:{}]", t, k, v)
    }
    Ok(())
  }
}
