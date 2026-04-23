mod attr;
mod codegen;
mod parse;
mod types;
mod util;

use attr::AddrMapAttr;
use parse::parse_entries;
use proc_macro::TokenStream;
use std::path::PathBuf;
use syn::{Fields, ItemStruct};

use crate::codegen::generate;

#[proc_macro_attribute]
pub fn addr_map(attr: TokenStream, item: TokenStream) -> TokenStream {
    let AddrMapAttr {
        path, default_abi, ..
    } = syn::parse_macro_input!(attr as AddrMapAttr);
    let ItemStruct {
        ident: struct_name,
        vis,
        attrs,
        fields,
        ..
    } = syn::parse_macro_input!(item as ItemStruct);
    let existing_fields: Vec<syn::Field> = match &fields {
        Fields::Named(f) => f.named.iter().cloned().collect(),
        Fields::Unit => vec![],
        Fields::Unnamed(_) => panic!("addr_map: tuple structs are not supported"),
    };
    let default_abi = default_abi.map(|x| x.value());

    let full_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(path.value());
    let content = std::fs::read_to_string(&full_path).unwrap_or_else(|e| panic!("read error: {e}"));
    let entries = parse_entries(&content, default_abi.as_deref());

    generate(
        &attrs,
        &vis,
        &struct_name,
        &existing_fields,
        entries,
        &content,
    )
    .into()
}
