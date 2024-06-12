#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub name: String,
    /// 'mainWeapon' | 'subWeapon' | 'equipment' | 'rom' | 'seal'
    pub r#type: String,
    pub number: i8,
    pub count: u8,
    pub flag: i32,
}

impl Item {
    pub fn new(name: String, r#type: String, number: i8, count: u8, flag: i32) -> Self {
        debug_assert!(
            flag == -1
                || [494, 524].contains(&flag)
                || (684..=883).contains(&flag)
                || r#type == "subWeapon" && flag == 65279,
            "invalid value: {flag} ({number})"
        );
        Self {
            name,
            r#type,
            number,
            count,
            flag,
        }
    }
}
