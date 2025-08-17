use crate::core;
use crate::data;
use crate::data::filter_by_tag;
use crate::data::get_df_unique_tags;
use crate::error::Result;
use crate::models::DataFrame;
use crate::models::TopoRecord;
use crate::models::TopoTag;
use dxf::Drawing;
use dxf::Point;
use dxf::enums::AcadVersion;
use std::path::Path;

fn get_drawing() -> Drawing {
    Drawing::new()
}

fn add_headers(drawing: &mut Drawing) {
    drawing.header.version = AcadVersion::R12;
    drawing.header.point_display_mode = 35;
    drawing.header.point_display_size = 0.5;
}

fn draw_pp(drawing: &mut Drawing, df: &DataFrame) {
    let tag = TopoTag::PP;
    let layer = Some(tag.name());
    data::filter_by_tag::<TopoRecord>(df, &tag)
        .into_iter()
        .for_each(|r| core::draw_label(drawing, r.to_3d_point(), r.tag, layer));
}

fn draw_sec(drawing: &mut Drawing, df: &DataFrame) {
    let layer = Some(TopoTag::SEC.name());
    data::get_section_groups(df)
        .into_iter()
        .for_each(|v| core::draw_polyline(drawing, v, true, layer));
}

fn draw_ltpx(drawing: &mut Drawing, df: &DataFrame) {
    get_df_unique_tags(df)
        .iter()
        .map(|x| TopoTag::from_str(x))
        .for_each(|tag| match tag {
            TopoTag::PP => {}
            TopoTag::SEC => {}
            TopoTag::D2 => {}
            _ => _draw_ltpx(drawing, df, &tag),
        });
}

fn draw_2d(drawing: &mut Drawing, df: &DataFrame) {
    let layer = Some(TopoTag::D2.name());
    data::filter_by_tag::<TopoRecord>(df, &TopoTag::PP)
        .into_iter()
        .for_each(|r| core::draw_label(drawing, r.to_2d_point(), r.tag, layer));

    let vertices = filter_by_tag::<Point>(df, &TopoTag::L);
    core::draw_lw_polyline(drawing, vertices, false, layer);
}

fn _draw_ltpx(drawing: &mut Drawing, df: &DataFrame, tag: &TopoTag) {
    let groups = data::group_by_tag::<TopoRecord>(df, &tag);
    for group in groups {
        if group.len() == 1 {
            let record = group.first().unwrap();
            core::draw_label(
                drawing,
                record.to_3d_point(),
                record.tag.clone(),
                Some(tag.name()),
            );
            continue;
        }
        let vertices = group.iter().map(|r| r.to_3d_point()).collect();
        core::draw_polyline(drawing, vertices, false, Some(tag.name()));
    }
}

pub fn draw(path: &Path) -> Result<()> {
    let df = data::read_csv(path)?;
    let mut drawing = get_drawing();
    add_headers(&mut drawing);
    draw_pp(&mut drawing, &df);
    draw_sec(&mut drawing, &df);
    draw_2d(&mut drawing, &df);
    draw_ltpx(&mut drawing, &df);

    let mut dxf_path = path.to_path_buf();
    dxf_path.set_extension("dxf");
    drawing.normalize();
    drawing.save_file(dxf_path)?;
    Ok(())
}
