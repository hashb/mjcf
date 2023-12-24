use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Equality {
    #[serde(default)]
    pub connect: Vec<EqualityConnect>,

    #[serde(default)]
    pub weld: Vec<EqualityWeld>,

    #[serde(default)]
    pub joint: Vec<EqualityJoint>,

    #[serde(default)]
    pub tendon: Vec<EqualityTendon>,

    #[serde(default)]
    pub flex: Vec<EqualityFlex>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EqualityConnect {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@body1")]
    pub body1: String,

    #[serde(rename = "@body2", default, skip_serializing_if = "String::is_empty")]
    pub body2: String,

    #[serde(rename = "@anchor")]
    pub anchor: [f64; 3],

    #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solimp", default, skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EqualityWeld {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@body1")]
    pub body1: String,

    #[serde(rename = "@body2", default, skip_serializing_if = "String::is_empty")]
    pub body2: String,

    #[serde(rename = "@relpose", skip_serializing_if = "Option::is_none")]
    pub relpose: Option<[f64; 7]>,

    #[serde(rename = "@anchor", skip_serializing_if = "Option::is_none")]
    pub anchor: Option<[f64; 3]>,

    #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solimp", default, skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,

    #[serde(rename = "@torquescale", skip_serializing_if = "Option::is_none")]
    pub torquescale: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EqualityJoint {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@joint1")]
    pub joint1: String,

    #[serde(rename = "@joint2", default, skip_serializing_if = "String::is_empty")]
    pub joint2: String,

    #[serde(rename = "@polycoef", skip_serializing_if = "Option::is_none")]
    pub polycoef: Option<[f64; 5]>,

    #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solimp", default, skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EqualityTendon {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@tendon1")]
    pub tendon1: String,

    #[serde(rename = "@tendon2", default, skip_serializing_if = "String::is_empty")]
    pub tendon2: String,

    #[serde(rename = "@polycoef", skip_serializing_if = "Option::is_none")]
    pub polycoef: Option<[f64; 5]>,

    #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solimp", default, skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EqualityFlex {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@flex")]
    pub flex: String,

    #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solimp", default, skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,
}
