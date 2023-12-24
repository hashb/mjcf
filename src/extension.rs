use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Plugin {
    #[serde(rename = "@plugin", default, skip_serializing_if = "String::is_empty")]
    pub plugin: String,

    #[serde(rename = "@instance", default, skip_serializing_if = "String::is_empty")]
    pub instance: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Extension {
    #[serde(default)]
    pub plugin: Vec<ExtensionPlugin>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExtensionPlugin {
    #[serde(rename = "@plugin")]
    pub plugin: String,

    #[serde(default)]
    pub instance: Vec<Instance>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Instance {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(default)]
    pub config: Vec<Config>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "@key", default, skip_serializing_if = "String::is_empty")]
    pub key: String,

    #[serde(rename = "@value", default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}
