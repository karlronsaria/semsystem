use super::*;
use futures;

pub async fn item(
    pool: &MySqlPool,
    db_name: &str,
    where_clause: &str,
    sentinel: &str,
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
    {where_clause}
;
            "#
        )
        .as_str()
    )
    .bind(sentinel)
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

pub async fn to_complete(
    pool: &MySqlPool,
    db_name: &str,
    items: &Vec<(i32, Item)>
) -> Vec<(i32, Item)> {
    items
        .into_iter()
        .map(|(id, item)| {
            let tags =
                futures::executor::block_on(
                    get::tag_from_itemid(
                        pool,
                        db_name,
                        *id,
                    )
                )
                .into_iter()
                .map(|(_, tag)| tag.Name)
                .collect::<Vec<String>>();

            let item = Item {
                Tags: tags,
                ..item.clone()
            };

            (*id, item)
        })
        .collect::<Vec<(i32, Item)>>()
}

pub async fn item_from_tagname(
    pool: &MySqlPool,
    db_name: &str,
    tag_name: &str,
) -> Vec<(i32, Item)> {
    item(
        pool,
        db_name,
        "`Tag_Id` = (SELECT `Id` FROM `Tag` WHERE `Name` = ?)",
        tag_name,
    )
    .await
}

pub async fn tag(
    pool: &MySqlPool,
    db_name: &str,
    where_clause: &str,
    sentinel: &str,
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
    {where_clause}
;
            "#
        )
        .as_str()
    )
    .bind(sentinel)
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

pub async fn tag_from_itemid(
    pool: &MySqlPool,
    db_name: &str,
    item_id: i32,
) -> Vec<(i32, Tag)> {
    tag(pool, db_name, "`Item_Id` = ?", &item_id.to_string()).await
}

