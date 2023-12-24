use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Compiler {
    #[serde(rename = "@autolimits", skip_serializing_if = "Option::is_none")]
    pub autolimits: Option<bool>,

    #[serde(rename = "@boundmass", skip_serializing_if = "Option::is_none")]
    pub boundmass: Option<f64>,

    #[serde(rename = "@boundinertia", skip_serializing_if = "Option::is_none")]
    pub boundinertia: Option<f64>,

    #[serde(rename = "@settotalmass", skip_serializing_if = "Option::is_none")]
    pub settotalmass: Option<f64>,

    #[serde(rename = "@balanceinertia", skip_serializing_if = "Option::is_none")]
    pub balanceinertia: Option<bool>,

    #[serde(rename = "@strippath", skip_serializing_if = "Option::is_none")]
    pub strippath: Option<bool>,

    #[serde(rename = "@coordinate", default, skip_serializing_if = "String::is_empty")]
    pub coordinate: String,

    #[serde(rename = "@angle", default, skip_serializing_if = "String::is_empty")]
    pub angle: String,

    #[serde(rename = "@fitaabb", skip_serializing_if = "Option::is_none")]
    pub fitaabb: Option<bool>,

    #[serde(rename = "@eulerseq", default, skip_serializing_if = "String::is_empty")]
    pub eulerseq: String,

    #[serde(rename = "@meshdir", default, skip_serializing_if = "String::is_empty")]
    pub meshdir: String,

    #[serde(rename = "@texturedir", default, skip_serializing_if = "String::is_empty")]
    pub texturedir: String,

    #[serde(rename = "@assetdir", default, skip_serializing_if = "String::is_empty")]
    pub assetdir: String,

    #[serde(rename = "@discardvisual", skip_serializing_if = "Option::is_none")]
    pub discardvisual: Option<bool>,

    #[serde(rename = "@convexhull", skip_serializing_if = "Option::is_none")]
    pub convexhull: Option<bool>,

    #[serde(rename = "@usethread", skip_serializing_if = "Option::is_none")]
    pub usethread: Option<bool>,

    #[serde(rename = "@fusestatic", skip_serializing_if = "Option::is_none")]
    pub fusestatic: Option<bool>,

    #[serde(rename = "@inertiafromgeom", default, skip_serializing_if = "String::is_empty")]
    pub inertiafromgeom: String,

    #[serde(rename = "@inertiagrouprange", skip_serializing_if = "Option::is_none")]
    pub inertiagrouprange: Option<[i64; 2]>,

    #[serde(rename = "@exactmeshinertia", skip_serializing_if = "Option::is_none")]
    pub exactmeshinertia: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lengthrange: Option<CompilerLengthRange>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompilerLengthRange {
    #[serde(rename = "@mode", default, skip_serializing_if = "String::is_empty")]
    pub mode: String,

    #[serde(rename = "@useexisting", skip_serializing_if = "Option::is_none")]
    pub useexisting: Option<bool>,

    #[serde(rename = "@uselimit", skip_serializing_if = "Option::is_none")]
    pub uselimit: Option<bool>,

    #[serde(rename = "@accel", skip_serializing_if = "Option::is_none")]
    pub accel: Option<f64>,

    #[serde(rename = "@maxforce", skip_serializing_if = "Option::is_none")]
    pub maxforce: Option<f64>,

    #[serde(rename = "@timeconst", skip_serializing_if = "Option::is_none")]
    pub timeconst: Option<f64>,

    #[serde(rename = "@timestep", skip_serializing_if = "Option::is_none")]
    pub timestep: Option<f64>,

    #[serde(rename = "@inttotal", skip_serializing_if = "Option::is_none")]
    pub inttotal: Option<f64>,

    #[serde(rename = "@interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,

    #[serde(rename = "@tolrange", skip_serializing_if = "Option::is_none")]
    pub tolrange: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Size {
    #[serde(rename = "@memory", default, skip_serializing_if = "String::is_empty")]
    pub memory: String,

    #[serde(rename = "@njmax", skip_serializing_if = "Option::is_none")]
    pub njmax: Option<i64>,

    #[serde(rename = "@nconmax", skip_serializing_if = "Option::is_none")]
    pub nconmax: Option<i64>,

    #[serde(rename = "@nstack", skip_serializing_if = "Option::is_none")]
    pub nstack: Option<i64>,

    #[serde(rename = "@nuserdata", skip_serializing_if = "Option::is_none")]
    pub nuserdata: Option<i64>,

    #[serde(rename = "@nkey", skip_serializing_if = "Option::is_none")]
    pub nkey: Option<i64>,

    #[serde(rename = "@nuser_body", skip_serializing_if = "Option::is_none")]
    pub nuser_body: Option<i64>,

    #[serde(rename = "@nuser_jnt", skip_serializing_if = "Option::is_none")]
    pub nuser_jnt: Option<i64>,

    #[serde(rename = "@nuser_geom", skip_serializing_if = "Option::is_none")]
    pub nuser_geom: Option<i64>,

    #[serde(rename = "@nuser_site", skip_serializing_if = "Option::is_none")]
    pub nuser_site: Option<i64>,

    #[serde(rename = "@nuser_cam", skip_serializing_if = "Option::is_none")]
    pub nuser_cam: Option<i64>,

    #[serde(rename = "@nuser_tendon", skip_serializing_if = "Option::is_none")]
    pub nuser_tendon: Option<i64>,

    #[serde(rename = "@nuser_actuator", skip_serializing_if = "Option::is_none")]
    pub nuser_actuator: Option<i64>,

    #[serde(rename = "@nuser_sensor", skip_serializing_if = "Option::is_none")]
    pub nuser_sensor: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Statistic {
    #[serde(rename = "@meanmass", skip_serializing_if = "Option::is_none")]
    pub meanmass: Option<f64>,

    #[serde(rename = "@meaninertia", skip_serializing_if = "Option::is_none")]
    pub meaninertia: Option<f64>,

    #[serde(rename = "@meansize", skip_serializing_if = "Option::is_none")]
    pub meansize: Option<f64>,

    #[serde(rename = "@extent", skip_serializing_if = "Option::is_none")]
    pub extent: Option<f64>,

    #[serde(rename = "@center", skip_serializing_if = "Option::is_none")]
    pub center: Option<[f64; 3]>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MujocoOption {
    #[serde(rename = "@timestep", skip_serializing_if = "Option::is_none")]
    pub timestep: Option<f64>,

    #[serde(rename = "@apirate", skip_serializing_if = "Option::is_none")]
    pub apirate: Option<f64>,

    #[serde(rename = "@impratio", skip_serializing_if = "Option::is_none")]
    pub impratio: Option<f64>,

    #[serde(rename = "@gravity", skip_serializing_if = "Option::is_none")]
    pub gravity: Option<[f64; 3]>,

    #[serde(rename = "@wind", skip_serializing_if = "Option::is_none")]
    pub wind: Option<[f64; 3]>,

    #[serde(rename = "@magnetic", skip_serializing_if = "Option::is_none")]
    pub magnetic: Option<[f64; 3]>,

    #[serde(rename = "@density", skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,

    #[serde(rename = "@viscosity", skip_serializing_if = "Option::is_none")]
    pub viscosity: Option<f64>,

    #[serde(rename = "@o_margin", skip_serializing_if = "Option::is_none")]
    pub o_margin: Option<f64>,

    #[serde(rename = "@o_solref", skip_serializing_if = "Option::is_none")]
    pub o_solref: Option<[f64; 2]>,

    #[serde(rename = "@o_solimp", skip_serializing_if = "Option::is_none")]
    pub o_solimp: Option<[f64; 5]>,

    #[serde(rename = "@o_friction", skip_serializing_if = "Option::is_none")]
    pub o_friction: Option<Vec<f64>>,

    #[serde(rename = "@integrator", default, skip_serializing_if = "String::is_empty")]
    pub integrator: String,

    #[serde(rename = "@cone", default, skip_serializing_if = "String::is_empty")]
    pub cone: String,

    #[serde(rename = "@jacobian", default, skip_serializing_if = "String::is_empty")]
    pub jacobian: String,

    #[serde(rename = "@solver", default, skip_serializing_if = "String::is_empty")]
    pub solver: String,

    #[serde(rename = "@iterations", skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    #[serde(rename = "@tolerance", skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,

    #[serde(rename = "@ls_iterations", skip_serializing_if = "Option::is_none")]
    pub ls_iterations: Option<i64>,

    #[serde(rename = "@ls_tolerance", skip_serializing_if = "Option::is_none")]
    pub ls_tolerance: Option<f64>,

    #[serde(rename = "@noslip_iterations", skip_serializing_if = "Option::is_none")]
    pub noslip_iterations: Option<i64>,

    #[serde(rename = "@noslip_tolerance", skip_serializing_if = "Option::is_none")]
    pub noslip_tolerance: Option<f64>,

    #[serde(rename = "@mpr_iterations", skip_serializing_if = "Option::is_none")]
    pub mpr_iterations: Option<i64>,

    #[serde(rename = "@mpr_tolerance", skip_serializing_if = "Option::is_none")]
    pub mpr_tolerance: Option<f64>,

    #[serde(rename = "@sdf_iterations", skip_serializing_if = "Option::is_none")]
    pub sdf_iterations: Option<i64>,

    #[serde(rename = "@sdf_initpoints", skip_serializing_if = "Option::is_none")]
    pub sdf_initpoints: Option<i64>,

    #[serde(rename = "@actuatorgroupdisable", default, skip_serializing_if = "Vec::is_empty")]
    pub actuatorgroupdisable: Vec<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<MujocoOptionFlag>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MujocoOptionFlag {
    #[serde(rename = "@constraint", default, skip_serializing_if = "String::is_empty")]
    pub constraint: String,

    #[serde(rename = "@equality", default, skip_serializing_if = "String::is_empty")]
    pub equality: String,

    #[serde(rename = "@frictionloss", default, skip_serializing_if = "String::is_empty")]
    pub frictionloss: String,

    #[serde(rename = "@limit", default, skip_serializing_if = "String::is_empty")]
    pub limit: String,

    #[serde(rename = "@contact", default, skip_serializing_if = "String::is_empty")]
    pub contact: String,

    #[serde(rename = "@passive", default, skip_serializing_if = "String::is_empty")]
    pub passive: String,

    #[serde(rename = "@gravity", default, skip_serializing_if = "String::is_empty")]
    pub gravity: String,

    #[serde(rename = "@clampctrl", default, skip_serializing_if = "String::is_empty")]
    pub clampctrl: String,

    #[serde(rename = "@warmstart", default, skip_serializing_if = "String::is_empty")]
    pub warmstart: String,

    #[serde(rename = "@filterparent", default, skip_serializing_if = "String::is_empty")]
    pub filterparent: String,

    #[serde(rename = "@actuation", default, skip_serializing_if = "String::is_empty")]
    pub actuation: String,

    #[serde(rename = "@refsafe", default, skip_serializing_if = "String::is_empty")]
    pub refsafe: String,

    #[serde(rename = "@sensor", default, skip_serializing_if = "String::is_empty")]
    pub sensor: String,

    #[serde(rename = "@midphase", default, skip_serializing_if = "String::is_empty")]
    pub midphase: String,

    #[serde(rename = "@eulerdamp", default, skip_serializing_if = "String::is_empty")]
    pub eulerdamp: String,

    #[serde(rename = "@override", default, skip_serializing_if = "String::is_empty")]
    pub r#override: String,

    #[serde(rename = "@energy", default, skip_serializing_if = "String::is_empty")]
    pub energy: String,

    #[serde(rename = "@fwdinv", default, skip_serializing_if = "String::is_empty")]
    pub fwdinv: String,

    #[serde(rename = "@invdiscrete", default, skip_serializing_if = "String::is_empty")]
    pub invdiscrete: String,

    #[serde(rename = "@sensornoise", default, skip_serializing_if = "String::is_empty")]
    pub sensornoise: String,

    #[serde(rename = "@multiccd", default, skip_serializing_if = "String::is_empty")]
    pub multiccd: String,

    #[serde(rename = "@island", default, skip_serializing_if = "String::is_empty")]
    pub island: String,
}
