pub mod myquery;
use crate::myquery::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let needle = "est uan sin ter";

    let opts = sqlx::mysql::MySqlConnectOptions::new()
        .host(HOST)
        .username(USER)
        .password(PASS)
        .database(DB);

    let (pool, root) = reset_db(&opts)
        .await?;

    for id in Query::builder(&pool)
        .from("tag")
        .when(&vec![When::Equal(("name", "?"))])
        .sentinel("claim")
        .build()
        .to::<Id::<Item>>()
        .await
        .into_iter() {
            println!("Id: {}", id.get());
        }

    for tier in Query::builder(&pool)
        .from("item")
        .when(&vec![
            When::Equal(("name", "?")),
            When::Like(("name", "?")),
            When::Match(("name", "?")),
        ])
        .sentinel(needle)
        .build()
        .to_tiers::<Item>()
        .await
        .into_iter() {
            println!("{:#?}", tier);
        }

    for tier in Query::builder(&pool)
        .from("item")
        .when(&vec![
            When::Equal(("name", "?")),
            When::Like(("name", "?")),
            When::Match(("name", "?")),
        ])
        .sentinel("uan sin ter ius")
        .build()
        .to_tiers::<Item>()
        .await
        .into_iter() {
            println!("{:#?}", tier);
        }

    let expected: Vec<When<Vec<Item>>> = vec![
        When::Equal(
            root.Items
                .clone()
                .into_iter()
                .filter(|x| x.Name == needle)
                .collect::<Vec<Item>>()
        ),

        When::Like(
            root.Items
                .clone()
                .into_iter()
                .filter(|x| x.Name.starts_with(needle))
                .collect::<Vec<Item>>()
        ),

        When::Match(
            root.Items
                .clone()
                .into_iter()
                .filter(|x| regex::Regex::new(needle)
                    .unwrap()
                    .is_match(&x.Name)
                )
                .collect::<Vec<Item>>()
        ),
    ];

    println!("{:#?}", expected);
    Ok(())
}

