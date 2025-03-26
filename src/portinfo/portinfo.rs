use serde::{Deserialize, Serialize};
use owo_colors::OwoColorize;
use std::{cmp::min, collections::HashMap, string::String};
use termion::terminal_size;
use textwrap::wrap;

#[derive(Debug, Deserialize, Serialize)]
pub struct PortInfo {
  port: u16,
  title: String,
  desc: String,
  layer4: Vec<Layer4Info>,
  wiki_link: Option<String>,
  rfi_link: Option<String>,
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