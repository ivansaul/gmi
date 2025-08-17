use crate::models::TopoLabel;
use dxf::{Drawing, LwPolylineVertex};
use dxf::{Point, entities::*};

pub fn draw_text(drawing: &mut Drawing, point: Point, text: String, layer_name: Option<&str>) {
    let mut entity = Entity::new(EntityType::Text(Text {
        value: text,
        location: point,
        ..Default::default()
    }));

    if let Some(layer_name) = layer_name {
        ensure_layer_is_present(drawing, &layer_name);
        entity.common.layer = layer_name.into();
    }

    drawing.add_entity(entity);
}

pub fn draw_point(drawing: &mut Drawing, point: Point, layer_name: Option<&str>) {
    let mut entity = Entity::new(EntityType::ModelPoint(ModelPoint::new(point)));

    if let Some(layer_name) = layer_name {
        ensure_layer_is_present(drawing, &layer_name);
        entity.common.layer = layer_name.into();
    }

    drawing.add_entity(entity);
}

pub fn draw_polyline_lw(
    drawing: &mut Drawing,
    points: Vec<Point>,
    close: bool,
    layer_name: Option<&str>,
) {
    let mut polyline = LwPolyline::default();
    polyline.set_is_closed(close);
    points.iter().for_each(|point| {
        polyline.vertices.push(LwPolylineVertex {
            x: point.x,
            y: point.y,
            ..Default::default()
        });
    });

    let mut entity = Entity::new(EntityType::LwPolyline(polyline));

    if let Some(layer_name) = layer_name {
        ensure_layer_is_present(drawing, &layer_name);
        entity.common.layer = layer_name.into();
    }

    drawing.add_entity(entity);
}

pub fn draw_polyline(
    drawing: &mut Drawing,
    points: Vec<Point>,
    close: bool,
    layer_name: Option<&str>,
) {
    let mut polyline = Polyline::default();
    polyline.set_is_3d_polyline(true);
    polyline.set_is_closed(close);
    points.iter().for_each(|point| {
        polyline.add_vertex(
            drawing,
            Vertex {
                location: point.clone(),
                ..Default::default()
            },
        );
    });

    let mut entity = Entity::new(EntityType::Polyline(polyline));

    if let Some(layer_name) = layer_name {
        ensure_layer_is_present(drawing, &layer_name);
        entity.common.layer = layer_name.into();
    }

    drawing.add_entity(entity);
}

fn ensure_layer_is_present(drawing: &mut Drawing, layer_name: &str) {
    if !drawing.layers().any(|l| l.name == layer_name) {
        let layer = TopoLabel::from_str(layer_name).layer();
        drawing.add_layer(layer);
    }
}
