use sqlx::{mysql::MySqlPool, mysql::MySqlRow, Row};
use chrono::NaiveDate;

pub const MAIN_TABLE: &str = "item";
pub const USER: &str = "myroot";
pub const PASS: &str = "asa1ase3";
pub const HOST: &str = "localhost";
pub const DB: &str = "mydb";
pub const DT_FORMAT: &str = "%Y-%m-%d";
pub const NEWDB_SQL_PATH: &str = "./sql/new-db.mysql.sql";
pub const ITEM_JSON_PATH: &str = "./json/Item.json";

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum When<T: std::clone::Clone> {
    Equal(T),
    Like(T),
    Match(T),
    Approx(T),
    Less(T),
    Greater(T),
    AtMost(T),
    AtLeast(T),
    Other(T),
}

impl<T: std::clone::Clone> When<T> {
    fn into_when<R: std::clone::Clone>(&self, to: R) -> When<R> {
        match self {
            When::Equal(_) => When::Equal(to),
            When::Like(_) => When::Like(to),
            When::Match(_) => When::Match(to),
            When::Approx(_) => When::Approx(to),
            When::Less(_) => When::Less(to),
            When::Greater(_) => When::Greater(to),
            When::AtMost(_) => When::AtMost(to),
            When::AtLeast(_) => When::AtLeast(to),
            When::Other(_) => When::Other(to),
        }
    }
}

impl<'a> Default for When<&'a str> {
    fn default() -> When<&'a str> {
        When::<&'a str>::Equal("")
    }
}

impl<'a> Default for When<(&'a str, &'a str)> {
    fn default() -> When<(&'a str, &'a str)> {
        When::<(&'a str, &'a str)>::Equal(("", ""))
    }
}

#[derive(Clone)]
pub enum Agg {
    Union,
    Intersect,
}

fn get_haystack_and_needle<'a>(
    when: When<(&'a str, &'a str)>
) -> (String, String) {
    let (by, search_type, needle) = match when {
        When::Equal((x, y)) =>
            (x.to_string(), String::from("REGEXP CONCAT('^', ?, '$')"), y.to_string()),

        When::Like((x, y)) =>
            (x.to_string(), String::from("REGEXP CONCAT('^', ?)"), y.to_string()),

        When::Match((x, y)) =>
            (x.to_string(), String::from("REGEXP (?)"), y.to_string()),

        When::Approx(_) => {
            eprintln!(
                "Error: new_query_str: Fuzzy search not yet implemented",
            );

            return (String::new(), String::new());
        },

        When::Less((x, y)) =>
            (x.to_string(), String::from("< (?)"), y.to_string()),

        When::Greater((x, y)) =>
            (x.to_string(), String::from("> (?)"), y.to_string()),

        When::AtMost((x, y)) =>
            (x.to_string(), String::from("<= (?)"), y.to_string()),

        When::AtLeast((x, y)) =>
            (x.to_string(), String::from(">= (?)"), y.to_string()),

        When::Other(_) => {
            eprintln!(
                "Error: new_query_str: Other search not yet implemented",
            );

            return (String::new(), String::new());
        },
    };

    (format!("`{by}` {search_type}"), needle)
}

fn new_query_str<'a>(
    from: &'a str,
    to: &'a str,
    haystack: String,
    minus: Option<&'a Vec<i32>>,
) -> Option<(String, String, String)> {
    let to =
        if to.is_empty() { from } else { to };

    let query =
        if to == from {
            (
                format!("`{to}`"),
                haystack,
                String::from("`id`"),
            )
        }
        else {
            let other_table: &'a str =
                if from == MAIN_TABLE {
                    to
                }
                else if to == MAIN_TABLE {
                    from
                }
                else {
                    eprintln!(
                        "Error: new_query_str: main table '{}' missing in query",
                        MAIN_TABLE,
                    );

                    return None;
                };

            let from_table_str: String =
                format!(
                    "`{to}` LEFT JOIN `item_has_{other_table}` ON `Id` = `{to}_Id`"
                );

            (
                from_table_str,
                format!(
                    "`{from}_Id` in (SELECT `Id` FROM `{from}` WHERE {haystack})"
                ),
                format!(
                    "`{from}_Id`"
                ),
            )
        };

    let query = match minus {
        Some(exclude_ids) if exclude_ids.len() > 0 => {
            let exclude_ids = exclude_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            (
                query.0.to_string(),
                format!(
                    "{} and `{to}_Id` not in ({})",
                    query.1,
                    exclude_ids,
                ),
                query.1,
            )
        },

        _ => query,
    };

    Some(query)
}

