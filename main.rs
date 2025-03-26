use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{env, cmp::min, string::String, collections::HashMap};
use termion::terminal_size;
use textwrap::wrap;

const MIN_TERM_WIDTH_FOR_BOX: usize = 40;
const BOX_PADDING: usize = 2;
const PORT_TITLE_GAP: usize = 2;
const MARGIN: usize = 2;
const DESC_MAX_WIDTH: usize = 100;

#[derive(Debug, Deserialize, Serialize)]
struct PortInfo {
  port: u16,
  title: String,
  desc: String,
  wiki_link: Option<String>,
  rfi_link: Option<String>,
}

impl PortInfo {
  fn pretty_print(&self)  {
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

fn parse_ports(input: &str) -> Vec<u16> {
  let mut ports = Vec::new();

  for part in input.split(',') {
    if let Some((start, end)) = part.split_once('-') {
      if let (Ok(start), Ok(end)) = (start.parse::<u16>(), end.parse::<u16>()) {
        ports.extend(start..=end);
      }
    } else if let Ok(port) = part.parse::<u16>() {
      ports.push(port);
    }
  }

  ports
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: portinfo <ports>\n\n{}\n\t{}\n\t{}\n\t{}\n\t{}",
      "Examples:",
      "portinfo 80",
      "portinfo 21,53",
      "portinfo 318-320",
      "portinfo 80,443-445,92"
    );
    return;
  }

  let json_data = include_str!("assets/ports.json");
  let protocols: Vec<PortInfo> = serde_json::from_str(json_data).expect("Invalid JSON");

  let protocol_map: HashMap<u16, PortInfo> = protocols
    .into_iter()
    .map(|p| (p.port, p))
    .collect();

  parse_ports(&args[1]).iter().for_each(|port| {
    if let Some(protocol) = protocol_map.get(port) {
      protocol.pretty_print();
      println!("\n");
    }
  });
}