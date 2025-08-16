use crate::error::Result;
use crate::models::{DataFrame, TopoLabel, TopoPoint};
use dxf::Point;
use std::fs;
use std::path::{Path, PathBuf};

pub fn read_csv(path: &Path) -> Result<Vec<TopoPoint>> {
    let temp_path: PathBuf = path.with_extension("tmp");

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(&temp_path)?;

    writer.write_record(["id", "x", "y", "z", "label"])?;

    for result in reader.records() {
        let record = result?;
        let filtrado: Vec<&str> = record.iter().take(5).collect();
        writer.write_record(&filtrado)?;
    }

    writer.flush()?;

    let mut reader = csv::Reader::from_path(&temp_path)?;
    let mut items: Vec<TopoPoint> = Vec::new();
    for result in reader.deserialize::<TopoPoint>() {
        items.push(result?);
    }

    clean_data(&mut items);
    fs::remove_file(&temp_path)?;
    Ok(items)
}

fn clean_data(df: &mut DataFrame) {
    df.iter_mut().for_each(|item| {
        item.id = item.id.to_uppercase();
        item.label = item.label.to_uppercase();
    });

    df.retain(|item| !item.id.is_empty() || !item.label.is_empty());
    df.dedup();
}

pub fn get_sections_data(df: &DataFrame) -> Vec<Vec<Point>> {
    use std::mem::take;

    let topo_points: Vec<&TopoPoint> = df
        .iter()
        .filter(|item| item.label == TopoLabel::SEC.name())
        .collect();

    let mut previous = 0;
    let mut group: Vec<Point> = Vec::new();
    let mut group_list: Vec<Vec<Point>> = Vec::new();

    for point in topo_points {
        let current = point.id.parse::<i32>().unwrap_or(0);
        if previous > current {
            group_list.push(take(&mut group));
        }
        group.push(point.to_entity_point_3d());
        previous = current;
    }
    if !group.is_empty() {
        group_list.push(take(&mut group));
    }
    group_list
}

pub fn get_labels_data(df: &DataFrame) -> Vec<(Point, String)> {
    get_topo_points_by_label(df, TopoLabel::PP)
        .iter()
        .map(|item| (item.to_entity_point_3d(), item.id.clone()))
        .collect()
}

pub fn get_points_by_label(df: &DataFrame, label: TopoLabel) -> Vec<Point> {
    df.iter()
        .filter(|item| item.label == label.name())
        .map(TopoPoint::to_entity_point_3d)
        .collect()
}

fn get_topo_points_by_label(df: &DataFrame, label: TopoLabel) -> Vec<TopoPoint> {
    df.iter()
        .filter(|item| item.label == label.name())
        .cloned()
        .collect()
}
