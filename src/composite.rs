use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Composite {
    #[serde(rename = "@prefix", default, skip_serializing_if = "String::is_empty")]
    pub prefix: String,

    #[serde(rename = "@type")]
    pub r#type: String,

    #[serde(rename = "@count")]
    pub count: [i64; 3],

    #[serde(rename = "@spacing")]
    pub spacing: f64,

    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<[f64; 3]>,

    #[serde(rename = "@flatinertia", skip_serializing_if = "Option::is_none")]
    pub flatinertia: Option<f64>,

    #[serde(rename = "@solrefsmooth", skip_serializing_if = "Option::is_none")]
    pub solrefsmooth: Option<[f64; 2]>,

    #[serde(rename = "@solimpsmooth", default, skip_serializing_if = "Vec::is_empty")]
    pub solimpsmooth: Vec<f64>,

    #[serde(rename = "@vertex", default, skip_serializing_if = "Vec::is_empty")]
    pub vertex: Vec<f64>,

    #[serde(rename = "@face", default, skip_serializing_if = "Vec::is_empty")]
    pub face: Vec<f64>,

    #[serde(rename = "@initial", default, skip_serializing_if = "String::is_empty")]
    pub initial: String,

    #[serde(rename = "@curve", default, skip_serializing_if = "String::is_empty")]
    pub curve: String,

    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<[i64; 3]>,

    #[serde(default)]
    pub joint: Vec<CompositeJoint>,

    #[serde(default)]
    pub tendon: Vec<CompositeTendon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geom: Option<CompositeGeom>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<CompositeSite>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<CompositeSkin>,

    #[serde(default)]
    pub pin: Vec<CompositePin>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<crate::Plugin>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompositeJoint {
    #[serde(rename = "@kind")]
    pub kind: String,

    #[serde(rename = "@solreffix", skip_serializing_if = "Option::is_none")]
    pub solreffix: Option<[f64; 2]>,

    #[serde(rename = "@solimpfix", default, skip_serializing_if = "Vec::is_empty")]
    pub solimpfix: Vec<f64>,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@type", default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(rename = "@axis", skip_serializing_if = "Option::is_none")]
    pub axis: Option<[f64; 3]>,

    #[serde(rename = "@stiffness", skip_serializing_if = "Option::is_none")]
    pub stiffness: Option<f64>,

    #[serde(rename = "@damping", skip_serializing_if = "Option::is_none")]
    pub damping: Option<f64>,

    #[serde(rename = "@armature", skip_serializing_if = "Option::is_none")]
    pub armature: Option<f64>,

    #[serde(rename = "@limited", default, skip_serializing_if = "String::is_empty")]
    pub limited: String,

    #[serde(rename = "@solreflimit", skip_serializing_if = "Option::is_none")]
    pub solreflimit: Option<[f64; 2]>,

    #[serde(rename = "@solimplimit", default, skip_serializing_if = "Vec::is_empty")]
    pub solimplimit: Vec<f64>,

    #[serde(rename = "@solreffriction", skip_serializing_if = "Option::is_none")]
    pub solreffriction: Option<[f64; 2]>,

    #[serde(rename = "@solimpfriction", default, skip_serializing_if = "Vec::is_empty")]
    pub solimpfriction: Vec<f64>,

    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<[f64; 2]>,

    #[serde(rename = "@margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,

    #[serde(rename = "@frictionloss", skip_serializing_if = "Option::is_none")]
    pub frictionloss: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompositeTendon {
    #[serde(rename = "@kind")]
    pub kind: String,

    #[serde(rename = "@solreffix", skip_serializing_if = "Option::is_none")]
    pub solreffix: Option<[f64; 2]>,

    #[serde(rename = "@solimpfix", default, skip_serializing_if = "Vec::is_empty")]
    pub solimpfix: Vec<f64>,

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
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompositeGeom {
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

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompositeSite {
    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<f64>>,

    #[serde(rename = "@material", default, skip_serializing_if = "String::is_empty")]
    pub material: String,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompositeSkin {
    #[serde(rename = "@texcoord", skip_serializing_if = "Option::is_none")]
    pub texcoord: Option<bool>,

    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,

    #[serde(rename = "@material", default, skip_serializing_if = "String::is_empty")]
    pub material: String,

    #[serde(rename = "@rgba", skip_serializing_if = "Option::is_none")]
    pub rgba: Option<[f64; 4]>,

    #[serde(rename = "@inflate", skip_serializing_if = "Option::is_none")]
    pub inflate: Option<f64>,

    #[serde(rename = "@subgrid", skip_serializing_if = "Option::is_none")]
    pub subgrid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompositePin {
    #[serde(rename = "@coord")]
    pub coord: [i64; 2],
}
