use database_migrations::get_database_client;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./sql");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let mut client = get_database_client().await?;

    let result = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    dbg!(result);

    Ok(())
}
