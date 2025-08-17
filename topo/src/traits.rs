use crate::models::TopoRecord;
use dxf::Point;

pub trait ToTopo {
    fn convert(p: &TopoRecord) -> Self;
}

impl ToTopo for TopoRecord {
    fn convert(p: &TopoRecord) -> Self {
        p.clone()
    }
}

impl ToTopo for Point {
    fn convert(p: &TopoRecord) -> Self {
        p.to_3d_point()
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
