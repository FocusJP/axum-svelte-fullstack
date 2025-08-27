use futures_util::pin_mut;
use tokio_postgres::{Client, binary_copy::BinaryCopyInWriter, types::Type};

use crate::data_file::{DataRecord, DataRecordEnriched};

pub async fn copy_records_to_database(
    client: &mut Client,
    records: Vec<DataRecordEnriched>,
) -> anyhow::Result<()> {
    let transaction = client.transaction().await?;

    transaction.query("delete from ssa.baby_names", &[]).await?;

    let sink = transaction
        .copy_in("COPY ssa.baby_names (name, year, sex, count) FROM STDIN BINARY")
        .await?;

    let writer = BinaryCopyInWriter::new(sink, &[Type::TEXT, Type::INT4, Type::CHAR, Type::INT8]);

    pin_mut!(writer);

    for DataRecordEnriched {
        year,
        data_record: DataRecord { name, sex, count },
    } in records.iter()
    {
        let sex_i8 = &(*sex as i8);

        writer.as_mut().write(&[name, &year, sex_i8, count]).await?;
    }

    writer.finish().await?;

    transaction.commit().await?;

    Ok(())
}
