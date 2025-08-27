mod data_file;
mod database;
mod zip_iterator;

use crate::{
    data_file::DataRecordEnriched, database::copy_records_to_database, zip_iterator::ZipIterator,
};
use database_migrations::get_database_client;

const ZIP_ARCHIVE_PATH: &str = "./temp/names.zip";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    tracing_subscriber::fmt::init();

    let zip_iterator = ZipIterator::new(ZIP_ARCHIVE_PATH)?;

    let mut all_records = Vec::new();

    for data_file in zip_iterator {
        let single_year_records = data_file.records();

        tracing::debug!(
            "Deserialized data file {} with {} bytes and {} records",
            data_file.name,
            data_file.text.len(),
            single_year_records.len()
        );

        let single_year_records: Vec<DataRecordEnriched> = single_year_records
            .into_iter()
            .map(|record| DataRecordEnriched::new(record, data_file.year))
            .collect();

        all_records.extend(single_year_records);
    }

    let mut client = get_database_client().await?;
    copy_records_to_database(&mut client, all_records).await?;

    Ok(())
}
