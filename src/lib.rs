mod actuator;
mod asset;
mod composite;
mod contact;
mod custom;
mod deformable;
mod equality;
mod extension;
mod option;
mod sensor;
mod tendon;
mod visual;

pub use actuator::*;
pub use asset::*;
pub use composite::*;
pub use contact::*;
pub use custom::*;
pub use deformable::*;
pub use equality::*;
pub use extension::*;
pub use option::*;
pub use sensor::*;
pub use tendon::*;
pub use visual::*;

use serde::{Deserialize, Serialize};

/// Deserialize an instance of [Mujoco] from a string of MJCF XML text.
pub fn from_str(text: &str) -> Result<Mujoco, quick_xml::DeError> {
    // The XML parser can not deal with multi-line strings. Some MJCF attributes
    // have a lot of data in them, which naturally gets split up into many lines.
    // Quick fix: just replace all of the newlines with spaces.
    let text = text.replace(char::is_whitespace, " ");
    quick_xml::de::from_str(&text)
}

/// Serialize a [Mujoco] struct into an MJCF XML `String`.
pub fn to_string(mujoco: &Mujoco) -> Result<String, quick_xml::DeError> {
    quick_xml::se::to_string(mujoco)
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename = "mujoco")]
#[serde(deny_unknown_fields)]
pub struct Mujoco {
    #[serde(rename = "@model", default, skip_serializing_if = "String::is_empty")]
    pub model: String,

    #[serde(default)]
    pub include: Vec<Include>,

    #[serde(default)]
    pub compiler: Vec<Compiler>,

    #[serde(default)]
    pub size: Vec<Size>,

    #[serde(default)]
    pub statistic: Vec<Statistic>,

    #[serde(default)]
    pub visual: Vec<Visual>,

    #[serde(default)]
    pub asset: Vec<Asset>,

    #[serde(default)]
    pub option: Vec<MujocoOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worldbody: Option<WorldBody>,

    #[serde(default)]
    pub deformable: Vec<Deformable>,

    #[serde(default)]
    pub contact: Vec<Contact>,

    #[serde(default)]
    pub equality: Vec<Equality>,

    #[serde(default)]
    pub tendon: Vec<Tendon>,

    #[serde(default)]
    pub actuator: Vec<Actuator>,

    #[serde(default)]
    pub sensor: Vec<Sensor>,

    #[serde(default)]
    pub keyframe: Vec<Keyframe>,

    #[serde(default)]
    pub default: Vec<MujocoDefault>,

    #[serde(default)]
    pub custom: Vec<Custom>,

