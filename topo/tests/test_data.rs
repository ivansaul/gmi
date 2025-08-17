mod common;
use crate::common::*;
use topo::{
    data::*,
    models::{TopoRecord, TopoTag},
};

#[test]
fn test_get_section_groups_multiple_groups() {
    let builder = TopoTestBuilder::new().sec_groups(3).sec_len(4);
    let data = builder.build();

    let sections = get_section_groups(&data);

    assert_eq!(sections.len(), builder.get_sec_groups());
    for group in sections {
        assert_eq!(group.len(), builder.get_sec_len());
    }
}

#[test]
fn test_get_section_groups_empty() {
    let builder = TopoTestBuilder::new();
    let data = builder.build();

    let sections = get_section_groups(&data);
    assert!(sections.is_empty());
}

#[test]
fn test_filter_by_tag() {
    let builder = TopoTestBuilder::new().p_len(4).t_len(3);
    let data = builder.build();

    let l_records: Vec<TopoRecord> = filter_by_tag(&data, &TopoTag::L);
    let p_records: Vec<TopoRecord> = filter_by_tag(&data, &TopoTag::P);
    let t_records: Vec<TopoRecord> = filter_by_tag(&data, &TopoTag::T);
    let pp_records: Vec<TopoRecord> = filter_by_tag(&data, &TopoTag::PP);

    assert_eq!(l_records.len(), builder.get_l_len());
    assert_eq!(p_records.len(), builder.get_p_len());
    assert_eq!(t_records.len(), builder.get_t_len());
    assert_eq!(pp_records.len(), builder.get_pp_len());
}

#[test]
fn test_get_df_unique_tag() {
    let builder = TopoTestBuilder::new()
        .pp_len(2)
        .l_len(1)
        .t_len(1)
        .p_len(1)
        .sec_groups(1)
        .sec_len(1);

    let data = builder.build();
    let tags = get_df_unique_tags(&data);

    assert!(tags.contains(&"PP".to_string()));
    assert!(tags.contains(&"L".to_string()));
    assert!(tags.contains(&"T".to_string()));
    assert!(tags.contains(&"P".to_string()));
    assert!(tags.contains(&"SEC".to_string()));
}
