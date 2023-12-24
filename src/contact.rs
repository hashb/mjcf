use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Contact {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pair: Vec<ContactPair>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<ContactExclude>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContactPair {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@geom1", default, skip_serializing_if = "String::is_empty")]
    pub geom1: String,

    #[serde(rename = "@geom2", default, skip_serializing_if = "String::is_empty")]
    pub geom2: String,

    #[serde(rename = "@condim", skip_serializing_if = "Option::is_none")]
    pub condim: Option<i64>,

    #[serde(rename = "@friction", default, skip_serializing_if = "Vec::is_empty")]
    pub friction: Vec<f64>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solreffriction", default, skip_serializing_if = "Vec::is_empty")]
    pub solreffriction: Vec<f64>,

    #[serde(rename = "@solimp", default, skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,

    #[serde(rename = "@gap", skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,

    #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContactExclude {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@body1")]
    pub body1: String,

    #[serde(rename = "@body2")]
    pub body2: String,
}
