pub mod myquery;
use crate::myquery::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let needle_1 = "est uan sin ter";
    let needle_2 = "uan sin ter ius";

    let opts = sqlx::mysql::MySqlConnectOptions::new()
        .host(HOST)
        .username(USER)
        .password(PASS)
        .database(DB);

    let (pool, _) = reset_db(&opts)
        .await?;

    for item in Query::builder(&pool)
        .from("date")
        .when(&vec![When::Greater("date")])
        .needle("2022-12-31")
        .build()
        .to::<Item>()
        .await
        .into_iter() {
            println!("Item: {}", item.Name);
        }

    // for id in Query::builder(&pool)
    //     .from("tag")
    //     .when(&vec![When::Match("name")])
    //     .needle("auto|claim")
    //     .aggregate(Agg::Intersect)
    //     .build()
    //     .to::<Id::<Item>>()
    //     .await
    //     .into_iter() {
    //         println!("Id: {}", id.get());
    //     }

    // for id in Query::builder(&pool)
    //     .from("item")
    //     .when(&vec![When::Match("name")])
    //     .needle("Financ")
    //     .aggregate(Agg::Intersect)
    //     .build()
    //     .to::<Id::<Item>>()
    //     .await
    //     .into_iter() {
    //         println!("Id: {}", id.get());
    //     }

    // for tier in Query::builder(&pool)
    //     .from("item")
    //     .when(&vec![
    //         When::Equal("name"),
    //         When::Like("name"),
    //         When::Match("name"),
    //     ])
    //     .needle(needle_1)
    //     .build()
    //     .to_tiers::<Item>()
    //     .await
    //     .into_iter() {
    //         println!("{:#?}", tier);
    //     }

    // for tier in Query::builder(&pool)
    //     .from("item")
    //     .when(&vec![
    //         When::Equal("name"),
    //         When::Like("name"),
    //         When::Match("name"),
    //     ])
    //     .needle(needle_2)
    //     .build()
    //     .to_tiers::<Item>()
    //     .await
    //     .into_iter() {
    //         println!("{:#?}", tier);
    //     }

    // let expected: Vec<When<Vec<Item>>> = vec![
    //     When::Equal(
    //         root.Items
    //             .clone()
    //             .into_iter()
    //             .filter(|x| x.Name == needle_1)
    //             .collect::<Vec<Item>>()
    //     ),

    //     When::Like(
    //         root.Items
    //             .clone()
    //             .into_iter()
    //             .filter(|x| x.Name.starts_with(needle_1))
    //             .collect::<Vec<Item>>()
    //     ),

    //     When::Match(
    //         root.Items
    //             .clone()
    //             .into_iter()
    //             .filter(|x| regex::Regex::new(needle_1)
    //                 .unwrap()
    //                 .is_match(&x.Name)
    //             )
    //             .collect::<Vec<Item>>()
    //     ),
    // ];

    // println!("{:#?}", expected);
    Ok(())
}

