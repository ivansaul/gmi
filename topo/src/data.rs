use crate::error::Result;
use crate::models::{DataFrame, TopoRecord, TopoTag};
use crate::traits::ToTopo;
use defer::defer;
use dxf::Point;
use std::fs;
use std::path::{Path, PathBuf};

pub fn read_csv(path: &Path) -> Result<Vec<TopoRecord>> {
    let temp_path: PathBuf = path.with_extension("tmp");
    defer!(fs::remove_file(&temp_path).unwrap_or_default());

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(&temp_path)?;

    writer.write_record(["id", "x", "y", "z", "tag"])?;

    for result in reader.records() {
        let record = result?;
        let filtered: Vec<&str> = record.iter().take(5).collect();
        writer.write_record(&filtered)?;
    }

    writer.flush()?;

    let mut reader = csv::Reader::from_path(&temp_path)?;
    let mut records: Vec<TopoRecord> = Vec::new();
    for record in reader.deserialize::<TopoRecord>() {
        records.push(record?);
    }

    clean_data(&mut records);
    Ok(records)
}

pub fn clean_data(df: &mut DataFrame) {
    df.iter_mut().for_each(|record| {
        record.id = record.id.to_uppercase();
        record.tag = record.tag.to_uppercase();
    });

    df.retain(|record| !record.id.is_empty() && !record.tag.is_empty());
    df.dedup();
}

pub fn get_section_groups(df: &DataFrame) -> Vec<Vec<Point>> {
    use std::mem::take;

    let records: Vec<&TopoRecord> = df
        .iter()
        .filter(|record| record.tag == TopoTag::SEC.name())
        .collect();

    let mut previous = 0;
    let mut group: Vec<Point> = Vec::new();
    let mut group_list: Vec<Vec<Point>> = Vec::new();

    for record in records {
        let current = record.id.parse::<i32>().unwrap_or(0);
        if previous > current {
            group_list.push(take(&mut group));
        }
        group.push(record.to_3d_point());
        previous = current;
    }
    if !group.is_empty() {
        group_list.push(take(&mut group));
    }
    group_list
}

pub fn group_by_tag<T>(df: &DataFrame, tag: &TopoTag) -> Vec<Vec<T>>
where
    T: ToTopo,
{
    use std::mem::take;
    let mut group: Vec<T> = Vec::new();
    let mut group_list: Vec<Vec<T>> = Vec::new();

    for record in df.iter() {
        if record.tag == tag.name() {
            group.push(T::convert(record));
        } else if !group.is_empty() {
            group_list.push(take(&mut group));
        }
    }
    if !group.is_empty() {
        group_list.push(take(&mut group));
    }
    group_list
}

pub fn filter_by_tag<T>(df: &DataFrame, tag: &TopoTag) -> Vec<T>
where
    T: ToTopo,
{
    df.iter()
        .filter(|record| record.tag == tag.name())
        .map(T::convert)
        .collect()
}

pub fn get_df_unique_tags(df: &DataFrame) -> Vec<String> {
    use std::collections::HashSet;
    df.iter()
        .map(|record| record.tag.clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>()
}
