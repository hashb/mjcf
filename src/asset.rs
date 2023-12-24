use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub texture: Vec<Texture>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hfield: Vec<HField>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mesh: Vec<Mesh>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skin: Vec<crate::DeformableSkin>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<Material>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Texture {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@type", default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(rename = "@content_type", default, skip_serializing_if = "String::is_empty")]
    pub content_type: String,

    #[serde(rename = "@file", default, skip_serializing_if = "String::is_empty")]
    pub file: String,

    #[serde(rename = "@gridsize", skip_serializing_if = "Option::is_none")]
    pub gridsize: Option<[i64; 2]>,

    #[serde(rename = "@gridlayout", default, skip_serializing_if = "String::is_empty")]
    pub gridlayout: String,

    #[serde(rename = "@fileright", default, skip_serializing_if = "String::is_empty")]
    pub fileright: String,

    #[serde(rename = "@fileleft", default, skip_serializing_if = "String::is_empty")]
    pub fileleft: String,

    #[serde(rename = "@fileup", default, skip_serializing_if = "String::is_empty")]
    pub fileup: String,

    #[serde(rename = "@filedown", default, skip_serializing_if = "String::is_empty")]
    pub filedown: String,

    #[serde(rename = "@filefront", default, skip_serializing_if = "String::is_empty")]
    pub filefront: String,

    #[serde(rename = "@fileback", default, skip_serializing_if = "String::is_empty")]
    pub fileback: String,

    #[serde(rename = "@builtin", default, skip_serializing_if = "String::is_empty")]
    pub builtin: String,

    #[serde(rename = "@rgb1", skip_serializing_if = "Option::is_none")]
    pub rgb1: Option<[f64; 3]>,

    #[serde(rename = "@rgb2", skip_serializing_if = "Option::is_none")]
    pub rgb2: Option<[f64; 3]>,

    #[serde(rename = "@mark", default, skip_serializing_if = "String::is_empty")]
    pub mark: String,

    #[serde(rename = "@markrgb", skip_serializing_if = "Option::is_none")]
    pub markrgb: Option<[f64; 3]>,

    #[serde(rename = "@random", skip_serializing_if = "Option::is_none")]
    pub random: Option<f64>,

    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    #[serde(rename = "@hflip", skip_serializing_if = "Option::is_none")]
    pub hflip: Option<bool>,

    #[serde(rename = "@vflip", skip_serializing_if = "Option::is_none")]
    pub vflip: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HField {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@content_type", default, skip_serializing_if = "String::is_empty")]
    pub content_type: String,

    #[serde(rename = "@file", default, skip_serializing_if = "String::is_empty")]
    pub file: String,

    #[serde(rename = "@nrow", skip_serializing_if = "Option::is_none")]
    pub nrow: Option<i64>,

    #[serde(rename = "@ncol", skip_serializing_if = "Option::is_none")]
    pub ncol: Option<i64>,

    #[serde(rename = "@size")]
    pub size: [f64; 4],
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@content_type", default, skip_serializing_if = "String::is_empty")]
    pub content_type: String,

    #[serde(rename = "@file", default, skip_serializing_if = "String::is_empty")]
    pub file: String,

    #[serde(rename = "@vertex", default, skip_serializing_if = "Vec::is_empty")]
    pub vertex: Vec<f64>,

    #[serde(rename = "@normal", default, skip_serializing_if = "Vec::is_empty")]
    pub normal: Vec<f64>,

    #[serde(rename = "@texcoord", default, skip_serializing_if = "Vec::is_empty")]
    pub texcoord: Vec<f64>,

    #[serde(rename = "@face", default, skip_serializing_if = "Vec::is_empty")]
    pub face: Vec<f64>,

    #[serde(rename = "@refpos", skip_serializing_if = "Option::is_none")]
    pub refpos: Option<[f64; 3]>,

    #[serde(rename = "@refquat", skip_serializing_if = "Option::is_none")]
    pub refquat: Option<[f64; 4]>,

    #[serde(rename = "@scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,

    #[serde(rename = "@smoothnormal", skip_serializing_if = "Option::is_none")]
    pub smoothnormal: Option<bool>,

    #[serde(rename = "@plugin", skip_serializing_if = "Option::is_none")]
    pub plugin: Option<crate::Plugin>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Material {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@texture", default, skip_serializing_if = "String::is_empty")]
    pub texture: String,

    #[serde(rename = "@texrepeat", skip_serializing_if = "Option::is_none")]
    pub texrepeat: Option<[f64; 2]>,

    #[serde(rename = "@texuniform", skip_serializing_if = "Option::is_none")]
    pub texuniform: Option<bool>,

    #[serde(rename = "@emission", skip_serializing_if = "Option::is_none")]
    pub emission: Option<f64>,

    #[serde(rename = "@specular", skip_serializing_if = "Option::is_none")]
    pub specular: Option<f64>,

    #[serde(rename = "@shininess", skip_serializing_if = "Option::is_none")]
    pub shininess: Option<f64>,

    #[serde(rename = "@reflectance", skip_serializing_if = "Option::is_none")]
    pub reflectance: Option<f64>,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,
}
