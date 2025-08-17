use topo::{
    data::clean_data,
    models::{DataFrame, TopoRecord},
};

#[derive(Default, Clone, Copy)]
pub struct TopoTestBuilder {
    pp_len: usize,
    l_len: usize,
    t_len: usize,
    p_len: usize,
    sec_len: usize,
    sec_groups: usize,
}

impl TopoTestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pp_len(mut self, len: usize) -> Self {
        self.pp_len = len;
        self
    }

    pub fn l_len(mut self, len: usize) -> Self {
        self.l_len = len;
        self
    }

    pub fn t_len(mut self, len: usize) -> Self {
        self.t_len = len;
        self
    }

    pub fn p_len(mut self, len: usize) -> Self {
        self.p_len = len;
        self
    }

    pub fn sec_len(mut self, len: usize) -> Self {
        self.sec_len = len;
        self
    }

    pub fn sec_groups(mut self, groups: usize) -> Self {
        self.sec_groups = groups;
        self
    }

    pub fn build(self) -> DataFrame {
        let mut df = Vec::new();
        self.add_points(&mut df);
        self.add_x(&mut df, "L", self.l_len);
        self.add_x(&mut df, "T", self.t_len);
        self.add_x(&mut df, "P", self.p_len);
        self.add_sections(&mut df);
        clean_data(&mut df);
        df
    }

    fn add_points(&self, df: &mut DataFrame) {
        for i in 0..self.pp_len {
            let mut record = TopoRecord::default();
            record.id = format!("A{i}");
            record.tag = "PP".into();
            df.push(record);

            if i % 2 == 0 {
                let mut record = TopoRecord::default();
                record.id = format!("R{}", i * 2);
                record.tag = "".into();
                df.push(record);
            }
        }
    }

    fn add_x(&self, df: &mut DataFrame, tag: &str, count: usize) {
        for i in 0..count {
            let mut record = TopoRecord::default();
            record.id = i.to_string();
            record.tag = tag.into();
            df.push(record);
        }
    }

    fn add_sections(&self, df: &mut DataFrame) {
        for _ in 0..self.sec_groups {
            for j in 0..self.sec_len {
                let mut record = TopoRecord::default();
                record.id = j.to_string();
                record.tag = "SEC".into();
                df.push(record);
            }
        }
    }
}

impl TopoTestBuilder {
    pub fn get_pp_len(&self) -> usize {
        self.pp_len
    }
    pub fn get_l_len(&self) -> usize {
        self.l_len
    }
    pub fn get_t_len(&self) -> usize {
        self.t_len
    }
    pub fn get_p_len(&self) -> usize {
        self.p_len
    }
    pub fn get_sec_len(&self) -> usize {
        self.sec_len
    }
    pub fn get_sec_groups(&self) -> usize {
        self.sec_groups
    }
}
