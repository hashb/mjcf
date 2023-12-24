use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Custom {
    #[serde(default)]
    pub numeric: Vec<CustomNumeric>,

    #[serde(default)]
    pub text: Vec<CustomText>,

    #[serde(default)]
    pub tuple: Vec<CustomTuple>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CustomNumeric {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "@data", skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CustomText {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@data")]
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CustomTuple {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(default)]
    pub element: Vec<CustomTupleElement>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CustomTupleElement {
    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@prm")]
    pub prm: Option<f64>,
}