    #[serde(default)]
    pub extension: Vec<Extension>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Include {
    #[serde(rename = "@file")]
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WorldBody {
    #[serde(default)]
    pub body: Vec<Body>,

    #[serde(default)]
    pub geom: Vec<BodyGeom>,

    #[serde(default)]
    pub site: Vec<BodySite>,

    #[serde(default)]
    pub camera: Vec<Camera>,

    #[serde(default)]
    pub light: Vec<Light>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Plugin>,

    #[serde(default)]
    pub composite: Vec<Composite>,

    #[serde(default)]
    pub flexcomp: Vec<Flexcomp>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Body {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@childclass", default, skip_serializing_if = "String::is_empty")]
    pub childclass: String,

    #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<[f64; 3]>,

    #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
    pub quat: Option<[f64; 4]>,

    #[serde(rename = "@mocap", skip_serializing_if = "Option::is_none")]
    pub mocap: Option<bool>,

    #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
    pub axisangle: Option<[f64; 4]>,

    #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
    pub xyaxes: Option<[f64; 6]>,

    #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
    pub zaxis: Option<[f64; 3]>,

    #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
    pub euler: Option<[f64; 3]>,

    #[serde(rename = "@gravcomp", skip_serializing_if = "Option::is_none")]
    pub gravcomp: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(default)]
    pub body: Vec<Body>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inertial: Option<BodyInertial>,

    #[serde(default)]
    pub joint: Vec<BodyJoint>,

    #[serde(default)]
    pub freejoint: Vec<BodyFreejoint>,

    #[serde(default)]
    pub geom: Vec<BodyGeom>,

    #[serde(default)]
    pub site: Vec<BodySite>,

    #[serde(default)]
    pub camera: Vec<Camera>,

    #[serde(default)]
    pub light: Vec<Light>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Plugin>,

    #[serde(default)]
    pub composite: Vec<Composite>,

    #[serde(default)]
    pub flexcomp: Vec<Flexcomp>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BodyInertial {
    #[serde(rename = "@pos")]
    pub pos: [f64; 3],

    #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
    pub quat: Option<[f64; 4]>,

    #[serde(rename = "@mass")]
    pub mass: f64,

    #[serde(rename = "@diaginertia", skip_serializing_if = "Option::is_none")]
    pub diaginertia: Option<[f64; 3]>,

    #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
    pub axisangle: Option<[f64; 4]>,

    #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
    pub xyaxes: Option<[f64; 6]>,

    #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
    pub zaxis: Option<[f64; 3]>,

    #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
    pub euler: Option<[f64; 3]>,

    #[serde(rename = "@fullinertia", skip_serializing_if = "Option::is_none")]
    pub fullinertia: Option<[f64; 6]>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BodyJoint {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@type", default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<[f64; 3]>,

    #[serde(rename = "@axis", skip_serializing_if = "Option::is_none")]
    pub axis: Option<[f64; 3]>,

    #[serde(rename = "@springdamper", skip_serializing_if = "Option::is_none")]
    pub springdamper: Option<[f64; 2]>,

    #[serde(rename = "@limited", default, skip_serializing_if = "String::is_empty")]
    pub limited: String,

    #[serde(rename = "@actuatorfrclimited", default, skip_serializing_if = "String::is_empty")]
    pub actuatorfrclimited: String,

    #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
    pub solreflimit: Option<[f64; 2]>,

    #[serde(rename = "@solimplimit", default, skip_serializing_if = "Vec::is_empty")]
    pub solimplimit: Vec<f64>,

    #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
    pub solreffriction: Option<[f64; 2]>,

    #[serde(rename = "@solimpfriction", default, skip_serializing_if = "Vec::is_empty")]
    pub solimpfriction: Vec<f64>,

    #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
    pub stiffness: Option<f64>,

    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<[f64; 2]>,

    #[serde(rename = "@actuatorfrcrange", skip_serializing_if = "Option::is_none")]
    pub actuatorfrcrange: Option<[f64; 2]>,

    #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,

    #[serde(rename = "@ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<f64>,

    #[serde(rename = "@springref", skip_serializing_if = "Option::is_none")]
    pub springref: Option<f64>,

    #[serde(rename = "@armature", skip_serializing_if = "Option::is_none")]
    pub armature: Option<f64>,

    #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
    pub damping: Option<f64>,

    #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
    pub frictionloss: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BodyFreejoint {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BodyGeom {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@type", default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(rename = "@contype", skip_serializing_if = "Option::is_none")]
    pub contype: Option<i64>,

    #[serde(rename = "@conaffinity", skip_serializing_if = "Option::is_none")]
    pub conaffinity: Option<i64>,

    #[serde(rename = "@condim", skip_serializing_if = "Option::is_none")]
    pub condim: Option<i64>,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<f64>>,

    #[serde(rename = "@material", default, skip_serializing_if = "String::is_empty")]
    pub material: String,

    #[serde(rename = "@friction", default, skip_serializing_if = "Vec::is_empty")]
    pub friction: Vec<f64>,

    #[serde(rename = "@mass", skip_serializing_if = "Option::is_none")]
    pub mass: Option<f64>,

    #[serde(rename = "@density", skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,

    #[serde(rename = "@shellinertia", skip_serializing_if = "Option::is_none")]
    pub shellinertia: Option<bool>,

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

    #[serde(rename = "@fromto", skip_serializing_if = "Option::is_none")]
    pub fromto: Option<[f64; 6]>,

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

    #[serde(rename = "@hfield", default, skip_serializing_if = "String::is_empty")]
    pub hfield: String,

    #[serde(rename = "@mesh", default, skip_serializing_if = "String::is_empty")]
    pub mesh: String,

    #[serde(rename = "@fitscale", skip_serializing_if = "Option::is_none")]
    pub fitscale: Option<f64>,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,

    #[serde(rename = "@fluidshape", default, skip_serializing_if = "String::is_empty")]
    pub fluidshape: String,

    #[serde(rename = "@fluidcoef", skip_serializing_if = "Option::is_none")]
    pub fluidcoef: Option<[f64; 5]>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Plugin>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BodySite {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@type", default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<[f64; 3]>,

    #[serde(rename = "@quat", skip_serializing_if = "Option::is_none")]
    pub quat: Option<[f64; 4]>,

    #[serde(rename = "@material", default, skip_serializing_if = "String::is_empty")]
    pub material: String,

    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<f64>>,

    #[serde(rename = "@fromto", skip_serializing_if = "Option::is_none")]
    pub fromto: Option<[f64; 6]>,

    #[serde(rename = "@axisangle", skip_serializing_if = "Option::is_none")]
    pub axisangle: Option<[f64; 4]>,

    #[serde(rename = "@xyaxes", skip_serializing_if = "Option::is_none")]
    pub xyaxes: Option<[f64; 6]>,

    #[serde(rename = "@zaxis", skip_serializing_if = "Option::is_none")]
    pub zaxis: Option<[f64; 3]>,

    #[serde(rename = "@euler", skip_serializing_if = "Option::is_none")]
    pub euler: Option<[f64; 3]>,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Keyframe {
    #[serde(default)]
    pub key: Vec<KeyframeKey>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct KeyframeKey {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,

    #[serde(rename = "@qpos", default, skip_serializing_if = "Vec::is_empty")]
    pub qpos: Vec<f64>,

    #[serde(rename = "@qvel", default, skip_serializing_if = "Vec::is_empty")]
    pub qvel: Vec<f64>,

    #[serde(rename = "@act", default, skip_serializing_if = "Vec::is_empty")]
    pub act: Vec<f64>,

    #[serde(rename = "@ctrl", default, skip_serializing_if = "Vec::is_empty")]
    pub ctrl: Vec<f64>,

    #[serde(rename = "@mpos", default, skip_serializing_if = "Vec::is_empty")]
    pub mpos: Vec<f64>,

    #[serde(rename = "@mquat", default, skip_serializing_if = "Vec::is_empty")]
    pub mquat: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MujocoDefault {
    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default: Vec<MujocoDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Mesh>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<Material>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub joint: Option<BodyJoint>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geom: Option<BodyGeom>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<BodySite>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<Camera>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<Light>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<ContactPair>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub equality: Option<MujocoDefaultEquality>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tendon: Option<TendonSpatial>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<ActuatorGeneral>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub motor: Option<ActuatorMotor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<ActuatorPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<ActuatorVelocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intvelocity: Option<ActuatorIntVelocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub damper: Option<ActuatorDamper>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cylinder: Option<ActuatorCylinder>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub muscle: Option<ActuatorMuscle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adhesion: Option<MujocoDefaultAdhesion>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MujocoDefaultEquality {
    #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "@solref", skip_serializing_if = "Option::is_none")]
    pub solref: Option<[f64; 2]>,

    #[serde(rename = "@solimp", default, skip_serializing_if = "Vec::is_empty")]
    pub solimp: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MujocoDefaultAdhesion {
    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@gain", skip_serializing_if = "Option::is_none")]
    pub gain: Option<f64>,
}
