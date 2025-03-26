use serde::{Deserialize, Serialize};
use owo_colors::OwoColorize;
use std::{cmp::min, collections::HashMap, string::String};
use termion::terminal_size;
use textwrap::wrap;

const MIN_TERM_WIDTH_FOR_BOX: usize = 40;
const BOX_PADDING: usize = 2;
const PORT_TITLE_GAP: usize = 2;
const MARGIN: usize = 2;
const DESC_MAX_WIDTH: usize = 100;

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
  pub fn get_port(&self) -> u16 {   // public method to access private field
    self.port
  }

  pub fn pretty_print(&self)  {
    let port_width = self.port.to_string().len() + 2;
    let (term_width, _) = terminal_size()
      .map(|(w, h)| (w as usize, h as usize))
      .unwrap_or((80, 24));

    let use_box = term_width > MIN_TERM_WIDTH_FOR_BOX;
    let max_title_width = term_width.saturating_sub( if use_box { BOX_PADDING + port_width + PORT_TITLE_GAP + BOX_PADDING } else { port_width + PORT_TITLE_GAP } );

    self.pretty_print_title(max_title_width, port_width, use_box);
    self.pretty_print_description(min(term_width, DESC_MAX_WIDTH));
    // self.pretty_print_protocols(term_width);
    self.pretty_print_links(term_width);
  }

  fn pretty_print_title(&self, max_text_width: usize, port_width: usize, use_box: bool) {
    let port = format!("[{}]", self.port);
    let wrapped_lines = wrap(self.title.as_str(), max_text_width);
    let text_width = wrapped_lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let padded_lines: Vec<String> = wrapped_lines.into_iter().map(|line| format!("{:<width$}", line, width = text_width)).collect();
    let box_width = BOX_PADDING + port_width + PORT_TITLE_GAP + text_width + BOX_PADDING;

    if use_box {
      println!("{}", format!("┌{}┐", "─".repeat(box_width)).blue());
    }

    let start = if use_box { format!("│{:<width$}", "", width = BOX_PADDING) } else { String::new() };
    let end = if use_box { format!("{:<width$}│", "", width = BOX_PADDING) } else { String::new() };
    let gap = " ".repeat(PORT_TITLE_GAP);

    for (i, title_line) in padded_lines.iter().enumerate() {
      if i == 0 {
        println!("{}{}{}{}{}", start.blue(), port.purple(), gap, title_line.cyan(), end.blue())
      } else {
        println!("{}{}{}{}{}", start.blue(), " ".repeat(port.len()), gap, title_line.cyan(), end.blue())
      }
    }

    if use_box {
      println!("{}", format!("└{}┘", "─".repeat(box_width)).blue());
    }
  }

  fn pretty_print_description(&self, line_width: usize) {
    wrap(self.desc.as_str(), line_width)
      .iter().for_each(|line| println!("{line}"))
  }

  /*fn pretty_print_protocols(&self, term_width: usize) {
    println!("\n{}", "Protocols".to_string().yellow());
  }*/

  fn pretty_print_links(&self, max_width: usize) {
    println!("\n{}", "More Info".to_string().yellow());

    if let Some(url) = &self.wiki_link {
      let max_link = max_width - (MARGIN * 2 + 6 + 3);
      let link_display = if url.len() > max_link { format!("{}...", &url[..max_link]) } else { url.clone() };

      println!("{:<margin$}Wiki: {}", "", hyperlink(&link_display, url).blue(), margin = MARGIN);
    }

    if let Some(url) = &self.wiki_link {
      let max_link = max_width - (MARGIN * 2 + 6 + 3);
      let link_display = if url.len() > max_link { format!("{}...", &url[..max_link]) } else { url.clone() };

      println!("{:<margin$}RFC: {}", "", hyperlink(&link_display, url).blue(), margin = MARGIN);
    }
  }
}

fn hyperlink(text: &str, url: &str) -> String {
  format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\")
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PortLookup {
  port_map: HashMap<u16, PortInfo>
}

