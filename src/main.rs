pub mod myquery;
use crate::myquery::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // let opts = sqlx::mysql::MySqlConnectOptions::new()
    //     .host(HOST)
    //     .username(USER)
    //     .password(PASS)
    //     .database(DB);

    // let (pool, _) = reset_db(&opts)
    //     .await?;

    // use sqlx::MySqlPool;
    // use chrono::NaiveDate;

    // let opts = sqlx::mysql::MySqlConnectOptions::new()
    //     .host(HOST)
    //     .username(USER)
    //     .password(PASS)
    //     .database(DB);

    // let json = std::fs::read_to_string(&ITEM_JSON_PATH)
    //     .expect(&format!("Error: Failed to find path '{}'", ITEM_JSON_PATH));

    // let root: DbRoot = serde_json::from_str(&json)
    //     .unwrap();

    // let lower: &str = "2022-12-31";

    // let lower_as_date: NaiveDate =
    //     NaiveDate::parse_from_str(lower, "%Y-%m-%d")
    //         .expect("You entered the test string or date format incorrectly.");

    // if let Ok(pool) = MySqlPool::connect_with(opts.to_owned()).await {
    //     let expected: Vec<Item> = root.Items
    //         .into_iter()
    //         .filter(|x| {
    //             for date in &x.Dates {
    //                 if date > &lower_as_date {
    //                     return true;
    //                 }
    //             }

    //             false
    //         })
    //         .collect();

    //     let actual = Query::builder(&pool)
    //         .from("date")
    //         .when(&vec![When::Greater(("date", lower))])
    //         .build()
    //         .to_complete_items()
    //         .await;

    //     for actual in actual {
    //         println!("{:#?}", actual);
    //     }
    // }

    Ok(())
}

