use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::token::Pub;
use syn::{Attribute, Field, Ident, Visibility};

use crate::types::{Entry, Function, SimpleEntry};
use crate::util::to_pascal_case;

pub fn generate(
    attrs: &[Attribute],
    vis: &Visibility,
    struct_name: &Ident,
    existing_fields: &[Field],
    entries: Vec<Entry>,
    content: &str,
) -> TokenStream {
    let type_aliases = gen_type_aliases(&entries);
    let struct_ = gen_struct_with_constructor(attrs, vis, struct_name, existing_fields, &entries);
    let (group_structs, new_methods) = gen_from_entries(entries);

    quote! {
        #(#type_aliases)*

        #struct_

        impl #struct_name {
            #(#new_methods)*
        }

        #(#group_structs)*

        const _: &str = #content;
    }
}

fn gen_type_aliases(entries: &[Entry]) -> Vec<TokenStream> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            Entry::Simple(SimpleEntry::StaticFnPtr(s)) => Some(s),
            _ => None,
        })
        .map(|s| {
            let alias_name = make_ident(&s.to_type_name());
            let fn_ty = &s.fn_ty;
            quote! { pub type #alias_name = #fn_ty; }
        })
        .collect()
}

fn gen_struct_with_constructor(
    attrs: &[Attribute],
    vis: &Visibility,
    struct_name: &Ident,
    existing_fields: &[Field],
    entries: &[Entry],
) -> TokenStream {
    let new_fields = gen_fields_from_entries(entries);

    let new_inits = gen_inits_from_entries(entries);
    let existing_inits = gen_existing_inits(existing_fields);

    quote! {
        #(#attrs)*
        #[derive(Debug, Clone)]
        #vis struct #struct_name {
            #(#existing_fields,)*
            #(#new_fields),*
        }

        impl #struct_name {
            pub const fn new(base_addr: usize) -> Self {
                Self {
                    #(#new_inits),*
                    #(#existing_inits),*
                }
            }
        }
    }
}

fn gen_inits_from_entries(entries: &[Entry]) -> Vec<TokenStream> {
    entries
        .iter()
        .map(|entry| match entry {
            Entry::Simple(ty) => gen_init_of_simple(ty),
            Entry::Nested {
                entrypoint,
                children: _,
            } => gen_init_of_nested(entrypoint),
        })
        .collect()
}

