use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Sensor {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub touch: Vec<SensorTouch>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accelerometer: Vec<SensorAccelerometer>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub velocimeter: Vec<SensorVelocimeter>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gyro: Vec<SensorGyro>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub force: Vec<SensorForce>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub torque: Vec<SensorTorque>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub magnetometer: Vec<SensorMagnetometer>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rangefinder: Vec<SensorRangefinder>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub camprojection: Vec<SensorCamprojection>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jointpos: Vec<SensorJointpos>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jointvel: Vec<SensorJointvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendonpos: Vec<SensorTendonpos>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendonvel: Vec<SensorTendonvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actuatorpos: Vec<SensorActuatorpos>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actuatorvel: Vec<SensorActuatorvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actuatorfrc: Vec<SensorActuatorfrc>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jointactuatorfrc: Vec<SensorJointactuatorfrc>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ballquat: Vec<SensorBallquat>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ballangvel: Vec<SensorBallangvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jointlimitpos: Vec<SensorJointlimitpos>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jointlimitvel: Vec<SensorJointlimitvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jointlimitfrc: Vec<SensorJointlimitfrc>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendonlimitpos: Vec<SensorTendonlimitpos>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendonlimitvel: Vec<SensorTendonlimitvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tendonlimitfrc: Vec<SensorTendonlimitfrc>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub framepos: Vec<SensorFramepos>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub framequat: Vec<SensorFramequat>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub framexaxis: Vec<SensorFramexaxis>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frameyaxis: Vec<SensorFrameyaxis>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub framezaxis: Vec<SensorFramezaxis>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub framelinvel: Vec<SensorFramelinvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frameangvel: Vec<SensorFrameangvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub framelinacc: Vec<SensorFramelinacc>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frameangacc: Vec<SensorFrameangacc>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtreecom: Vec<SensorSubtreecom>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtreelinvel: Vec<SensorSubtreelinvel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtreeangmom: Vec<SensorSubtreeangmom>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clock: Vec<SensorClock>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<SensorUser>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<SensorPlugin>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorTouch {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorAccelerometer {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorVelocimeter {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorGyro {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorForce {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorTorque {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorMagnetometer {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorRangefinder {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorCamprojection {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@site")]
    pub site: String,

    #[serde(rename = "@camera")]
    pub camera: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorJointpos {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorJointvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorTendonpos {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@tendon")]
    pub tendon: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorTendonvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@tendon")]
    pub tendon: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorActuatorpos {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@actuator")]
    pub actuator: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorActuatorvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@actuator")]
    pub actuator: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorActuatorfrc {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@actuator")]
    pub actuator: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorJointactuatorfrc {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorBallquat {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorBallangvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorJointlimitpos {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorJointlimitvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorJointlimitfrc {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@joint")]
    pub joint: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorTendonlimitpos {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@tendon")]
    pub tendon: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorTendonlimitvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@tendon")]
    pub tendon: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorTendonlimitfrc {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@tendon")]
    pub tendon: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFramepos {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@reftype", default, skip_serializing_if = "String::is_empty")]
    pub reftype: String,

    #[serde(rename = "@refname", default, skip_serializing_if = "String::is_empty")]
    pub refname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFramequat {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@reftype", default, skip_serializing_if = "String::is_empty")]
    pub reftype: String,

    #[serde(rename = "@refname", default, skip_serializing_if = "String::is_empty")]
    pub refname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFramexaxis {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@reftype", default, skip_serializing_if = "String::is_empty")]
    pub reftype: String,

    #[serde(rename = "@refname", default, skip_serializing_if = "String::is_empty")]
    pub refname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFrameyaxis {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@reftype", default, skip_serializing_if = "String::is_empty")]
    pub reftype: String,

    #[serde(rename = "@refname", default, skip_serializing_if = "String::is_empty")]
    pub refname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFramezaxis {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@reftype", default, skip_serializing_if = "String::is_empty")]
    pub reftype: String,

    #[serde(rename = "@refname", default, skip_serializing_if = "String::is_empty")]
    pub refname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFramelinvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@reftype", default, skip_serializing_if = "String::is_empty")]
    pub reftype: String,

    #[serde(rename = "@refname", default, skip_serializing_if = "String::is_empty")]
    pub refname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFrameangvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@reftype", default, skip_serializing_if = "String::is_empty")]
    pub reftype: String,

    #[serde(rename = "@refname", default, skip_serializing_if = "String::is_empty")]
    pub refname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFramelinacc {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorFrameangacc {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype")]
    pub objtype: String,

    #[serde(rename = "@objname")]
    pub objname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorSubtreecom {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@body")]
    pub body: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorSubtreelinvel {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@body")]
    pub body: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorSubtreeangmom {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@body")]
    pub body: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorClock {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorUser {
    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype", default, skip_serializing_if = "String::is_empty")]
    pub objtype: String,

    #[serde(rename = "@objname", default, skip_serializing_if = "String::is_empty")]
    pub objname: String,

    #[serde(rename = "@datatype", default, skip_serializing_if = "String::is_empty")]
    pub datatype: String,

    #[serde(rename = "@needstage", default, skip_serializing_if = "String::is_empty")]
    pub needstage: String,

    #[serde(rename = "@dim")]
    pub dim: i64,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SensorPlugin {
    #[serde(rename = "@plugin", default, skip_serializing_if = "String::is_empty")]
    pub plugin: String,

    #[serde(rename = "@instance", default, skip_serializing_if = "String::is_empty")]
    pub instance: String,

    #[serde(rename = "@name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(rename = "@objtype", default, skip_serializing_if = "String::is_empty")]
    pub objtype: String,

    #[serde(rename = "@objname", default, skip_serializing_if = "String::is_empty")]
    pub objname: String,

    #[serde(rename = "@reftype", default, skip_serializing_if = "String::is_empty")]
    pub reftype: String,

    #[serde(rename = "@refname", default, skip_serializing_if = "String::is_empty")]
    pub refname: String,

    #[serde(rename = "@cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,

    #[serde(rename = "@noise", skip_serializing_if = "Option::is_none")]
    pub noise: Option<f64>,

    #[serde(rename = "@user", default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<f64>,
}
