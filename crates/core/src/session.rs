use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SessionStatus {
  Creating,
  Active,
  Complete
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EstimateSession {
  key: String,
  title: String,
  choices: Vec<String>,
  options: HashMap<String, String>,
  status: SessionStatus
}

impl EstimateSession {
  pub fn new(title: String) -> Self {
    Self {
      key: slug_for(&title),
      title,
      choices: default_choices(),
      options: HashMap::new(),
      status: SessionStatus::Creating
    }
  }

  pub const fn key(&self) -> &String {
    &self.key
  }

  pub const fn title(&self) -> &String {
    &self.title
  }

  pub fn set_title(&mut self, title: String) {
    self.title = title;
  }

  pub const fn choices(&self) -> &Vec<String> {
    &self.choices
  }

  pub fn set_choices(&mut self, choices: Vec<String>) {
    self.choices = choices;
  }
}

fn slug_for(title: &str) -> String {
  // slugify soon!
  title.into()
}

fn default_choices() -> Vec<String> {
  vec![
    "0".into(),
    "0.5".into(),
    "1".into(),
    "2".into(),
    "3".into(),
    "5".into(),
    "8".into(),
    "13".into(),
    "20".into(),
    "40".into(),
    "100".into(),
    "?".into(),
  ]
}
