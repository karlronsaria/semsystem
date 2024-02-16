pub mod mydb;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = sqlx::mysql::MySqlConnectOptions::new()
        .host(mydb::HOST)
        .username(mydb::USER)
        .password(mydb::PASS)
        .database(mydb::DB);

    let (pool, root) = mydb::reset_db(&opts)
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

    for tier in mydb::Query::builder(&pool)
        .db_name(mydb::DB)
        .from("item")
        .by("name")
        .when(&vec![
            mydb::When::Equal("?"),
            mydb::When::Like("?"),
            mydb::When::Match("?"),
        ])
        .sentinel("est uan sin ter")
        .build()
        .to_tiers::<mydb::Item>()
        .await
        .into_iter() {
            println!("{:#?}", tier);
        }

    for tier in mydb::Query::builder(&pool)
        .db_name(mydb::DB)
        .from("item")
        .by("name")
        .when(&vec![
            mydb::When::Equal("?"),
            mydb::When::Like("?"),
            mydb::When::Match("?"),
        ])
        .sentinel("uan sin ter ius")
        .build()
        .to_tiers::<mydb::Item>()
        .await
        .into_iter() {
            println!("{:#?}", tier);
        }


        let expected: Vec<mydb::When<Vec<mydb::Item>>> = vec![
            mydb::When::Equal(
                root.Items
                    .clone()
                    .into_iter()
                    .filter(|x| x.Name == "est uan sin ter")
                    .collect::<Vec<mydb::Item>>()
            ),

            mydb::When::Like(
                root.Items
                    .clone()
                    .into_iter()
                    .filter(|x| x.Name.starts_with("est uan sin ter"))
                    .collect::<Vec<mydb::Item>>()
            ),

            mydb::When::Match(
                root.Items
                    .clone()
                    .into_iter()
                    .filter(|x| regex::Regex::new(r"est uan sin ter")
                        .unwrap()
                        .is_match(&x.Name)
                    )
                    .collect::<Vec<mydb::Item>>()
            ),
        ];

    println!("{:#?}", expected);



    Ok(())
}

