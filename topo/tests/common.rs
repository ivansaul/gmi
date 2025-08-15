use topo::models::TopoPoint;

const T_LEN: u32 = 12;
const P_LEN: u32 = 14;
const L_LEN: u32 = 10;

const PP_LEN: u32 = 8;

pub const SEC_LEN: usize = 10;
pub const SEC_GROUPS: usize = 5;

pub fn get_test_topo_items() -> Vec<TopoPoint> {
    let mut csv: Vec<TopoPoint> = Vec::new();
    add_points(&mut csv);
    add_x(&mut csv, "L", L_LEN);
    add_x(&mut csv, "L", T_LEN);
    add_x(&mut csv, "P", P_LEN);
    add_secctions(&mut csv);
    csv
}

fn get_random_item(est: &str, des: &str) -> TopoPoint {
    let mut item = TopoPoint::default();
    item.id = est.into();
    item.label = des.into();
    item
}

fn add_secctions(csv: &mut Vec<TopoPoint>) {
    for _ in 1..=SEC_GROUPS {
        for j in 1..=SEC_LEN {
            csv.push(get_random_item(&j.to_string(), "SEC"));
        }
    }
}

fn add_points(csv: &mut Vec<TopoPoint>) {
    for i in 1..=PP_LEN {
        let mut item = TopoPoint::default();
        item.id = format!("A{i}");
        item.label = String::from("PP");
        csv.push(item);
        if i % 2 == 0 {
            let mut item = TopoPoint::default();
            item.id = format!("R{}", i * 2);
            item.label = String::from("");
            csv.push(item);
        }
    }
}

fn add_x(csv: &mut Vec<TopoPoint>, desc: &str, count: u32) {
    for i in 1..=count {
        csv.push(get_random_item(&i.to_string(), desc));
    }
}
