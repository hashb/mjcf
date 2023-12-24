use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Deformable {
    #[serde(default)]
    pub flex: Vec<DeformableFlex>,

    #[serde(default)]
    pub skin: Vec<DeformableSkin>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeformableFlex {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@dim", skip_serializing_if = "Option::is_none")]
    pub dim: Option<i64>,

    #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,

    #[serde(rename = "@material", default, skip_serializing_if = "String::is_empty")]
    pub material: String,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,

    #[serde(rename = "@flatskin", skip_serializing_if = "Option::is_none")]
    pub flatskin: Option<bool>,

    #[serde(rename = "@body")]
    pub body: Vec<String>,

    #[serde(rename = "@vertex", default, skip_serializing_if = "Vec::is_empty")]
    pub vertex: Vec<f64>,

    #[serde(rename = "@element")]
    pub element: Vec<i64>,

    #[serde(rename = "@vertex", default, skip_serializing_if = "Vec::is_empty")]
    pub texcoord: Vec<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<DeformableFlexContact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge: Option<DeformableFlexEdge>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeformableFlexContact {
    #[serde(rename = "@contype", skip_serializing_if = "Option::is_none")]
    pub contype: Option<i64>,

    #[serde(rename = "@conaffinity", skip_serializing_if = "Option::is_none")]
    pub conaffinity: Option<i64>,

    #[serde(rename = "@condim", skip_serializing_if = "Option::is_none")]
    pub condim: Option<i64>,

    #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    #[serde(rename = "@friction", default, skip_serializing_if = "Vec::is_empty")]
    pub friction: Vec<f64>,

    #[serde(rename = "@solmix", skip_serializing_if = "Option::is_none")]
    pub solmix: Option<f64>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solimp", default, skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,

    #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,

    #[serde(rename = "@gap", skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,

    #[serde(rename = "@internal", skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,

    #[serde(rename = "@selfcollide", default, skip_serializing_if = "String::is_empty")]
    pub selfcollide: String,

    #[serde(rename = "@activelayers", skip_serializing_if = "Option::is_none")]
    pub activelayers: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeformableFlexEdge {
    #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
    pub stiffness: Option<f64>,

    #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
    pub damping: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Flexcomp {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@dim", skip_serializing_if = "Option::is_none")]
    pub dim: Option<i64>,

    #[serde(rename = "@type", default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(rename = "@count", skip_serializing_if = "Option::is_none")]
    pub count: Option<[i64; 3]>,

    #[serde(rename = "@spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<[f64; 3]>,

    #[serde(rename = "@point", skip_serializing_if = "Vec::is_empty")]
    pub point: Vec<f64>,

    #[serde(rename = "@element", skip_serializing_if = "Vec::is_empty")]
    pub element: Vec<i64>,

    #[serde(rename = "@texcoord", skip_serializing_if = "Vec::is_empty")]
    pub texcoord: Vec<f64>,

    #[serde(rename = "@radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,

    #[serde(rename = "@rigid", skip_serializing_if = "Option::is_none")]
    pub rigid: Option<bool>,

    #[serde(rename = "@mass", skip_serializing_if = "Option::is_none")]
    pub mass: Option<f64>,

    #[serde(rename = "@inertiabox", skip_serializing_if = "Option::is_none")]
    pub inertiabox: Option<f64>,

    #[serde(rename = "@scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,

    #[serde(rename = "@file", default, skip_serializing_if = "String::is_empty")]
    pub file: String,

    #[serde(rename = "@material", default, skip_serializing_if = "String::is_empty")]
    pub material: String,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,

    #[serde(rename = "@flatskin", skip_serializing_if = "Option::is_none")]
    pub flatskin: Option<bool>,

    #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<[f64; 3]>,

    #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
    pub quat: Option<[f64; 4]>,

    #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
    pub axisangle: Option<[f64; 4]>,

    #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
    pub xyaxes: Option<[f64; 6]>,

    #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
    pub zaxis: Option<[f64; 3]>,

    #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
    pub euler: Option<[f64; 3]>,

    #[serde(default)]
    pub contact: Vec<DeformableFlexContact>,

    #[serde(default)]
    pub edge: Vec<FlexcompEdge>,

    #[serde(default)]
    pub pin: Vec<FlexcompPin>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<crate::Plugin>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FlexcompEdge {
    #[serde(rename = "@equality", skip_serializing_if = "Option::is_none")]
    pub equality: Option<bool>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solimp", skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,

    #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
    pub stiffness: Option<f64>,

    #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
    pub damping: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FlexcompPin {
    #[serde(rename = "@id")]
    pub id: Vec<i64>,

    #[serde(rename = "@range")]
    pub range: Vec<i64>,

    #[serde(rename = "@grid")]
    pub grid: Vec<i64>,

    #[serde(rename = "@gridrange")]
    pub gridrange: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeformableSkin {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@file", default, skip_serializing_if = "String::is_empty")]
    pub file: String,

    #[serde(rename = "@material", default, skip_serializing_if = "String::is_empty")]
    pub material: String,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,

    #[serde(rename = "@inflate", skip_serializing_if = "Option::is_none")]
    pub inflate: Option<f64>,

    #[serde(rename = "@vertex", default, skip_serializing_if = "Vec::is_empty")]
    pub vertex: Vec<f64>,

    #[serde(rename = "@texcoord", default, skip_serializing_if = "Vec::is_empty")]
    pub texcoord: Vec<f64>,

    #[serde(rename = "@face", default, skip_serializing_if = "Vec::is_empty")]
    pub face: Vec<i64>,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(default)]
    pub bone: Vec<DeformableBone>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeformableBone {
    #[serde(rename = "@body")]
    pub body: String,

    #[serde(rename = "@bindpos")]
    pub bindpos: [f64; 3],

    #[serde(rename = "@bindquat")]
    pub bindquat: [f64; 4],

    #[serde(rename = "@vertid")]
    pub vertid: Vec<i64>,

    #[serde(rename = "@vertweight")]
    pub vertweight: Vec<f64>,
}
