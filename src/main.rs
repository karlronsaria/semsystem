pub mod mydb;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let opts = sqlx::mysql::MySqlConnectOptions::new()
        .host(mydb::HOST)
        .username(mydb::USER)
        .password(mydb::PASS)
        .database(mydb::DB);

    let (pool, _) = mydb::reset_db(&opts)
        .await?;

    for id in mydb::Query::builder(&pool)
        .db_name(mydb::DB)
        .from("tag")
        .by("name")
        .when(&vec![mydb::When::Equal("?")])
        .sentinel("claim")
        .build()
        .to::<mydb::Id::<mydb::Item>>()
        .await
        .into_iter() {
            println!("Id: {}", id.get());
        }

    // for item in mydb::get::item_from_tagname(
    //     &pool,
    //     &mydb::DB,
    //     "finance"
    // ).await.iter() {
    //     println!("{:#?}", item);
    // }

    Ok(())
}

