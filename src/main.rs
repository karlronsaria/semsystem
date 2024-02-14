pub mod mydb;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = sqlx::mysql::MySqlConnectOptions::new()
        .host(mydb::HOST)
        .username(mydb::USER)
        .password(mydb::PASS)
        .database(mydb::DB);

    let (pool, _) = mydb::reset_db(&opts)
        .await?;

    for item in mydb::get::item_from_tagname(
        &pool,
        &mydb::DB,
        "finance"
    ).await.iter() {
        println!("{:#?}", item);
    }

    Ok(())
}

