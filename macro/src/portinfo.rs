use serde::{Deserialize, Serialize};
use std::{collections::HashMap, string::String};

#[derive(Debug, Deserialize, Serialize)]
pub struct PortInfo {
  port: u16,
  title: String,
  desc: String,
  layer4: Vec<Layer4Info>,
  wiki_link: Option<String>,
  rfc_link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Layer4Info {
  name: String,
  usage: String,
}

impl PortInfo {
  pub fn get_port(&self) -> u16 {
    self.port
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PortLookup {
  port_map: HashMap<u16, PortInfo>
}