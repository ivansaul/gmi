use dxf::{Color, Point, tables::Layer};
use serde::{Deserialize, Deserializer};

pub type DataFrame = Vec<TopoRecord>;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct TopoRecord {
    pub id: String,
    x: f64,
    y: f64,
    z: f64,
    pub tag: String,
}

impl TopoRecord {
    pub fn to_3d_point(&self) -> Point {
        Point::new(self.x, self.y, self.z)
    }
    pub fn to_2d_point(&self) -> Point {
        Point::new(self.x, self.y, 0.0)
    }
}

#[derive(Debug, Clone)]
pub enum TopoTag {
    L,
    T,
    P,
    PP,
    SEC,
    L1,
    L2,
    L3,
    L4,
    D2,
    Unknown(String),
}

impl TopoTag {
    pub fn layer(&self) -> Layer {
        match self {
            TopoTag::L => self.layer_from_color(1),
            TopoTag::T => self.layer_from_color(6),
            TopoTag::P => self.layer_from_color(2),
            TopoTag::PP => self.layer_from_color(188),
            TopoTag::SEC => self.layer_from_color(7),
            TopoTag::L1 => self.layer_from_color(4),
            TopoTag::L2 => self.layer_from_color(3),
            TopoTag::L3 => self.layer_from_color(5),
            TopoTag::L4 => self.layer_from_color(40),
            TopoTag::D2 => Layer {
                name: self.name().into(),
                color: Color::from_index(234),
                line_type_name: "DASHED".into(),
                is_layer_on: false,
                // TO-DO: line_weight: 0.1
                ..Default::default()
            },
            TopoTag::Unknown(label) => Layer {
                name: label.into(),
                ..Default::default()
            },
        }
    }

    fn layer_from_color(&self, index: u8) -> Layer {
        Layer {
            name: self.name().into(),
            color: Color::from_index(index),
            ..Default::default()
        }
    }
}

impl TopoTag {
    pub fn name(&self) -> &str {
        match self {
            TopoTag::L => "L",
            TopoTag::T => "T",
            TopoTag::P => "P",
            TopoTag::PP => "PP",
            TopoTag::SEC => "SEC",
            TopoTag::L1 => "L1",
            TopoTag::L2 => "L2",
            TopoTag::L3 => "L3",
            TopoTag::L4 => "L4",
            TopoTag::D2 => "2D",
            TopoTag::Unknown(label) => label,
        }
    }
}

impl TopoTag {
    pub fn from_str(s: &str) -> Self {
        match s {
            "L" => TopoTag::L,
            "T" => TopoTag::T,
            "P" => TopoTag::P,
            "PP" => TopoTag::PP,
            "SEC" => TopoTag::SEC,
            "L1" => TopoTag::L1,
            "L2" => TopoTag::L2,
            "L3" => TopoTag::L3,
            "L4" => TopoTag::L4,
            "2D" => TopoTag::D2,
            other => TopoTag::Unknown(other.into()),
        }
    }
}

impl<'de> Deserialize<'de> for TopoTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(TopoTag::from_str(&s))
    }
}
