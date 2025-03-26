extern crate proc_macro;
use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use serde_json;

use portinfo::{ PortInfo, PortLookup };

#[proc_macro]
pub fn compiletime_deserialise(input: TokenStream) -> TokenStream {
  let json_data = include_str!("assets/ports.json");
  let ports: Vec<PortInfo> = serde_json::from_str(json_data).expect("Invalid JSON");

  let port_map: HashMap<u16, PortInfo> = ports
    .into_iter()
    .map(|p| (p.get_port(), p))
    .collect();

  let generated = quote! {
    pub const PORT_LOOKUP: &'static [PortLookup] = &[#( PortLookup { port_map: #port_map } ),*];
  };

  generated.into()
}