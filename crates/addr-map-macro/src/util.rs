pub fn parse_hex(s: &str) -> usize {
    let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    usize::from_str_radix(s, 16).unwrap_or_else(|e| panic!("hex parse error '{s}': {e}"))
}

pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}
