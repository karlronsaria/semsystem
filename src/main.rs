pub mod myquery;
pub mod app;

use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pipe = match app::load_stdin() {
        Ok(lines) => {
            let json: String = lines.join("\n");

            match serde_json::from_str::<Vec<myquery::Item>>(&json) {
                Ok(items) => Some(items),
                Err(_) => match serde_json::from_str::<myquery::Item>(&json) {
                    Ok(item) => Some(vec![item]),
                    Err(message) => {
                        eprintln!("Error: {message}");
                        None
                    },
                }
            }
        },

        Err(_) => None,
    };

    let opts = sqlx::mysql::MySqlConnectOptions::new()
        .host(myquery::HOST)
        .username(myquery::USER)
        .password(myquery::PASS)
        .database(myquery::DB);

    let (pool, _) = myquery::reset_db(&opts)
        .await?;

    let args = app::Cli::parse();

    match args.command {
        None => {},
        Some(ref what) => match what {
            app::Command::Search(query) => {
                let needle = query.strings.join("|");

                let items = myquery::Query::new(&pool)
                    .from("item")
                    .when(&vec![
                        myquery::When::Like(("name", &needle))
                    ])
                    .build()
                    .to_complete_items()
                    .await;

                let mut minus: Vec<i32> = vec![];

                println!("\nItems with names like: \"{needle}*\"");

                for item in items {
                    println!("  {} {}", item.Id, item.Name);

                    for subneedle in &query.strings {
                        let subneedle = subneedle.to_lowercase();

                        if item.Tags.contains(&subneedle) {
                            println!("    Has tag: {subneedle}");
                        }
                    }

                    minus.push(item.Id);
                }

                let items = myquery::Query::new(&pool)
                    .from("item")
                    .when(&vec![
                        myquery::When::Match(("name", &needle))
                    ])
                    .minus(Some(&minus))
                    .build()
                    .to::<myquery::Item>()
                    .await;

                println!("\nItems with names matching: /{needle}/");

                for item in items {
                    println!("  {} {}", item.Id, item.Name);

                    for subneedle in &query.strings {
                        let subneedle = subneedle.to_lowercase();

                        if item.Tags.contains(&subneedle) {
                            println!("    Has tag: {subneedle}");
                        }
                    }

                    minus.push(item.Id);
                }

                let items = myquery::Query::new(&pool)
                    .from("tag")
                    .when(&vec![
                        myquery::When::Equal(("name", &needle.to_lowercase()))
                    ])
                    .minus(Some(&minus))
                    .build()
                    .to::<myquery::Item>()
                    .await;

                println!("\nItems with tag: \"{}\"", needle.to_lowercase());

                for item in items {
                    println!("  {} {}", item.Id, item.Name);
                    minus.push(item.Id);
                }

                for search_str in &query.strings {
                    println!("\nFuzzy name search with: \"{search_str}\"");

                    let fuzzy = myquery::Query::new(&pool)
                        .from("item")
                        .minus(Some(&minus))
                        .build()
                        .to_fuzzy::<myquery::Item>(
                            "name",
                            &search_str,
                        )
                        .await;

                    for dist in fuzzy {
                        println!(
                            "  {} {} {}",
                            dist.distance,
                            dist.payload.Id,
                            dist.payload.Name
                        );

                        minus.push(dist.payload.Id);
                    }
                }

                println!();

    //     let actual = Query::builder(&pool)
    //         .from("date")
    //         .when(&vec![When::Greater(("date", lower))])
    //         .build()
    //         .to_complete_items()
    //         .await;
            },

            app::Command::Item(get_item) => match get_item {
                app::GetItem::Id { id } => println!("Get item by id: {id}"),
                app::GetItem::Name { name } => println!("Get item by name: {name}"),
            },

            app::Command::Tag { names } => for name in names {
                println!("Tag: {name}");
            },

            app::Command::Date {
                r#in,
                before,
                after,
                atleast,
                atmost,
            } => {
                match r#in {
                    None => {},
                    Some(x) => for y in x {
                        println!("In: {y}");
                    },
                };

                match before {
                    None => {},
                    Some(x) => println!("Before: {x}"),
                };

                match after {
                    None => {},
                    Some(x) => println!("After: {x}"),
                };

                match atleast {
                    None => {},
                    Some(x) => println!("At least: {x}"),
                };

                match atmost {
                    None => {},
                    Some(x) => println!("At most: {x}"),
                };
            },
        },
    };

    println!("{:#?}", pipe);
    println!("{:#?}", args);

    for needle in ["finance", "adventure"] {
        println!("{:#?}", myquery::new_tag(&pool, needle).await);
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

