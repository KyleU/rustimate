use anyhow::Result;
use wasm_bindgen::JsCast;

impl crate::ctx::ClientContext {
  pub(crate) fn get_element_by_id(&self, id: &str) -> Result<web_sys::HtmlElement> {
    self.get_element_by_id_as::<web_sys::HtmlElement>(id)
  }

  pub(crate) fn get_element_by_id_as<T: JsCast>(&self, id: &str) -> Result<T> {
    match self.document().get_element_by_id(id) {
      Some(e) => match e.dyn_into::<T>() {
        Ok(el) => Ok(el),
        Err(_) => Err(anyhow::anyhow!(format!("Cannot load html element with id [{}]", id)))
      },
      None => Err(anyhow::anyhow!(format!("Cannot load element with id [{}]", id)))
    }
  }

  pub(crate) fn create_element<T: JsCast>(&self, tag: &str) -> Result<T> {
    match self.document().create_element(tag) {
      Ok(el) => el
        .dyn_into::<T>()
        .map_err(|_| anyhow::anyhow!(format!("Cannot cast [{}] element", tag))),
      Err(_) => Err(anyhow::anyhow!(format!("Cannot create [{}] element", tag)))
    }
  }

  pub(crate) fn append_template(&self, id: &str, tag: &str, template: maud::Markup) -> Result<web_sys::HtmlElement> {
    let parent = self.get_element_by_id(id)?;
    let el = self.create_element::<web_sys::HtmlElement>(tag)?;
    el.set_inner_html(&template.into_string());
    parent
      .append_child(&el)
      .map_err(|_| anyhow::anyhow!(format!("Cannot load html element with id [{}]", id)))
      .map(|_| el)
  }

  pub(crate) fn replace_template(&self, id: &str, template: maud::Markup) -> Result<()> {
    let parent = self.get_element_by_id(id)?;
    parent.set_inner_html(&template.into_string());
    Ok(())
  }

  pub(crate) fn set_visible(&self, id: &str, v: bool) -> Result<()> {
    let el = self.get_element_by_id(id)?;
    let style = el.style();
    let _ = style.set_property("display", if v { "block" } else { "none" });
    Ok(())
  }

  pub(crate) fn set_input_value(&self, id: &str, v: &str) -> Result<()> {
    self.get_element_by_id_as::<web_sys::HtmlInputElement>(id)?.set_value(v);
    Ok(())
  }

  pub(crate) fn get_input_value(&self, id: &str) -> Result<String> {
    Ok(self.get_element_by_id_as::<web_sys::HtmlInputElement>(id)?.value())
  }
}

pub(crate) fn onclick_event(t: &str, k: &str, v: &str) -> String {
  format!(
    "rustimate.on_event('{}', '{}', {});return false;",
    t,
    k,
    if v.is_empty() { "''" } else { v }
  )
}
