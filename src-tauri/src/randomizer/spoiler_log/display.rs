use std::fmt;

use super::{Line, SpoilerLog};

/// 同じ shop の枠をまとめた 1 行
struct ShopGroup<'a> {
    shop: &'a str,
    items: Vec<(usize, &'a str)>,
}

impl fmt::Display for ShopGroup<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let item = |idx| {
            self.items
                .iter()
                .find(|(x, _)| *x == idx)
                .map_or("_", |(_, item)| item)
        };
        writeln!(f, "{} = {}, {}, {}", self.shop, item(0), item(1), item(2))
    }
}

fn fmt_lines(lines: &[Line], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut group: Option<ShopGroup> = None;
    for line in lines {
        let same_shop = match (&group, line) {
            (Some(group), Line::Shop { shop, .. }) => group.shop == shop,
            _ => false,
        };
        if !same_shop && let Some(group) = group.take() {
            write!(f, "{}", group)?;
        }
        match line {
            Line::Spot { spot, item } => writeln!(f, "{} = {}", spot, item)?,
            Line::Shop { shop, idx, item } => group
                .get_or_insert_with(|| ShopGroup {
                    shop,
                    items: Vec::new(),
                })
                .items
                .push((*idx, item)),
        }
    }
    if let Some(group) = group {
        write!(f, "{}", group)?;
    }
    Ok(())
}

impl fmt::Display for SpoilerLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, sphere) in self.progression.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            writeln!(f, "[Sphere {}]", i)?;
            fmt_lines(&sphere.lines, f)?;
            for event in &sphere.events {
                writeln!(f, "{}", event)?;
            }
        }
        writeln!(f)?;
        writeln!(f, "[Maps]")?;
        fmt_lines(&self.maps, f)
    }
}
