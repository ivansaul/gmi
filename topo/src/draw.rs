use crate::core;
use crate::data;
use crate::data::get_points_by_label;
use crate::error::Result;
use crate::models::DataFrame;
use crate::models::ToDxfPoint;
use crate::models::TopoLabel;
use dxf::Drawing;
use dxf::Point;
use dxf::enums::AcadVersion;
use std::path::Path;
use strum::IntoEnumIterator;

pub fn get_drawing() -> Drawing {
    Drawing::new()
}

pub fn add_headers(msp: &mut Drawing) {
    msp.header.version = AcadVersion::R2000;
    msp.header.point_display_mode = 35;
    msp.header.point_display_size = 0.5;
}

pub fn draw_labels(drawing: &mut Drawing, df: &DataFrame) {
    let layer_name = Some(TopoLabel::PP.name());
    let data = data::get_labels_data(df);
    for (point, text) in data {
        core::draw_point(drawing, point.clone(), layer_name);
        core::draw_text(drawing, point.clone(), text, layer_name);
    }
}

pub fn draw_seccions(drawing: &mut Drawing, df: &DataFrame) {
    let layer_name = Some(TopoLabel::SEC.name());
    let data = data::get_sections_data(df);
    for points in data {
        core::draw_polyline(drawing, points, true, layer_name);
    }
}

pub fn draw_ltp(drawing: &mut Drawing, df: &DataFrame) {
    for label in TopoLabel::iter() {
        match label {
            TopoLabel::PP => continue,
            TopoLabel::SEC => continue,
            TopoLabel::D2 => continue,
            _ => {
                let data = data::get_points_by_label(df, label);
                if !data.is_empty() {
                    core::draw_polyline(drawing, data, false, Some(label.name()));
                }
            }
        }
    }
}

pub fn draw_2d(drawing: &mut Drawing, df: &DataFrame) {
    // PP + LABEL
    let layer_name = Some(TopoLabel::D2.name());
    let data = data::get_labels_data(df);
    for (point, text) in data {
        core::draw_point(drawing, point.to_2d(), layer_name);
        core::draw_text(drawing, point.to_2d(), text, layer_name);
    }

    // L
    let data = get_points_by_label(df, TopoLabel::L)
        .into_iter()
        .map(|point| point.to_2d())
        .collect::<Vec<Point>>();

    core::draw_polyline_lw(drawing, data, false, layer_name);
}

pub fn run(path: &Path) -> Result<()> {
    let df = data::read_csv(path)?;
    let mut drawing = get_drawing();
    add_headers(&mut drawing);
    draw_labels(&mut drawing, &df);
    draw_ltp(&mut drawing, &df);
    draw_seccions(&mut drawing, &df);
    draw_2d(&mut drawing, &df);

    let mut dxf_path = path.to_path_buf();
    dxf_path.set_extension("dxf");
    drawing.save_file(dxf_path)?;
    Ok(())
}
