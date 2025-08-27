use crate::{
    database::DatabaseConnectionRef,
    error::AppError,
    model::ssa::{NameStat, request::GetNameStatsQuery, response::GetNameStatsResponse},
};

#[tracing::instrument(skip(database_connection))]
pub async fn get_name_stats(
    database_connection: DatabaseConnectionRef<'_>,
    GetNameStatsQuery {
        names,
        start_year,
        end_year,
    }: GetNameStatsQuery,
) -> Result<GetNameStatsResponse, AppError> {
    let rows = database_connection
        .query(
            "
        select name, sex, year, count
        from ssa.baby_names
        where year >= $1 and year <= $2 and name = ANY($3)
        ",
            &[&start_year, &end_year, &names],
        )
        .await?;

    let name_stats = rows
        .iter()
        .map(|row| {
            let name: String = row.try_get(0)?;
            let sex: String = row.try_get(1)?;
            let year: i32 = row.try_get(2)?;
            let count: i64 = row.try_get(3)?;

            let sex = sex.chars().next().unwrap_or('U');

            Ok(NameStat {
                name,
                sex,
                year,
                count,
            })
        })
        .collect::<Result<Vec<NameStat>, tokio_postgres::Error>>()?;

    Ok(GetNameStatsResponse(name_stats))
}
