use mysql_async::prelude::*;
use mysql_async::chrono::DateTime;
use mysql_async::chrono::offset::FixedOffset;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Order {
    id: i64,
    code: String,
    type_: i32,
    status: i32,
    user_code: Option<String>,
    deleted: bool,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>
}

#[tokio::main]
async fn main() -> Result<(), mysql_async::error::Error> {
    let date_time = DateTime::parse_from_rfc3339("2020-02-19T16:39:57+08:00").expect("");
    let orders = vec![
        Order { id: 1, code: String::from("1"), type_: 0, status: 0, user_code: Some(String::from("u1")), deleted: false, created_at: date_time, updated_at: date_time },
        Order { id: 2, code: String::from("2"), type_: 0, status: 0, user_code: Some(String::from("u2")), deleted: false, created_at: date_time, updated_at: date_time },
        Order { id: 3, code: String::from("3"), type_: 0, status: 0, user_code: Some(String::from("u3")), deleted: false, created_at: date_time, updated_at: date_time },
        Order { id: 4, code: String::from("4"), type_: 0, status: 0, user_code: Some(String::from("u4")), deleted: false, created_at: date_time, updated_at: date_time },
        Order { id: 5, code: String::from("5"), type_: 0, status: 0, user_code: Some(String::from("u5")), deleted: false, created_at: date_time, updated_at: date_time },
    ];
    let orders_clone = orders.clone();

    let database_url = "mysql://orion:orion@127.0.0.1:3308/orion"; /* ... */

    let pool = mysql_async::Pool::new(database_url);
    let conn = pool.get_conn().await?;

    // Create temporary table
    // let conn = conn.batch_exec(
    //     r"CREATE TABLE `order` (
    //         id bigint unsigned not null,
    //         amount int not null,
    //         account_name text
    //     )"
    // ).await?;

    // Save payments
    let params = orders_clone.into_iter().map(|order| {
        params! {
            "id" => order.id,
            "code" => order.code,
            "type" => order.type_,
            "status" => order.status,
            "user_code" => order.user_code.clone(),
            "deleted" => order.deleted,
            "created_at" => order.created_at.to_rfc3339(),
            "updated_at" => order.updated_at.to_rfc3339(),
        }
    });

    let conn = conn.batch_exec(r"INSERT INTO `order` (id, code, type, status, user_code, deleted, created_at, updated_at)
                    VALUES (:id, :code, :type, :status, :user_code, :deleted, :created_at, :updated_at)", params).await?;

    // Load payments from database.
    let result = conn.prep_exec("SELECT id, code, type, status, user_code, deleted FROM `order`", ()).await?;

    // Collect payments
    let (_ /* conn */, loaded_orders) = result.map_and_drop(|row| {
        let (id, code, type_, status, user_code, deleted) = mysql_async::from_row(row);
        Order {
            id: id,
            code: code,
            type_: type_,
            status: status,
            user_code: user_code,
            deleted: deleted,
            created_at: DateTime::parse_from_rfc3339("2020-02-19T16:39:57+08:00").expect(""),
            updated_at: DateTime::parse_from_rfc3339("2020-02-19T16:39:57+08:00").expect("")
        }
    }).await?;

    // The destructor of a connection will return it to the pool,
    // but pool should be disconnected explicitly because it's
    // an asynchronous procedure.
    pool.disconnect().await?;

    assert_eq!(loaded_orders, orders);

    // the async fn returns Result, so
    Ok(())
}