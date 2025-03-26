use std::{env, string::String, collections::HashMap};

mod portinfo;

use portinfo::{PortInfo, Verbosity};

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
    .map(|p| (p.get_port(), p))
    .collect();

  parse_ports(&args[1]).iter().for_each(|port| {
    if let Some(protocol) = protocol_map.get(port) {
      protocol.pretty_print(Verbosity::Verbose);
      println!("\n");
    }
  });
}