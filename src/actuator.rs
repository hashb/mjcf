use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Actuator {
    #[serde(default)]
    pub general: Vec<ActuatorGeneral>,

    #[serde(default)]
    pub motor: Vec<ActuatorMotor>,

    #[serde(default)]
    pub position: Vec<ActuatorPosition>,

    #[serde(default)]
    pub velocity: Vec<ActuatorVelocity>,

    #[serde(default)]
    pub intvelocity: Vec<ActuatorIntVelocity>,

    #[serde(default)]
    pub damper: Vec<ActuatorDamper>,

    #[serde(default)]
    pub cylinder: Vec<ActuatorCylinder>,

    #[serde(default)]
    pub muscle: Vec<ActuatorMuscle>,

    #[serde(default)]
    pub adhesion: Vec<ActuatorAdhesion>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<ActuatorPlugin>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorGeneral {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@ctrllimited", default, skip_serializing_if = "String::is_empty")]
    pub ctrllimited: String,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@actlimited", default, skip_serializing_if = "String::is_empty")]
    pub actlimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@actrange", skip_serializing_if = "Option::is_none")]
    pub actrange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@site", default, skip_serializing_if = "String::is_empty")]
    pub site: String,

    #[serde(rename = "@refsite", default, skip_serializing_if = "String::is_empty")]
    pub refsite: String,

    #[serde(rename = "@body", default, skip_serializing_if = "String::is_empty")]
    pub body: String,

    #[serde(rename = "@actdim", skip_serializing_if = "Option::is_none")]
    pub actdim: Option<f64>,

    #[serde(rename = "@dyntype", default, skip_serializing_if = "String::is_empty")]
    pub dyntype: String,

    #[serde(rename = "@gaintype", default, skip_serializing_if = "String::is_empty")]
    pub gaintype: String,

    #[serde(rename = "@biastype", default, skip_serializing_if = "String::is_empty")]
    pub biastype: String,

    #[serde(rename = "@dynprm", default, skip_serializing_if = "Vec::is_empty")]
    pub dynprm: Vec<f64>,

    #[serde(rename = "@gainprm", default, skip_serializing_if = "Vec::is_empty")]
    pub gainprm: Vec<f64>,

    #[serde(rename = "@biasprm", default, skip_serializing_if = "Vec::is_empty")]
    pub biasprm: Vec<f64>,

    #[serde(rename = "@actearly", skip_serializing_if = "Option::is_none")]
    pub actearly: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorMotor {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@ctrllimited", default, skip_serializing_if = "String::is_empty")]
    pub ctrllimited: String,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@site", default, skip_serializing_if = "String::is_empty")]
    pub site: String,

    #[serde(rename = "@refsite", default, skip_serializing_if = "String::is_empty")]
    pub refsite: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorPosition {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@ctrllimited", default, skip_serializing_if = "String::is_empty")]
    pub ctrllimited: String,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@site", default, skip_serializing_if = "String::is_empty")]
    pub site: String,

    #[serde(rename = "@refsite", default, skip_serializing_if = "String::is_empty")]
    pub refsite: String,

    #[serde(rename = "@kp", skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,

    #[serde(rename = "@kv", skip_serializing_if = "Option::is_none")]
    pub kv: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorVelocity {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@ctrllimited", default, skip_serializing_if = "String::is_empty")]
    pub ctrllimited: String,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@site", default, skip_serializing_if = "String::is_empty")]
    pub site: String,

    #[serde(rename = "@refsite", default, skip_serializing_if = "String::is_empty")]
    pub refsite: String,

    #[serde(rename = "@kv", skip_serializing_if = "Option::is_none")]
    pub kv: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorIntVelocity {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@ctrllimited", default, skip_serializing_if = "String::is_empty")]
    pub ctrllimited: String,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@actrange", skip_serializing_if = "Option::is_none")]
    pub actrange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@site", default, skip_serializing_if = "String::is_empty")]
    pub site: String,

    #[serde(rename = "@refsite", default, skip_serializing_if = "String::is_empty")]
    pub refsite: String,

    #[serde(rename = "@kp", skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,

    #[serde(rename = "@kv", skip_serializing_if = "Option::is_none")]
    pub kv: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorDamper {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@site", default, skip_serializing_if = "String::is_empty")]
    pub site: String,

    #[serde(rename = "@refsite", default, skip_serializing_if = "String::is_empty")]
    pub refsite: String,

    #[serde(rename = "@kv", skip_serializing_if = "Option::is_none")]
    pub kv: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorCylinder {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@ctrllimited", default, skip_serializing_if = "String::is_empty")]
    pub ctrllimited: String,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@site", default, skip_serializing_if = "String::is_empty")]
    pub site: String,

    #[serde(rename = "@refsite", default, skip_serializing_if = "String::is_empty")]
    pub refsite: String,

    #[serde(rename = "@timeconst", skip_serializing_if = "Option::is_none")]
    pub timeconst: Option<f64>,

    #[serde(rename = "@area", skip_serializing_if = "Option::is_none")]
    pub area: Option<f64>,

    #[serde(rename = "@diameter", skip_serializing_if = "Option::is_none")]
    pub diameter: Option<f64>,

    #[serde(rename = "@bias", skip_serializing_if = "Option::is_none")]
    pub bias: Option<[f64; 3]>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorMuscle {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@ctrllimited", default, skip_serializing_if = "String::is_empty")]
    pub ctrllimited: String,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@timeconst", skip_serializing_if = "Option::is_none")]
    pub timeconst: Option<[f64; 2]>,

    #[serde(rename = "@tausmooth", skip_serializing_if = "Option::is_none")]
    pub tausmooth: Option<f64>,

    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<[f64; 2]>,

    #[serde(rename = "@force", skip_serializing_if = "Option::is_none")]
    pub force: Option<f64>,

    #[serde(rename = "@scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,

    #[serde(rename = "@lmin", skip_serializing_if = "Option::is_none")]
    pub lmin: Option<f64>,

    #[serde(rename = "@lmax", skip_serializing_if = "Option::is_none")]
    pub lmax: Option<f64>,

    #[serde(rename = "@vmax", skip_serializing_if = "Option::is_none")]
    pub vmax: Option<f64>,

    #[serde(rename = "@fpmax", skip_serializing_if = "Option::is_none")]
    pub fpmax: Option<f64>,

    #[serde(rename = "@fvmax", skip_serializing_if = "Option::is_none")]
    pub fvmax: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorAdhesion {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

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

    #[serde(rename = "@body")]
    pub body: String,

    #[serde(rename = "@gain", skip_serializing_if = "Option::is_none")]
    pub gain: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ActuatorPlugin {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@class", default, skip_serializing_if = "String::is_empty")]
    pub class: String,

    #[serde(rename = "@plugin", default, skip_serializing_if = "String::is_empty")]
    pub plugin: String,

    #[serde(rename = "@instance", default, skip_serializing_if = "String::is_empty")]
    pub instance: String,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@ctrllimited", default, skip_serializing_if = "String::is_empty")]
    pub ctrllimited: String,

    #[serde(rename = "@forcelimited", default, skip_serializing_if = "String::is_empty")]
    pub forcelimited: String,

    #[serde(rename = "@actlimited", default, skip_serializing_if = "String::is_empty")]
    pub actlimited: String,

    #[serde(rename = "@ctrlrange", skip_serializing_if = "Option::is_none")]
    pub ctrlrange: Option<[f64; 2]>,

    #[serde(rename = "@forcerange", skip_serializing_if = "Option::is_none")]
    pub forcerange: Option<[f64; 2]>,

    #[serde(rename = "@actrange", skip_serializing_if = "Option::is_none")]
    pub actrange: Option<[f64; 2]>,

    #[serde(rename = "@lengthrange", skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<[f64; 2]>,

    #[serde(rename = "@gear", default, skip_serializing_if = "Vec::is_empty")]
    pub gear: Vec<f64>,

    #[serde(rename = "@cranklength", skip_serializing_if = "Option::is_none")]
    pub cranklength: Option<f64>,

    #[serde(rename = "@joint", default, skip_serializing_if = "String::is_empty")]
    pub joint: String,

    #[serde(rename = "@jointinparent", default, skip_serializing_if = "String::is_empty")]
    pub jointinparent: String,

    #[serde(rename = "@site", default, skip_serializing_if = "String::is_empty")]
    pub site: String,

    #[serde(rename = "@dyntype", default, skip_serializing_if = "String::is_empty")]
    pub dyntype: String,

    #[serde(rename = "@dynprm", default, skip_serializing_if = "Vec::is_empty")]
    pub dynprm: Vec<f64>,

    #[serde(rename = "@tendon", default, skip_serializing_if = "String::is_empty")]
    pub tendon: String,

    #[serde(rename = "@cranksite", default, skip_serializing_if = "String::is_empty")]
    pub cranksite: String,

    #[serde(rename = "@slidersite", default, skip_serializing_if = "String::is_empty")]
    pub slidersite: String,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,

    #[serde(rename = "@actearly", skip_serializing_if = "Option::is_none")]
    pub actearly: Option<bool>,
}