fn gen_init_of_simple(ty: &SimpleEntry) -> TokenStream {
    let ident = make_ident(ty.name());
    let offset = ty.offset();
    let type_tokens = addr_type_to_tokens(ty);

    match ty {
        SimpleEntry::Function(_) | SimpleEntry::Label(_) => {
            quote! { #ident: (base_addr + #offset) as #type_tokens }
        }
        SimpleEntry::Static(_) | SimpleEntry::StaticFnPtr(_) => {
            quote! { #ident: std::ptr::NonNull::new((base_addr + #offset) as *mut _).unwrap() }
        }
    }
}

fn gen_init_of_nested(base: &Function) -> TokenStream {
    let struct_name = make_ident(&to_pascal_case(&base.name));
    let field_ident = make_ident(&base.name);
    let base_offset = base.offset;
    quote! {
        #field_ident: #struct_name::new(base_addr +#base_offset)
    }
}

fn addr_type_to_tokens(ty: &SimpleEntry) -> TokenStream {
    match ty {
        SimpleEntry::Label(..) => quote! { *const () },
        SimpleEntry::Function(_) => quote! { *const () }, // 関数ポインターを定数で生成することはできないため、型不明のポインターで保持する
        SimpleEntry::Static(t) => {
            let ty_tokens = &t.ty;
            quote! { std::ptr::NonNull<#ty_tokens> }
        }
        SimpleEntry::StaticFnPtr(s) => {
            let alias_name = make_ident(&s.to_type_name());
            quote! { std::ptr::NonNull<*const #alias_name> }
        }
    }
}

fn gen_method_for_simple(ty: &SimpleEntry) -> Option<TokenStream> {
    let ident = make_ident(ty.name());

    match ty {
        SimpleEntry::Function(sig) => Some(gen_fn_method(&ident, &sig.ty, sig.comment.as_deref())),
        SimpleEntry::Label(_) | SimpleEntry::Static(_) | SimpleEntry::StaticFnPtr(_) => None,
    }
}

// gen_doc_comment ヘルパーを追加
fn gen_doc_comment(comment: Option<&str>) -> TokenStream {
    match comment {
        None => quote! {},
        Some(c) => {
            let lines: Vec<TokenStream> = c.lines().map(|l| quote! { #[doc = #l] }).collect();
            quote! { #(#lines)* }
        }
    }
}

fn gen_fn_method(ident: &Ident, f: &syn::TypeBareFn, comment: Option<&str>) -> TokenStream {
    let doc = gen_doc_comment(comment);
    let inputs = f.inputs.iter().enumerate().map(|(i, arg)| {
        let ty = &arg.ty;
        let name = make_ident(&format!("arg{i}"));
        quote! { #name: #ty }
    });

    let arg_names = (0..f.inputs.len()).map(|i| {
        let name = make_ident(&format!("arg{i}"));
        quote! { #name }
    });

    let output = &f.output;

    quote! {
        #doc
        pub unsafe fn #ident(&self, #(#inputs),*) #output {
            let f: #f = std::mem::transmute(self.#ident);
            f(#(#arg_names),*)
        }
    }
}

fn gen_struct_with_constructor_from_nested(
    mut base: Function,
    children: Vec<SimpleEntry>,
) -> TokenStream {
    let struct_name = make_ident(&to_pascal_case(&base.name));
    base.offset = 0;
    base.name = "entrypoint".into();

    let fields: Vec<_> = [SimpleEntry::Function(base)]
        .into_iter()
        .chain(children)
        .map(Entry::Simple)
        .collect();

    let vis = Visibility::Public(Pub::default());
    gen_struct_with_constructor(&[], &vis, &struct_name, &[], &fields)
}

pub fn gen_existing_inits(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap();
            quote! { #ident: Default::default() }
        })
        .collect()
}

fn gen_from_entries(entries: Vec<Entry>) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut new_methods = vec![];
    let mut group_structs = vec![];
    entries
        .into_iter()
        .map(|entry| match entry {
            Entry::Simple(ty) => {
                let m = gen_method_for_simple(&ty);
                (m, None)
            }
            Entry::Nested {
                entrypoint,
                children,
            } => {
                let def = gen_struct_with_constructor_from_nested(entrypoint, children);
                (None, Some(def))
            }
        })
        .for_each(|(m, def)| {
            if let Some(m) = m {
                new_methods.push(m);
            }
            if let Some(def) = def {
                group_structs.push(def);
            }
        });
    (group_structs, new_methods)
}

fn gen_fields_from_entries(entries: &[Entry]) -> Vec<TokenStream> {
    entries
        .iter()
        .map(|entry| match entry {
            Entry::Simple(SimpleEntry::Function(entry)) => {
                // 関数はメソッド側にdocを付けるのでフィールドにはつけない
                // 関数ポインターは定数で生成することはできないため、型不明のポインターで保持する
                (None, entry.name.as_str(), quote! { *const () })
            }
            Entry::Simple(SimpleEntry::StaticFnPtr(entry)) => {
                let ty = addr_type_to_tokens(&SimpleEntry::StaticFnPtr(entry.clone()));
                (entry.comment.as_deref(), entry.name.as_str(), ty)
            }
            Entry::Simple(entry) => {
                let ty = addr_type_to_tokens(entry);
                (entry.comment(), entry.name(), ty)
            }
            Entry::Nested {
                entrypoint,
                children: _,
            } => {
                let struct_name = make_ident(&to_pascal_case(&entrypoint.name));
                (None, entrypoint.name.as_str(), quote! { #struct_name })
            }
        })
        .map(|(doc, name, ty)| {
            let doc = gen_doc_comment(doc);
            let name = make_ident(name);
            quote! {
                #doc
                pub #name: #ty
            }
        })
        .collect()
}

fn make_ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}