pub trait MySqlMarshal {
    fn marshal(row: MySqlRow) -> Self;
    fn col_name() -> String;
    fn table_name() -> String;
    fn id(&self) -> i32;
}

#[derive(Clone)]
pub struct Query<'a> {
    pool: &'a MySqlPool,
    from: &'a str,

    /// [ SearchType ( Haystack ) ]
    when: Vec<When<(&'a str, &'a str)>>,

    minus: Option<&'a Vec<i32>>,
    aggregate: Agg,
}

impl<'a> Query<'a> {
    pub fn builder(pool: &'a MySqlPool) -> QueryBuilder::<'a> {
        QueryBuilder::<'a>::new(pool)
    }

    pub async fn to_complete_item(&self, item: Item) -> Item {
        let tags = Query::builder(self.pool)
            .from("item")
            .when(&vec![When::Equal(("id", &item.Id.to_string()))])
            .build()
            .to::<Tag>()
            .await
            .into_iter()
            .map(|tag| tag.Name)
            .collect::<Vec<String>>();

        let dates = Query::builder(self.pool)
            .from("item")
            .when(&vec![When::Equal(("id", &item.Id.to_string()))])
            .build()
            .to::<Date>()
            .await
            .into_iter()
            .map(|date| date.Date)
            .collect::<Vec<NaiveDate>>();

        Item {
            Tags: tags,
            Dates: dates,
            ..item.clone()
        }
    }

    pub async fn to_complete_items(&self) -> Vec<Item> {
        let mut list: Vec<Item> = vec![];

        let items = self
            .to::<Item>()
            .await;

        // Do not try to map iterators here.
        for item in items {
            list.push(self.to_complete_item(item).await);
        }

        list
    }

    pub async fn to<T>(&self) -> Vec<T>
    where
        T: MySqlMarshal
            + std::clone::Clone
            + Eq
            + PartialEq
    {
        self.get_results(
            vec![self.when[0].clone()],
            self.minus,
        )
        .await
    }

    async fn get_results<T>(
        &self,
        when: Vec<When<(&str, &str)>>,
        minus: Option<&Vec<i32>>,
    ) -> Vec<T>
    where
        T: MySqlMarshal
            + std::clone::Clone
            + Eq
            + PartialEq
    {
        let to = T::table_name();
        let mut haystacks: Vec<String> = vec![];
        let mut needles: Vec<String> = vec![];
        let mut true_number_of_needles: usize = 0;

        for pair in when {
            let (haystack, needle) = get_haystack_and_needle(pair);

            haystacks.push(haystack);
            needles.push(needle.clone());

            true_number_of_needles += needle
                .split("|")
                .collect::<Vec<&str>>()
                .len();
        }

        let haystacks: String = haystacks.join(" AND ");

        let query = new_query_str(
            self.from,
            to.as_str(),
            haystacks,
            minus,
        );

        if let Some((from, where_clause, countby_col)) = query {
            // link
            // - url: <https://stackoverflow.com/questions/41887460/select-list-is-not-in-group-by-clause-and-contains-nonaggregated-column-inc>
            // - retrieved: 2024_02_18
            let query = format!(
r#"SELECT {} FROM `{}` WHERE `Id` IN (
    SELECT `Id`
    FROM
        {}
    WHERE
        {}
    GROUP BY
        `Id`{}
)
ORDER BY `Id`;"#,
                T::col_name(),
                to,
                from,
                where_clause,
                match self.aggregate {
                    Agg::Union => String::new(),
                    Agg::Intersect => format!(
                        " HAVING COUNT(DISTINCT {countby_col}) = {}",
                        true_number_of_needles,
                    ),
                },
            );

            // // todo
            // println!("\n{}", query);
            // println!("? = {:#?}\n", needles);

            let mut query = sqlx::query(query.as_str());

            for needle in needles {
                query = query.bind(needle.clone())
            }

            match query
                .fetch_all(self.pool)
                .await {
                    Err(msg) => {
                        eprintln!("Error: {}", msg);
                        vec![]
                    },

                    Ok(rows) => rows
                        .into_iter()
                        .map(|row| T::marshal(row))
                        .collect::<Vec<T>>(),
                }
        }
        else {
            vec![]
        }
    }
}

