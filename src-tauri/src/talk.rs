#[cfg(not(test))]
use crate::script::data::{shop_items_data, talk::Talk};

#[cfg(not(test))]
#[allow(unused)]
mod dataset;
#[cfg(not(test))]
#[allow(unused)]
mod randomizer;
#[cfg(not(test))]
#[allow(unused)]
mod script;

#[cfg(not(test))]
fn main() {
    let a = std::env::args().nth(1).unwrap();
    let talk = Talk::from_bytes(parse_bytes(&a));
    let sid = shop_items_data::parse(&talk).unwrap();

    println!("{:#?}", sid);
}

#[cfg(not(test))]
fn parse_bytes(input: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            let mut num = String::new();

            while let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    num.push(next);
                    chars.next();
                } else {
                    break;
                }
            }

            if !num.is_empty() {
                let value: u16 = num.parse().unwrap();
                result.push(value as u8);
            }
        }
    }

    result
}
