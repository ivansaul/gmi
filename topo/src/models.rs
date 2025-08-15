use dxf::{Color, Point, tables::Layer};
use serde::Deserialize;
use strum::{AsRefStr, EnumIter, EnumString};

pub type DataFrame = Vec<TopoPoint>;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct TopoPoint {
    pub id: String,
    x: f64,
    y: f64,
    z: f64,
    pub label: String,
}

impl TopoPoint {
    pub fn to_entity_point_3d(&self) -> Point {
        Point::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, EnumIter, EnumString, AsRefStr)]
pub enum TopoLabel {
    L,
    T,
    P,
    PP,
    SEC,
    L1,
    L2,
    L3,
    L4,
    #[strum(serialize = "2D")]
    D2,
}

impl TopoLabel {
    pub fn name(&self) -> &str {
        match self {
            TopoLabel::D2 => "2D",
            _ => self.as_ref(),
        }
    }

    pub fn layer(&self) -> Layer {
        match self {
            TopoLabel::L => self.layer_from_color(1),
            TopoLabel::T => self.layer_from_color(6),
            TopoLabel::P => self.layer_from_color(2),
            TopoLabel::PP => self.layer_from_color(188),
            TopoLabel::SEC => self.layer_from_color(7),
            TopoLabel::L1 => self.layer_from_color(4),
            TopoLabel::L2 => self.layer_from_color(3),
            TopoLabel::L3 => self.layer_from_color(5),
            TopoLabel::L4 => self.layer_from_color(40),
            TopoLabel::D2 => Layer {
                name: self.name().into(),
                color: Color::from_index(234),
                line_type_name: "DASHED".into(),
                is_layer_on: false,
                // TO-DO: line_weight: 0.1
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

pub trait ToDxfPoint {
    fn to_2d(&self) -> Point;
}

impl ToDxfPoint for Point {
    fn to_2d(&self) -> Self {
        Point::new(self.x, self.y, 0.0)
    }
}