pub struct QueryBuilder<'a> {
    pool: &'a MySqlPool,
    from: &'a str,

    /// [ SearchType ( Haystack ) ]
    when: Vec<When<(&'a str, &'a str)>>,

    minus: Option<&'a Vec<i32>>,
    aggregate: Agg,
}

impl<'a> QueryBuilder<'a> {
    pub const DEFAULT_STR: &'static str = "";

    pub fn new(pool: &'a MySqlPool) -> Self {
        QueryBuilder {
            pool: pool,
            from: Self::DEFAULT_STR,
            when: vec![When::<(&str, &str)>::default()],
            minus: None,
            aggregate: Agg::Union,
        }
    }

    pub fn pool(&'a mut self, pool: &'a MySqlPool) -> &'a mut Self {
        self.pool = pool;
        self
    }

    pub fn from(&'a mut self, from: &'a str) -> &'a mut Self {
        self.from = from;
        self
    }

    pub fn when(&'a mut self, when: &'a Vec<When<(&'a str, &'a str)>>) -> &'a mut Self {
        self.when = when.clone();
        self
    }

    pub fn minus(&'a mut self, minus: Option<&'a Vec<i32>>) -> &'a mut Self {
        self.minus = minus.clone();
        self
    }

    pub fn aggregate(&'a mut self, aggregate: Agg) -> &'a mut Self {
        self.aggregate = aggregate.clone();
        self
    }

    pub fn build(&'a self) -> Query<'a> {
        Query {
            pool: self.pool,
            from: self.from,
            when: self.when.clone(),
            minus: self.minus.clone(),
            aggregate: self.aggregate.clone(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Item {
    #[serde(default)]
    pub Id: i32,

    pub Name: String,
    pub Description: Option<String>,

    #[serde(deserialize_with = "deserialize_optional_naive_date")]
    pub Arrival: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_optional_naive_date")]
    pub Expiry: Option<NaiveDate>,

    #[serde(deserialize_with = "deserialize_optional_naive_date")]
    pub Created: Option<NaiveDate>,

    pub Tags: Vec<String>,

    #[serde(deserialize_with = "deserialize_naive_date_vec")]
    pub Dates: Vec<NaiveDate>,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    #[serde(default)]
    pub Id: i32,

    pub Name: String,

    #[serde(deserialize_with = "deserialize_optional_naive_date")]
    pub Created: Option<NaiveDate>,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Date {
    #[serde(default)]
    pub Id: i32,

    #[serde(deserialize_with = "deserialize_naive_date")]
    pub Date: NaiveDate,
}

fn deserialize_naive_date<'de, D>(
    deserializer: D
) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: Option<&str> =
        serde::Deserialize::deserialize(deserializer)?;

    if let Some(x) = date_str {
        NaiveDate::parse_from_str(x, DT_FORMAT)
            .map_err(serde::de::Error::custom)
    }
    else {
        Ok(NaiveDate::default())
    }
}

fn deserialize_naive_date_vec<'de, D>(
    deserializer: D
) -> Result<Vec<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let maybe_str_list: Option<Vec<&str>> =
        serde::Deserialize::deserialize(deserializer)?;

    let mut list: Vec<NaiveDate> = vec![];

    if let Some(str_list) = maybe_str_list {
        for item in str_list {
            let parse = NaiveDate::parse_from_str(item, DT_FORMAT);

            match parse {
                Ok(y) => list.push(y),
                Err(_) => (),
            }
        }
    }

    Ok(list)
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
#[derive(serde::Deserialize, Debug)]
pub struct DbRoot {
    pub Items: Vec<Item>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Id<T> {
    id: i32,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Id<T> {
    pub fn get(&self) -> i32 { self.id }
}

impl MySqlMarshal for i32 {
    fn marshal(row: MySqlRow) -> i32 {
        row.get("Id")
    }

    fn col_name() -> String { String::from("`Id`") }
    fn table_name() -> String { String::new() }
    fn id(&self) -> i32 { *self }
}

impl<T> MySqlMarshal for Id<T>
where
    T: MySqlMarshal,
{
    fn marshal(row: MySqlRow) -> Id<T> {
        Id::<T> {
            id: i32::marshal(row),
            phantom: std::marker::PhantomData,
        }
    }

    fn col_name() -> String { i32::col_name() }
    fn table_name() -> String { T::table_name() }
    fn id(&self) -> i32 { self.get() }
}

impl MySqlMarshal for Item {
    fn marshal(row: MySqlRow) -> Item {
        Item {
            Id: row.get("Id"),
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
            Dates: vec![],
        }
    }

    fn col_name() -> String { String::from("*") }
    fn table_name() -> String { String::from("item") }
    fn id(&self) -> i32 { self.Id }
}

impl MySqlMarshal for Tag {
    fn marshal(row: MySqlRow) -> Tag {
        Tag {
            Id: row.get("Id"),
            Name: row.get("Name"),
            Created: row
                .try_get::<NaiveDate, &str>("Created")
                .ok(),
        }
    }

    fn col_name() -> String { String::from("*") }
    fn table_name() -> String { String::from("tag") }
    fn id(&self) -> i32 { self.Id }
}

impl MySqlMarshal for Date {
    fn marshal(row: MySqlRow) -> Date {
        Date {
            Id: row.get("Id"),
            Date: row.get("Date"),
        }
    }

    fn col_name() -> String { String::from("*") }
    fn table_name() -> String { String::from("date") }
    fn id(&self) -> i32 { self.Id }
}

mod dbstring {
    use super::*;

    pub fn string_to_dbrow(input: &Option<String>) -> String {
        match input {
            Some(s) => format!("'{}'", s),
            None => String::from("NULL"),
        }
    }

    pub fn date_to_dbrow(date: &Option<NaiveDate>) -> String {
        match date {
            Some(s) => format!(
                "'{}'",
                s.format(DT_FORMAT).to_string()
            ),

            None => String::from("NULL"),
        }
    }

    pub fn myitem_to_dbrow(item: &Item) -> String {
        vec![
            format!("'{}'", item.Name),
            string_to_dbrow(&item.Description),
            date_to_dbrow(&item.Arrival),
            date_to_dbrow(&item.Expiry),
            date_to_dbrow(&item.Created),
        ]
        .join(",\n    ")
    }

    pub fn mytag_to_dbrow(tag: &Tag) -> String {
        vec![
            format!("'{}'", tag.Name),
            date_to_dbrow(&tag.Created),
        ]
        .join(",\n    ")
    }
}

fn get_zip<T: std::clone::Clone>(
    first: Vec<T>,
    secnd: Vec<T>
) -> Vec<(T, T)> {
    first
        .iter()
        .cloned()
        .zip(secnd
            .iter()
            .cloned()
        )
        .collect::<Vec<(T, T)>>()
}

pub fn myrow_to_dbremove(table: &str, ids: Vec<i32>) -> String {
    format!(
        "DELETE FROM `{table}` WHERE `Id` IN ({});",
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", "),
    )
}

pub fn myrow_to_dbassociate(
    main_table: &str,
    property_table: &str,
    main_id: i32,
    property_id: i32,
) -> String {
    format!(
r#"INSERT IGNORE INTO `{main_table}_has_{property_table}` (
    `{main_table}_Id`, `{property_table}_Id`
) VALUES (
    {main_id}, {property_id}
);"#,
    )
}

pub fn myrow_to_dbdissociate(
    main_table: &str,
    property_table: &str,
    main_id: i32,
    property_id: i32,
) -> String {
    format!(
r#"DELETE FROM `{main_table}_has_{property_table}`
WHERE `{main_table}_Id` = {main_id}
AND `{property_table}_Id` = {property_id};"#
    )
}

pub fn myitems_to_dbinsert(items: &Vec<Item>) -> String {
    let values = items
        .iter()
        .map(|x| dbstring::myitem_to_dbrow(x))
        .collect::<Vec<String>>();

    let columns = vec![
        "Name",
        "Description",
        "Arrival",
        "Expiry",
        "Created",
    ];

    format!(
r#"INSERT INTO `Item` (
    {}
)
VALUES (
    {}
)
AS new_rows
ON DUPLICATE KEY UPDATE
    {}
;"#,
        columns.join(",\n    "),
        values.join("\n), (\n    "),
        columns
            .iter()
            .map(|x| format!("{x} = new_rows.{x}"))
            .collect::<Vec<String>>()
            .join(",\n    "),
    )
}

pub fn mydates_to_dbinsert(db_name: &str, items: &Vec<Item>) -> String {
    let mut dates = std::
        collections::
        BTreeMap::
        <&NaiveDate, Date>::
        new();

    for item in items {
        for naive_date in &item.Dates {
            let date = Date {
                Id: -1,
                Date: naive_date.clone(),
            };

            dates
                .entry(naive_date)
                .or_insert(date);
        }
    }

    format!(
r#"INSERT IGNORE INTO `{}`.`Date` (
    Date
) VALUES (
    {}
);"#,
        db_name,
        dates
            .iter()
            .map(|(_, date)|
                dbstring::date_to_dbrow(&Some(date.Date))
            )
            .collect::<Vec<String>>()
            .join("\n), (\n    "),
    )
}

pub fn mytags_to_dbinsert(db_name: &str, items: &Vec<Item>) -> String {
    let mut tags = std::
        collections::
        BTreeMap::
        <&str, Tag>::
        new();

    for item in items {
        for tag_name in &item.Tags {
            let tag = Tag {
                Id: -1,
                Name: tag_name.clone(),
                Created: item.Created.clone(),
            };

            tags
                .entry(tag_name)
                .or_insert(tag);
        }
    }

    format!(
r#"INSERT IGNORE INTO `{}`.`Tag` (
    Name,
    Created
) VALUES (
    {}
);"#,
        db_name,
        tags
            .iter()
            .map(|(_, tag)|
                dbstring::mytag_to_dbrow(&tag)
            )
            .collect::<Vec<String>>()
            .join("\n), (\n    "),
    )
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

pub async fn add_by_name_itemhasdate(
    pool: &MySqlPool,
    db_name: &str,
    item: &str,
    date: &NaiveDate,
) -> anyhow::Result<u64> {
    let id = sqlx::query(
        format!(
            r#"
INSERT INTO `{db_name}`.`Item_has_Date`
    (`Item_Id`, `Date_Id`)
SELECT
    a.`Id`, b.`Id`
FROM
    `{db_name}`.`Item` as a
    JOIN
    `{db_name}`.`Date` as b
WHERE
    a.`Name` = ?
    AND
    b.`Date` = ?
ON DUPLICATE KEY UPDATE
    `Item_Id` = a.`Id`,
    `Date_Id` = b.`Id`
;
            "#
        )
        .as_str()
    )
    .bind(item)
    .bind(*date)
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

pub async fn add_list_itemhasdate(
    pool: &MySqlPool,
    db_name: &str,
    items: &Vec<Item>,
) -> anyhow::Result<u64> {
    let mut id: u64 = 0;

    for item in items {
        for naive_date in &item.Dates {
            id = add_by_name_itemhasdate(
                pool,
                db_name,
                &item.Name,
                naive_date,
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

    _ = sqlx::query(&myitems_to_dbinsert(&root.Items))
        .execute(&pool)
        .await
        .map_err(|err| {
            eprintln!("Item table insert failed. Error: {}", err);
        });

    _ = sqlx::query(&mytags_to_dbinsert(DB, &root.Items))
        .execute(&pool)
        .await
        .map_err(|err| {
            eprintln!("Item table insert failed. Error: {}", err);
        });

    _ = add_list_itemhastag(&pool, DB, &root.Items)
        .await?;

    _ = sqlx::query(&mydates_to_dbinsert(DB, &root.Items))
        .execute(&pool)
        .await
        .map_err(|err| {
            eprintln!("Item table insert failed. Error: {}", err);
        });

    _ = add_list_itemhasdate(&pool, DB, &root.Items)
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

    fn exclude<T: std::clone::Clone>(
        list: Vec<T>,
        minus: Vec<i32>
    ) -> Option<Vec<T>> {
        if minus.len() == 0 {
            return None;
        }

        Some(
            list.into_iter()
                .enumerate()
                .filter(|(index, _)| {
                    // (karlr 2024_02_15)
                    // ``&((*index as i32) + 1)`` !!!?
                    // I really feel like I shouldn't have to do this.
                    !minus.contains(&((*index as i32) + 1))
                })
                .map(|(_, value)| value.clone())
                .collect::<Vec<T>>()
        )
    }

    fn get_actual_searches(
        root: &DbRoot,
        needle: &str,
    ) -> (Vec<Item>, Vec<Item>, Vec<Item>) {
        let equals = root.Items
            .clone()
            .into_iter()
            .filter(|x| x.Name == needle)
            .collect::<Vec<Item>>();

        let minus = equals
            .clone()
            .into_iter()
            .map(|x| x.Id)
            .collect::<Vec<i32>>();

        let likes = root.Items
            .clone()
            .into_iter()
            .filter(|x| x.Name.starts_with(needle))
            .collect::<Vec<Item>>();

        let likes = match exclude(likes.clone(), minus.clone()) {
            Some(list) => list,
            None => likes,
        };

        let minus = likes
            .clone()
            .into_iter()
            .map(|x| x.Id)
            .collect::<Vec<i32>>();

        let matches = root.Items
            .clone()
            .into_iter()
            .filter(|x| regex::Regex::new(needle)
                .unwrap()
                .is_match(&x.Name)
            )
            .collect::<Vec<Item>>();

        let matches = match exclude(matches.clone(), minus.clone()) {
            Some(list) => list,
            None => matches,
        };

        (equals, likes, matches)
    }

    #[allow(unused_variables)]
    async fn get_tiered_results<T>(
        query: &Query<'_>,
    ) -> Vec<When<Vec<T>>>
    where
        T: MySqlMarshal
            + std::clone::Clone
            + Eq
            + PartialEq
    {
        let mut all_results: Vec<When<Vec<T>>> = vec![];
        let minus: Option<&Vec<i32>> = None;

        for tier in &query.when {
            let results =
                query.get_results::<T>(
                    vec![tier.clone()],
                    minus,
                )
                .await;

            all_results.push(tier.into_when(results.clone()));

            let minus = Some(
                results
                    .into_iter()
                    .map(|x| x.id())
                    .collect::<Vec<i32>>()
            );
        }

        all_results
    }

    async fn test_three_tiered_search(
        pool: &sqlx::mysql::MySqlPool,
        from: &str,
        by: &str,
        needle: &str,
    ) {
        let json = std::fs::read_to_string(&ITEM_JSON_PATH)
            .expect(&format!("Error: Failed to find path '{}'", ITEM_JSON_PATH));

        let root: DbRoot = serde_json::from_str(&json)
            .unwrap();

        let (equals, likes, matches) = get_actual_searches(&root, needle);

        let when = vec![
            When::Equal((by, needle)),
            When::Like((by, needle)),
            When::Match((by, needle)),
        ];

        let mut builder = Query::builder(&pool);

        let query = builder
            .from(&from)
            .when(&when)
            .build();

        let actual: Vec<When<Vec<Item>>> =
            get_tiered_results(&query)
                .await;

        assert_ne!(actual.len(), 0);

        for when in actual {
            let (actual, expected) = match when {
                When::Equal(list) => (list, equals.clone()),
                When::Like(list) => (list, likes.clone()),
                When::Match(list) => (list, matches.clone()),
                _ => { continue; },
            };

            for (actual, expected) in get_zip(actual, expected) {
                assert_eq!(actual.Name, expected.Name);
            }
        }
    }

    #[test]
    pub fn test_000_items_by_tagname_succeeds() {
        let opts = sqlx::mysql::MySqlConnectOptions::new()
            .host(HOST)
            .username(USER)
            .password(PASS)
            .database(DB);

        let minus: Vec<i32> = vec![2, 4];

        tokio_test::block_on(async {
            if let Ok((pool, root)) = reset_db(&opts).await {
                let expected: Vec<Item> = root.Items
                    .into_iter()
                    .filter(|x| x.Name.starts_with("Financ"))
                    .collect();

                let expected = match exclude(expected.clone(), minus.clone()) {
                    Some(list) => list,
                    None => expected,
                };

                let actual = Query::builder(&pool)
                    .from("tag")
                    .when(&vec![When::Equal(("name", "finance"))])
                    .minus(Some(&minus))
                    .build()
                    .to_complete_items()
                    .await;

                assert_ne!(actual.len(), 0);

                for (index, item) in actual
                    .iter()
                    .enumerate()
                {
                    assert_eq!(*item, expected[index]);
                }
            }
        });
    }

    #[test]
    pub fn test_001_items_by_tagname_intersect_succeeds() {
        let opts = sqlx::mysql::MySqlConnectOptions::new()
            .host(HOST)
            .username(USER)
            .password(PASS)
            .database(DB);

        tokio_test::block_on(async {
            if let Ok((pool, root)) = reset_db(&opts).await {
                let expected: Vec<Item> = root.Items
                    .into_iter()
                    .filter(|x| x.Name.starts_with("Auto Claim"))
                    .collect();

                let actual = Query::builder(&pool)
                    .from("tag")
                    .when(&vec![When::Match(("name", "auto|claim"))])
                    .aggregate(Agg::Intersect)
                    .build()
                    .to_complete_items()
                    .await;

                assert_ne!(actual.len(), 0);

                for (index, item) in actual
                    .iter()
                    .enumerate()
                {
                    assert_eq!(*item, expected[index]);
                }
            }
        });
    }

    #[test]
    pub fn test_002_three_tiered_search_items_by_name_succeeds() {
        let opts = sqlx::mysql::MySqlConnectOptions::new()
            .host(HOST)
            .username(USER)
            .password(PASS)
            .database(DB);

        let needles: Vec<&str> = vec![
            "est uan sin ter",
            "uan sin ter ius",
        ];

        tokio_test::block_on(async {
            if let Ok(pool) = MySqlPool::connect_with(opts.to_owned())
                .await
            {
                for needle in needles {
                    test_three_tiered_search(&pool, "item", "name", needle)
                        .await;
                }
            }
        });
    }

    #[test]
    pub fn test_003_date_lower_bound() {
        let opts = sqlx::mysql::MySqlConnectOptions::new()
            .host(HOST)
            .username(USER)
            .password(PASS)
            .database(DB);

        let json = std::fs::read_to_string(&ITEM_JSON_PATH)
            .expect(&format!("Error: Failed to find path '{}'", ITEM_JSON_PATH));

        let root: DbRoot = serde_json::from_str(&json)
            .unwrap();

        let lower: &str = "2022-12-31";

        let lower_as_date: NaiveDate =
            NaiveDate::parse_from_str(lower, DT_FORMAT)
                .expect("You entered the test string or date format incorrectly.");

        tokio_test::block_on(async {
            if let Ok(pool) = MySqlPool::connect_with(opts.to_owned()).await {
                let expected: Vec<Item> = root.Items
                    .into_iter()
                    .filter(|x| {
                        for date in &x.Dates {
                            if date > &lower_as_date {
                                return true;
                            }
                        }

                        false
                    })
                    .collect();

                let actual = Query::builder(&pool)
                    .from("date")
                    .when(&vec![When::Greater(("date", lower))])
                    .build()
                    .to::<Item>()
                    .await;

                assert_ne!(actual.len(), 0);

                for (actual, expected) in get_zip(actual, expected) {
                    assert_eq!(actual.Name, expected.Name);
                }
            }
        });
    }
}

