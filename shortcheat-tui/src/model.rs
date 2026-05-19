use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct App {
    pub app: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub description: String,
    #[serde(default)]
    pub sections: Vec<Section>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Section {
    pub name: String,
    #[serde(default)]
    pub shortcuts: Vec<Shortcut>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Shortcut {
    pub keys: String,
    pub description: String,
}
