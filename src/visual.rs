use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Camera {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@fovy", skip_serializing_if = "Option::is_none")]
    pub fovy: Option<f64>,

    #[serde(rename = "@ipd", skip_serializing_if = "Option::is_none")]
    pub ipd: Option<f64>,

    #[serde(rename = "@resolution", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<[i64; 2]>,

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

    #[serde(rename = "@mode", default, skip_serializing_if = "String::is_empty")]
    pub mode: String,

    #[serde(rename = "@target", default, skip_serializing_if = "String::is_empty")]
    pub target: String,

    #[serde(rename = "@focal", skip_serializing_if = "Option::is_none")]
    pub focal: Option<[f64; 2]>,

    #[serde(rename = "@focalpixel", skip_serializing_if = "Option::is_none")]
    pub focalpixel: Option<[f64; 2]>,

    #[serde(rename = "@principal", skip_serializing_if = "Option::is_none")]
    pub principal: Option<[f64; 2]>,

    #[serde(rename = "@principalpixel", skip_serializing_if = "Option::is_none")]
    pub principalpixel: Option<[f64; 2]>,

    #[serde(rename = "@sensorsize", skip_serializing_if = "Option::is_none")]
    pub sensorsize: Option<[f64; 2]>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Light {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@directional", skip_serializing_if = "Option::is_none")]
    pub directional: Option<bool>,

    #[serde(rename = "@castshadow", skip_serializing_if = "Option::is_none")]
    pub castshadow: Option<bool>,

    #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<[f64; 3]>,

    #[serde(rename = "@dir", skip_serializing_if = "Option::is_none")]
    pub dir: Option<[f64; 3]>,

    #[serde(rename = "@attenuation", skip_serializing_if = "Option::is_none")]
    pub attenuation: Option<f64>,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@exponent", skip_serializing_if = "Option::is_none")]
    pub exponent: Option<f64>,

    #[serde(rename = "@ambient", skip_serializing_if = "Option::is_none")]
    pub ambient: Option<[f64; 3]>,

    #[serde(rename = "@diffuse", skip_serializing_if = "Option::is_none")]
    pub diffuse: Option<[f64; 3]>,

    #[serde(rename = "@specular", skip_serializing_if = "Option::is_none")]
    pub specular: Option<[f64; 3]>,

    #[serde(rename = "@mode", default, skip_serializing_if = "String::is_empty")]
    pub mode: String,

    #[serde(rename = "@target", default, skip_serializing_if = "String::is_empty")]
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Visual {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<VisualGlobal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<VisualQuality>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headlight: Option<VisualHeadlight>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<VisualMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<VisualScale>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgba: Option<VisualRgba>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VisualGlobal {
    #[serde(rename = "@fovy", skip_serializing_if = "Option::is_none")]
    pub fovy: Option<f64>,

    #[serde(rename = "@ipd", skip_serializing_if = "Option::is_none")]
    pub ipd: Option<f64>,

    #[serde(rename = "@azimuth", skip_serializing_if = "Option::is_none")]
    pub azimuth: Option<f64>,

    #[serde(rename = "@elevation", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    #[serde(rename = "@linewidth", skip_serializing_if = "Option::is_none")]
    pub linewidth: Option<f64>,

    #[serde(rename = "@glow", skip_serializing_if = "Option::is_none")]
    pub glow: Option<f64>,

    #[serde(rename = "@offwidth", skip_serializing_if = "Option::is_none")]
    pub offwidth: Option<i64>,

    #[serde(rename = "@offheight", skip_serializing_if = "Option::is_none")]
    pub offheight: Option<i64>,

    #[serde(rename = "@realtime", skip_serializing_if = "Option::is_none")]
    pub realtime: Option<f64>,

    #[serde(rename = "@ellipsoidinertia", skip_serializing_if = "Option::is_none")]
    pub ellipsoidinertia: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VisualQuality {
    #[serde(rename = "@shadowsize", skip_serializing_if = "Option::is_none")]
    pub shadowsize: Option<i64>,

    #[serde(rename = "@offsamples", skip_serializing_if = "Option::is_none")]
    pub offsamples: Option<i64>,

    #[serde(rename = "@numslices", skip_serializing_if = "Option::is_none")]
    pub numslices: Option<i64>,

    #[serde(rename = "@numstacks", skip_serializing_if = "Option::is_none")]
    pub numstacks: Option<i64>,

    #[serde(rename = "@numquads", skip_serializing_if = "Option::is_none")]
    pub numquads: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VisualHeadlight {
    #[serde(rename = "@ambient", skip_serializing_if = "Option::is_none")]
    pub ambient: Option<[f64; 3]>,

    #[serde(rename = "@diffuse", skip_serializing_if = "Option::is_none")]
    pub diffuse: Option<[f64; 3]>,

    #[serde(rename = "@specular", skip_serializing_if = "Option::is_none")]
    pub specular: Option<[f64; 3]>,

    #[serde(rename = "@active", skip_serializing_if = "Option::is_none")]
    pub active: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VisualMap {
    #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
    pub stiffness: Option<f64>,

    #[serde(rename = "@stiffnessrot", skip_serializing_if = "Option::is_none")]
    pub stiffnessrot: Option<f64>,

    #[serde(rename = "@force", skip_serializing_if = "Option::is_none")]
    pub force: Option<f64>,

    #[serde(rename = "@torque", skip_serializing_if = "Option::is_none")]
    pub torque: Option<f64>,

    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,

    #[serde(rename = "@fogstart", skip_serializing_if = "Option::is_none")]
    pub fogstart: Option<f64>,

    #[serde(rename = "@fogend", skip_serializing_if = "Option::is_none")]
    pub fogend: Option<f64>,

    #[serde(rename = "@znear", skip_serializing_if = "Option::is_none")]
    pub znear: Option<f64>,

    #[serde(rename = "@zfar", skip_serializing_if = "Option::is_none")]
    pub zfar: Option<f64>,

    #[serde(rename = "@haze", skip_serializing_if = "Option::is_none")]
    pub haze: Option<f64>,

    #[serde(rename = "@shadowclip", skip_serializing_if = "Option::is_none")]
    pub shadowclip: Option<f64>,

    #[serde(rename = "@shadowscale", skip_serializing_if = "Option::is_none")]
    pub shadowscale: Option<f64>,

    #[serde(rename = "@actuatortendon", skip_serializing_if = "Option::is_none")]
    pub actuatortendon: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VisualScale {
    #[serde(rename = "@forcewidth", skip_serializing_if = "Option::is_none")]
    pub forcewidth: Option<f64>,

    #[serde(rename = "@contactwidth", skip_serializing_if = "Option::is_none")]
    pub contactwidth: Option<f64>,

    #[serde(rename = "@contactheight", skip_serializing_if = "Option::is_none")]
    pub contactheight: Option<f64>,

    #[serde(rename = "@connect", skip_serializing_if = "Option::is_none")]
    pub connect: Option<f64>,

    #[serde(rename = "@com", skip_serializing_if = "Option::is_none")]
    pub com: Option<f64>,

    #[serde(rename = "@camera", skip_serializing_if = "Option::is_none")]
    pub camera: Option<f64>,

    #[serde(rename = "@light", skip_serializing_if = "Option::is_none")]
    pub light: Option<f64>,

    #[serde(rename = "@selectpoint", skip_serializing_if = "Option::is_none")]
    pub selectpoint: Option<f64>,

    #[serde(rename = "@jointlength", skip_serializing_if = "Option::is_none")]
    pub jointlength: Option<f64>,

    #[serde(rename = "@jointwidth", skip_serializing_if = "Option::is_none")]
    pub jointwidth: Option<f64>,

    #[serde(rename = "@actuatorlength", skip_serializing_if = "Option::is_none")]
    pub actuatorlength: Option<f64>,

    #[serde(rename = "@actuatorwidth", skip_serializing_if = "Option::is_none")]
    pub actuatorwidth: Option<f64>,

    #[serde(rename = "@framelength", skip_serializing_if = "Option::is_none")]
    pub framelength: Option<f64>,

    #[serde(rename = "@framewidth", skip_serializing_if = "Option::is_none")]
    pub framewidth: Option<f64>,

    #[serde(rename = "@constraint", skip_serializing_if = "Option::is_none")]
    pub constraint: Option<f64>,

    #[serde(rename = "@slidercrank", skip_serializing_if = "Option::is_none")]
    pub slidercrank: Option<f64>,

    #[serde(rename = "@frustum", skip_serializing_if = "Option::is_none")]
    pub frustum: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VisualRgba {
    #[serde(rename = "@fog", skip_serializing_if = "Option::is_none")]
    pub fog: Option<[f64; 4]>,

    #[serde(rename = "@haze", skip_serializing_if = "Option::is_none")]
    pub haze: Option<[f64; 4]>,

    #[serde(rename = "@force", skip_serializing_if = "Option::is_none")]
    pub force: Option<[f64; 4]>,

    #[serde(rename = "@inertia", skip_serializing_if = "Option::is_none")]
    pub inertia: Option<[f64; 4]>,

    #[serde(rename = "@joint", skip_serializing_if = "Option::is_none")]
    pub joint: Option<[f64; 4]>,

    #[serde(rename = "@actuator", skip_serializing_if = "Option::is_none")]
    pub actuator: Option<[f64; 4]>,

    #[serde(rename = "@actuatornegative", skip_serializing_if = "Option::is_none")]
    pub actuatornegative: Option<[f64; 4]>,

    #[serde(rename = "@actuatorpositive", skip_serializing_if = "Option::is_none")]
    pub actuatorpositive: Option<[f64; 4]>,

    #[serde(rename = "@com", skip_serializing_if = "Option::is_none")]
    pub com: Option<[f64; 4]>,

    #[serde(rename = "@camera", skip_serializing_if = "Option::is_none")]
    pub camera: Option<[f64; 4]>,

    #[serde(rename = "@light", skip_serializing_if = "Option::is_none")]
    pub light: Option<[f64; 4]>,

    #[serde(rename = "@selectpoint", skip_serializing_if = "Option::is_none")]
    pub selectpoint: Option<[f64; 4]>,

    #[serde(rename = "@connect", skip_serializing_if = "Option::is_none")]
    pub connect: Option<[f64; 4]>,

    #[serde(rename = "@contactpoint", skip_serializing_if = "Option::is_none")]
    pub contactpoint: Option<[f64; 4]>,

    #[serde(rename = "@contactforce", skip_serializing_if = "Option::is_none")]
    pub contactforce: Option<[f64; 4]>,

    #[serde(rename = "@contactfriction", skip_serializing_if = "Option::is_none")]
    pub contactfriction: Option<[f64; 4]>,

    #[serde(rename = "@contacttorque", skip_serializing_if = "Option::is_none")]
    pub contacttorque: Option<[f64; 4]>,

    #[serde(rename = "@contactgap", skip_serializing_if = "Option::is_none")]
    pub contactgap: Option<[f64; 4]>,

    #[serde(rename = "@rangefinder", skip_serializing_if = "Option::is_none")]
    pub rangefinder: Option<[f64; 4]>,

    #[serde(rename = "@constraint", skip_serializing_if = "Option::is_none")]
    pub constraint: Option<[f64; 4]>,

    #[serde(rename = "@slidercrank", skip_serializing_if = "Option::is_none")]
    pub slidercrank: Option<[f64; 4]>,

    #[serde(rename = "@crankbroken", skip_serializing_if = "Option::is_none")]
    pub crankbroken: Option<[f64; 4]>,

    #[serde(rename = "@frustum", skip_serializing_if = "Option::is_none")]
    pub frustum: Option<[f64; 4]>,
}
