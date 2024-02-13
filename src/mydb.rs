use sqlx::{mysql::MySqlPool, Row};
use serde::Deserialize;
use chrono::NaiveDate;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    Name: String,
    Description: Option<String>,

    #[serde(deserialize_with = "deserialize_optional_naive_date")]
    Arrival: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_optional_naive_date")]
    Expiry: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_optional_naive_date")]
    Created: Option<NaiveDate>,
    Tags: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Tag {
    Name: String,

    #[serde(deserialize_with = "deserialize_optional_naive_date")]
    Created: Option<NaiveDate>,
}

fn deserialize_optional_naive_date<'de, D>(
    deserializer: D
) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: Option<&str> =
        serde::Deserialize::deserialize(deserializer)?;

    if let Some(x) = date_str {
        NaiveDate::parse_from_str(x, DT_FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
    else {
        Ok(None)
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DbRoot {
    Items: Vec<Item>,
}

pub const USER: &str = "myroot";
pub const PASS: &str = "asa1ase3";
pub const HOST: &str = "localhost";
pub const DB: &str = "mydb";
pub const DT_FORMAT: &str = "%Y_%m_%d";

const NEWDB_SQL_PATH: &str = "./sql/new-db.mysql.sql";
const ITEM_JSON_PATH: &str = "./json/Item.json";

fn string_to_dbrow(input: &Option<String>) -> String {
    match input {
        Some(s) => String::from(format!("'{}'", s)),
        None => String::from("NULL"),
    }
}

fn date_to_dbrow(date: &Option<NaiveDate>, format: &str) -> String {
    match date {
        Some(s) => String::from(
            format!(
                "STR_TO_DATE('{}', '{}')",
                s.format(format).to_string(),
                format,
            )
        ),

        None => String::from("NULL"),
    }
}

fn myitem_to_dbrow(item: &Item) -> String {
    vec![
        format!("'{}'", item.Name),
        string_to_dbrow(&item.Description),
        date_to_dbrow(&item.Arrival, DT_FORMAT),
        date_to_dbrow(&item.Expiry, DT_FORMAT),
        date_to_dbrow(&item.Created, DT_FORMAT),
    ]
    .join(",\n    ")
}

fn mytag_to_dbrow(tag: &Tag) -> String {
    vec![
        format!("'{}'", tag.Name),
        date_to_dbrow(&tag.Created, DT_FORMAT),
    ]
    .join(",\n    ")
}

fn myitems_to_dbinsert(db_name: &str, items: &Vec<Item>) -> String {
    format!(
        r#"
INSERT INTO `{}`.`Item` (
    Name,
    Description,
    Arrival,
    Expiry,
    Created
) VALUES (
    {}
);
        "#,
        db_name,
        items
            .iter()
            .map(|x|
                myitem_to_dbrow(x)
            )
            .collect::<Vec<String>>()
            .join("\n), (\n    "),
    )
}

fn mytags_to_dbinsert(db_name: &str, items: &Vec<Item>) -> String {
    let mut tags = std::
        collections::
        BTreeMap::
        <String, Tag>::
        new();

    for item in items {
        for tag_name in &item.Tags {
            let tag = Tag {
                Name: tag_name.clone(),
                Created: item.Created.clone(),
            };

            tags
                .entry(tag_name.to_string())
                .or_insert(tag);
        }
    }

    format!(
        r#"
INSERT IGNORE INTO `{}`.`Tag` (
    Name,
    Created
) VALUES (
    {}
);
        "#,
        db_name,
        tags
            .iter()
            .map(|(_, tag)|
                mytag_to_dbrow(&tag)
            )
            .collect::<Vec<String>>()
            .join("\n), (\n    "),
    )
}

pub async fn get_tag_from_itemid(
    pool: &MySqlPool,
    db_name: &str,
    item_id: i32,
) -> Vec<(i32, Tag)> {
    match sqlx::query(
        format!(
            r#"
SELECT *
FROM
    `{db_name}`.`Tag`
    LEFT JOIN
    `{db_name}`.`Item_has_Tag`
    ON `Id` = `Tag_Id`
WHERE
    `Item_Id` = ?
;
            "#
        )
        .as_str()
    )
    .bind(item_id)
    .fetch_all(pool)
    .await {
        Err(msg) => {
            eprintln!("Error: {}", msg);
            vec![]
        },

        Ok(list) => list
            .iter()
            .map(|row| {
                let id: i32 = row.get("Id");

                let tag = Tag {
                    Name: row.get("Name"),
                    Created: row
                        .try_get::<NaiveDate, &str>("Created")
                        .ok(),
                };

                (id, tag)
            })
            .collect::<Vec<(i32, Tag)>>(),
    }
}

pub async fn get_item_from_tagname(
    pool: &MySqlPool,
    db_name: &str,
    tag_name: &str,
) -> Vec<(i32, Item)> {
    match sqlx::query(
        format!(
            r#"
SELECT *
FROM
    `{db_name}`.`Item`
    LEFT JOIN
    `{db_name}`.`Item_has_Tag`
    ON `Id` = `Item_Id`
WHERE
    `Tag_Id` = (
        SELECT `Id`
        FROM `Tag`
        WHERE `Name` = ?
    )
;
            "#
        )
        .as_str()
    )
    .bind(tag_name)
    .fetch_all(pool)
    .await {
        Err(msg) => {
            eprintln!("Error: {}", msg);
            vec![]
        },

        Ok(list) => list
            .iter()
            .map(|row| {
                let id: i32 = row.get("Id");

                let item = Item {
                    Name: row.get("Name"),
                    Description: row.get("Description"),
                    Arrival: row
                        .try_get::<NaiveDate, &str>("Arrival")
                        .ok(),
                    Expiry: row
                        .try_get::<NaiveDate, &str>("Expiry")
                        .ok(),
                    Created: row
                        .try_get::<NaiveDate, &str>("Created")
                        .ok(),
                    Tags: vec![],
                };

                (id, item)
            })
            .collect::<Vec<(i32, Item)>>(),
    }
}

pub async fn add_by_name_itemhastag(
    pool: &MySqlPool,
    db_name: &str,
    item: &str,
    tag: &str,
) -> anyhow::Result<u64> {
    let id = sqlx::query(
        format!(
            r#"
INSERT INTO `{db_name}`.`Item_has_Tag`
    (`Item_Id`, `Tag_Id`)
SELECT
    a.`Id`, b.`Id`
FROM
    `{db_name}`.`Item` as a
    JOIN
    `{db_name}`.`Tag` as b
WHERE
    a.`Name` = ?
    AND
    b.`Name` = ?
ON DUPLICATE KEY UPDATE
    `Item_Id` = a.`Id`,
    `Tag_Id` = b.`Id`
;
            "#
        )
        .as_str()
    )
    .bind(item)
    .bind(tag)
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(id)
}

pub async fn add_list_itemhastag(
    pool: &MySqlPool,
    db_name: &str,
    items: &Vec<Item>
) -> anyhow::Result<u64> {
    let mut id: u64 = 0;

    for item in items {
        for tag_name in &item.Tags {
            id = add_by_name_itemhastag(
                pool,
                db_name,
                &item.Name,
                tag_name
            )
            .await?
        }
    }

    Ok(id)
}

// todo: move to test
pub async fn reset_db(
    opts: &sqlx::mysql::MySqlConnectOptions
) -> anyhow::Result<(MySqlPool, DbRoot)> {
    let sql = std::fs::read_to_string(&NEWDB_SQL_PATH)
        .expect(&format!("Error: Failed to find path '{}'", NEWDB_SQL_PATH));

    let pool: MySqlPool =
        MySqlPool::connect_with(opts.to_owned())
        .await?;

    let json = std::fs::read_to_string(&ITEM_JSON_PATH)
        .expect(&format!("Error: Failed to find path '{}'", ITEM_JSON_PATH));

    let root: DbRoot = serde_json::from_str(&json)
        .unwrap();

    for item in sql
        .split(";")
        .map(|x| x.trim())
        .filter(|&x| !x.is_empty())
    {
        _ = sqlx::query(format!("{};", item).as_str())
            .execute(&pool)
            .await
            .map_err(|err| {
                eprintln!("Query failed: [{}]\nError: {}", &item, err);
            });
    }

    _ = sqlx::query(&myitems_to_dbinsert("mydb", &root.Items))
        .execute(&pool)
        .await
        .map_err(|err| {
            eprintln!("Item table insert failed. Error: {}", err);
        });

    _ = sqlx::query(&mytags_to_dbinsert("mydb", &root.Items))
        .execute(&pool)
        .await
        .map_err(|err| {
            eprintln!("Item table insert failed. Error: {}", err);
        });

    _ = add_list_itemhastag(&pool, "mydb", &root.Items)
        .await?;

    Ok((pool, root))
}

#[allow(dead_code)]
fn itemhastag_to_dbinsert(db_name: &str, items: &Vec<Item>) -> String {
    let mut item_has_tag: Vec<String> = vec![];

    for item in items {
        for tag_name in &item.Tags {
            item_has_tag
                .push(
                    format!(
                        r#"
INSERT INTO `{db_name}`.`Item_has_Tag`
    (`Item_Id`, `Tag_Id`)
SELECT
    a.`Id`, b.`Id`
FROM
    `{db_name}`.`Item` as a
    JOIN
    `{db_name}`.`Tag` as b
WHERE
    a.`Name` = '{}'
    AND
    b.`Name` = '{}'
ON DUPLICATE KEY UPDATE
    `Item_Id` = a.`Id`,
    `Tag_Id` = b.`Id`
;
                        "#,
                        item.Name,
                        tag_name,
                    )
                );
        }
    }

    item_has_tag.join("\n")
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tokio_test;
    use crate::mydb;

    #[test]
    pub fn it_works() {
        let opts = sqlx::mysql::MySqlConnectOptions::new()
            .host(mydb::HOST)
            .username(mydb::USER)
            .password(mydb::PASS)
            .database(mydb::DB);

        tokio_test::block_on(async {
            if let Ok((pool, root)) = reset_db(&opts).await {
                let finance_items: Vec<Item> = root.Items
                    .into_iter()
                    .filter(|x| x.Name.starts_with("Finance"))
                    .collect();

                let queried_items = get_item_from_tagname(
                    &pool,
                    &mydb::DB,
                    "finance",
                )
                .await;

                for (index, (id, item)) in queried_items
                    .into_iter()
                    .enumerate()
                {
                    let tags = get_tag_from_itemid(
                        &pool,
                        &mydb::DB,
                        id
                    )
                    .await
                    .into_iter()
                    .map(|(_, tag)| tag.Name)
                    .collect::<Vec<String>>();

                    let item = Item {
                        Tags: tags,
                        ..item
                    };

                    assert_eq!(item, finance_items[index]);
                }
            }
        });
    }
}

