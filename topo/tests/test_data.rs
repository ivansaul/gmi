mod common;
use crate::common::*;
use topo::data::*;

#[test]
fn test_get_sections() {
    let test_data = get_test_topo_items();
    let sections = get_sections_data(&test_data);
    assert_eq!(sections.len(), SEC_GROUPS);
    assert_eq!(sections[0].len(), SEC_LEN);
}
