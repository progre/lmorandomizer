use std::collections::HashSet;
use syn::Type;
use syn::TypeBareFn;
use toml_edit::{DocumentMut, Item, Table, Value};

use crate::types::{Entry, Function, Label, SimpleEntry, Static, StaticFnPtr};
use crate::util::parse_hex;

/// toml_edit の decor prefix からコメント文字列を抽出する
fn extract_comment(decor_prefix: &str) -> Option<String> {
    let lines: Vec<&str> = decor_prefix
        .lines()
        .filter_map(|l| {
            let t = l.trim();
            t.strip_prefix('#').map(|c| c.trim())
        })
        .collect();
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}

/// キーの前方コメント（prefix）と値の末尾コメント（suffix）を結合して返す
fn extract_comment_for_entry(table: &Table, key: &str) -> Option<String> {
    let prefix_comment = table
        .key(key)
        .and_then(|k| k.leaf_decor().prefix())
        .and_then(|p| p.as_str())
        .and_then(extract_comment);

    let suffix_comment = table
        .get_key_value(key)
        .and_then(|(_, item)| item.as_value())
        .map(|v| v.decor())
        .and_then(|d| d.suffix())
        .and_then(|s| s.as_str())
        .and_then(extract_comment);

    match (prefix_comment, suffix_comment) {
        (Some(a), Some(b)) => Some(format!("{a}\n{b}")),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn parse_var_entry_value(val: &str) -> (String, Type) {
    let val = val.trim();
    let (name, ty) = val
        .split_once(':')
        .unwrap_or_else(|| panic!("invalid variable entry value: {val}"));
    let name = name.trim().to_string();
    let ty = syn::parse_str(ty).unwrap_or_else(|e| panic!("invalid type '{ty}': {e}"));
    (name, ty)
}

fn parse_fn_entry_value(
    val: &str,
    fn_pos: usize,
    default_abi: Option<&str>,
) -> (String, TypeBareFn) {
    let abi = if fn_pos == 0 {
        default_abi
            .map(|x| format!(r#"extern "{x}" "#))
            .unwrap_or_default()
    } else {
        val[..fn_pos].to_owned()
    };
    let after_fn = &val[fn_pos + 3..];
    let Some(paren_pos) = after_fn.find('(') else {
        panic!("invalid function entry value: {val}");
    };
    let name = after_fn[..paren_pos].trim().to_string();
    let ty = format!("{abi}fn{}", &after_fn[paren_pos..]);
    let Ok(syn::Type::BareFn(ty)) = syn::parse_str::<syn::Type>(&ty) else {
        panic!("invalid function type '{ty}'");
    };
    (name, ty)
}

fn parse_text_entry_value(
    offset: usize,
    val: &str,
    default_abi: Option<&str>,
    comment: Option<String>,
) -> SimpleEntry {
    let val = val.trim();
    if let Some(end) = val.strip_prefix("'") {
        return SimpleEntry::Label(Label {
            offset,
            name: end.to_string(),
            comment,
        });
    }
    parse_text_entry_value_as_fn(offset, val, default_abi, comment).into()
}

fn parse_text_entry_value_as_fn(
    offset: usize,
    val: &str,
    default_abi: Option<&str>,
    comment: Option<String>,
) -> Function {
    let fn_pos = val
        .find("fn ")
        .unwrap_or_else(|| panic!("invalid function entry value: {val}"));
    let (name, ty) = parse_fn_entry_value(val, fn_pos, default_abi);
    Function {
        offset,
        name,
        ty,
        comment,
    }
}

fn parse_data_entry_value(offset: usize, val: &str, comment: Option<String>) -> SimpleEntry {
    let (name, ty) = parse_var_entry_value(val.trim());
    if let syn::Type::Ptr(ptr) = &ty
        && let syn::Type::BareFn(bare_fn) = ptr.elem.as_ref()
    {
        return SimpleEntry::StaticFnPtr(StaticFnPtr {
            offset,
            name,
            fn_ty: bare_fn.clone(),
            comment,
        });
    }
    SimpleEntry::Static(Static {
        offset,
        name,
        ty,
        comment,
    })
}

fn parse_group_children(t: &Table, default_abi: Option<&str>) -> (SimpleEntry, Vec<SimpleEntry>) {
    let mut signature = None;
    let children = t
        .iter()
        .filter_map(|(offset_key, item)| {
            // こちらも同様にキーのleaf_decorから取る
            let comment = extract_comment_for_entry(t, offset_key);
            let s = item.as_str().expect("group child must be string");
            if offset_key == "------" {
                signature = Some(parse_text_entry_value(0, s, default_abi, comment));
                return None;
            }
            let offset = parse_hex(offset_key);
            Some(parse_text_entry_value(offset, s, default_abi, comment))
        })
        .collect();
    (signature.expect("group must have a signature"), children)
}

fn collect_table(table: &Table, default_abi: Option<&str>) -> Vec<Entry> {
    let mut result: Vec<_> = table
        .get("data")
        .into_iter()
        .flat_map(|x| x.as_table())
        .flatten()
        .map(|(key, item)| match item {
            Item::Value(Value::String(s)) => {
                let addr = parse_hex(key);
                // コメントはキーのleaf_decorのprefixに確定的に入っている
                let comment = extract_comment_for_entry(table, key);
                Entry::Simple(parse_data_entry_value(addr, s.value(), comment))
            }
            other => panic!("unexpected [data] value type for key '{key}': {other:?}"),
        })
        .collect();

    let mut simples = vec![];
    let mut groups = vec![];
    let text_iter = table
        .get("text")
        .into_iter()
        .flat_map(|x| x.as_table())
        .flatten();
    for (key, item) in text_iter {
        match item {
            Item::Value(Value::String(s)) => {
                let addr = parse_hex(key);
                // コメントはキーのleaf_decorのprefixに確定的に入っている
                let comment = extract_comment_for_entry(table, key);
                let ty = parse_text_entry_value(addr, s.value(), default_abi, comment);
                simples.push(ty);
            }
            Item::Table(t) => {
                let (mut signature, children) = parse_group_children(t, default_abi);
                signature.set_offset(parse_hex(key));
                groups.push(Entry::Nested {
                    signature,
                    children,
                });
            }
            other => panic!("unexpected [text] value type for key '{key}': {other:?}"),
        }
    }

    let group_keys: HashSet<_> = groups.iter().map(|x| x.offset()).collect();
    let simples: Vec<_> = simples
        .into_iter()
        .filter(|entry| !group_keys.contains(&entry.offset()))
        .map(Entry::Simple)
        .collect();
    result.extend(simples);
    result.extend(groups);

    result
}

pub fn parse_entries(content: &str, default_abi: Option<&str>) -> Vec<Entry> {
    let doc: DocumentMut = content
        .parse()
        .unwrap_or_else(|e| panic!("toml parse error: {e}"));
    collect_table(doc.as_table(), default_abi)
}
