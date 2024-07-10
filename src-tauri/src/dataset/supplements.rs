#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplementFiles {
    pub weapons_yml: String,
    pub chests_yml: String,
    pub seals_yml: String,
    pub shops_yml: String,
    pub events_yml: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct YamlSpot {
    pub name: String,
    #[serde(default)]
    pub requirements: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct YamlShop {
    pub names: String,
    #[serde(default)]
    pub requirements: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponsYaml {
    pub main_weapons: Vec<YamlSpot>,
    pub sub_weapons: Vec<YamlSpot>,
}

#[derive(serde::Deserialize)]
pub struct SpotYaml(pub Vec<YamlSpot>);
