use serde::Deserialize;

pub struct DataFile {
    pub name: String,
    pub text: String,
    pub year: i32,
}

#[derive(Debug, Deserialize)]
pub struct DataRecord {
    pub name: String,
    pub sex: char,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
pub struct DataRecordEnriched {
    pub data_record: DataRecord,
    pub year: i32,
}

impl DataFile {
    pub fn records(&self) -> Vec<DataRecord> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(self.text.as_bytes());

        rdr.deserialize().map(|item| item.unwrap()).collect()
    }
}

impl DataRecordEnriched {
    pub fn new(data_record: DataRecord, year: i32) -> Self {
        Self { data_record, year }
    }
}
