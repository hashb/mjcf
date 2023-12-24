use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Tendon {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixed: Vec<TendonFixed>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spatial: Vec<TendonSpatial>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TendonFixed {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@limited", default, skip_serializing_if = "String::is_empty")]
    pub limited: String,

    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<[f64; 2]>,

    #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
    pub solreflimit: Option<[f64; 2]>,

    #[serde(rename = "@solimplimit", skip_serializing_if = "Option::is_none")]
    pub solimplimit: Option<[f64; 5]>,

    #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
    pub solreffriction: Option<[f64; 2]>,

    #[serde(rename = "@solimpfriction", skip_serializing_if = "Option::is_none")]
    pub solimpfriction: Option<[f64; 5]>,

    #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
    pub frictionloss: Option<f64>,

    #[serde(rename = "@springlength", default, skip_serializing_if = "Vec::is_empty")]
    pub springlength: Vec<f64>,

    #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,

    #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
    pub stiffness: Option<f64>,

    #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
    pub damping: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub joint: Vec<TendonFixedJoint>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TendonFixedJoint {
    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@coef")]
    pub coef: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TendonSpatial {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@limited", default, skip_serializing_if = "String::is_empty")]
    pub limited: String,

    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<[f64; 2]>,

    #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
    pub solreflimit: Option<[f64; 2]>,

    #[serde(rename = "@solimplimit", skip_serializing_if = "Option::is_none")]
    pub solimplimit: Option<[f64; 5]>,

    #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
    pub solreffriction: Option<[f64; 2]>,

    #[serde(rename = "@solimpfriction", skip_serializing_if = "Option::is_none")]
    pub solimpfriction: Option<[f64; 5]>,

    #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
    pub frictionloss: Option<f64>,

    #[serde(rename = "@springlength", default, skip_serializing_if = "Vec::is_empty")]
    pub springlength: Vec<f64>,

    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    #[serde(rename = "@material", default, skip_serializing_if = "String::is_empty")]
    pub material: String,

    #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,

    #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
    pub stiffness: Option<f64>,

    #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
    pub damping: Option<f64>,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub site: Vec<TendonSpatialSite>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub geom: Vec<TendonSpatialGeom>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pulley: Vec<TendonSpatialPully>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TendonSpatialSite {
    #[serde(rename = "@site")]
    pub site: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TendonSpatialGeom {
    #[serde(rename = "@geom")]
    pub geom: String,

    #[serde(rename = "@sidesite")]
    pub sidesite: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TendonSpatialPully {
    #[serde(rename = "@divisor")]
    pub divisor: f64,
}
