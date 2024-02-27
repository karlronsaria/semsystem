pub mod myquery;
pub mod app;

use clap::Parser;

#[allow(unused_imports)]
use crate::myquery::*;

#[allow(unused_imports)]
use crate::app::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    match load_stdin() {
        Ok(lines) => {
            let json: String = lines.join("\n");

            match serde_json::from_str::<Vec<Item>>(&json) {
                Ok(items) => println!("{:#?}", items),
                Err(message) => println!("Error: {message}"),
            }
        },

        Err(_) => {},
    }

    let args = MyCli::parse();
    println!("{:#?}", args);


    let opts = sqlx::mysql::MySqlConnectOptions::new()
        .host(HOST)
        .username(USER)
        .password(PASS)
        .database(DB);

    let (pool, _) = reset_db(&opts)
        .await?;

    for needle in ["finance", "adventure"] {
        println!("{:#?}", new_tag(&pool, needle).await);
    }





    // let opts = sqlx::mysql::MySqlConnectOptions::new()
    //     .host(HOST)
    //     .username(USER)
    //     .password(PASS)
    //     .database(DB);

    // let (pool, _) = reset_db(&opts)
    //     .await?;

    // use sqlx::MySqlPool;
    // use chrono::NaiveDateTime;

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

    // let lower_as_date: NaiveDateTime =
    //     NaiveDateTime::parse_from_str(lower, "%Y-%m-%d")
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

    // let json = std::fs::read_to_string(&ITEM_JSON_PATH)
    //     .expect(&format!("Error: Failed to find path '{}'", ITEM_JSON_PATH));

    // let root: DbRoot = serde_json::from_str(&json)
    //     .unwrap();

    // let temp: String = myrow_to_dbremove("item", vec![2, 3, 5, 1]);
    // println!("{temp}");

    // let temp: String = myrow_to_dbdissociate("item", "tag", 5, 2);
    // println!("{temp}");

    // let temp: String = myrow_to_dbassociate("item", "tag", 5, 2);
    // println!("{temp}");




    // todo

    // let opts = sqlx::mysql::MySqlConnectOptions::new()
    //     .host(HOST)
    //     .username(USER)
    //     .password(PASS)
    //     .database(DB);

    // use sqlx::Row;

    // if let Ok((pool, _)) = reset_db(&opts).await {
    //     match sqlx::query(format!(" SELECT Levenshtein('what', 'tahw');").as_str())
    //         .fetch_all(&pool)
    //         .await {
    //             Err(msg) => {
    //                 eprintln!("Error: {}", msg);
    //             },

    //             Ok(rows) => {
    //                 let what = rows.into_iter()
    //                     .map(|row| row.get(0))
    //                     .collect::<Vec<i32>>();

    //                 println!("{}", what[0]);
    //             }
    //         };

    //     let query = format!(
    //         " SELECT *, Levenshtein(name, '?') AS dist FROM item ORDER BY dist, `Id`;"
    //     );

    //     let query = sqlx::query(query.as_str());

    //     match query
    //         .bind("Finance")
    //         .fetch_all(&pool)
    //         .await {
    //             Err(msg) => {
    //                 eprintln!("Error: {}", msg);
    //             },

    //             Ok(rows) => {
    //                 let what = rows.into_iter()
    //                     .map(|row| row.get("dist"))
    //                     .collect::<Vec<i32>>();

    //                 println!("{}", what[0]);
    //             }
    //         };

    //     for needle in [
    //         "Finance Statement 00",
    //         "Auto Claim - Januayr 2023.pdf",
    //         "zzzzzzzzzzzzzzzzzzzzzz",
    //     ] {
    //         println!("\n[");

    //         for x in Query::builder(&pool)
    //             .from("item")
    //             .build()
    //             .to_fuzzy::<Item>(
    //                 "name",
    //                 needle,
    //                 None,
    //             )
    //             .await
    //         {
    //             match x {
    //                 Dist::<Item> { distance, payload } => {
    //                     println!("{distance}: {} {}", payload.Id, payload.Name);
    //                 }
    //             }
    //         }

    //         println!("]\n");
    //     }
    // }

    Ok(())
}

