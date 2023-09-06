use crate::restaurant::models::{Items, OrderItems, Tables};
use postgres::{Client, NoTls};
use std::time::SystemTime;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

const DB_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";

//handle_get_all_items function
pub fn handle_get_all_items(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut items = Vec::new();

            // loop table and get all data
            for row in client.query("SELECT * FROM Items", &[]).unwrap() {
                items.push(Items {
                    item_number: row.get(0),
                    item_name: row.get(1),
                    item_price_usd: row.get(2),
                    item_cooking_time_min: row.get(3),
                });
            }
            // return response
            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&items).unwrap(),
            )
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_get_all_tables function
pub fn handle_get_all_tables(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut items = Vec::new();

            // loop table and get all data
            for row in client.query("SELECT * FROM tables", &[]).unwrap() {
                items.push(Tables {
                    table_number: row.get(0),
                    name: row.get(1),
                    is_table_available: row.get(2),
                });
            }
            // return response
            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&items).unwrap(),
            )
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

// handle_get_orders_for_table function
pub fn handle_get_orders_for_table(request: &str) -> (String, String) {
    match (
        get_table_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(table_id), Ok(mut client)) => {
            let mut items = Vec::new();
            // loop table and get all data
            for row in client
                .query(
                    "SELECT * FROM order_items where table_number=$1",
                    &[&table_id],
                )
                .unwrap()
            {
                items.push(OrderItems {
                    table_number: row.get(1),
                    item_number: row.get(2),
                    created_by_name: row.get(3),
                });
            }
            // return response
            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&items).unwrap(),
            )
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

// handle_get_single_order_of_table function
pub fn handle_get_single_order_of_table(request: &str) -> (String, String) {
    match (
        get_table_id(&request).parse::<i32>(),
        get_item_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(table_id), Ok(item_id), Ok(mut client)) => {
            let mut items = Vec::new();
            // loop table and get all data
            for row in client
                .query(
                    "SELECT * FROM order_items where table_number=$1 and item_number=$2",
                    &[&table_id, &item_id],
                )
                .unwrap()
            {
                items.push(OrderItems {
                    table_number: row.get(1),
                    item_number: row.get(2),
                    created_by_name: row.get(3),
                });
            }
            // return response
            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&items).unwrap(),
            )
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_post_order_request function
pub fn handle_post_order_request(request: &str) -> (String, String) {
    match (get_request_body(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(orders), Ok(mut client)) => {
            let now = SystemTime::now();
            let mut next_order_id: i32 = 1;
            // Get Next Order ID for insert
            for row in client
                .query(
                    "SELECT order_items_id FROM order_items ORDER BY order_items_id DESC LIMIT 1",
                    &[],
                )
                .unwrap()
            {
                let value: Option<i32> = row.get(0);
                next_order_id = next_order_id + value.unwrap();
            }

            // Insert data in Order Items
            client
                .execute(
                    "INSERT INTO order_items (order_items_id, table_number, item_number, created_by_name, created_date_time ) VALUES ($1, $2, $3, $4, $5)",
                    &[&next_order_id, &orders.table_number,  &orders.item_number, &orders.created_by_name, &now],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Order Item created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_delete_order_request function
pub fn handle_delete_order_request(request: &str) -> (String, String) {
    match (
        get_table_id(&request).parse::<i32>(),
        get_item_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(table_id), Ok(item_id), Ok(mut client)) => {
            println!(
                "Deleting item number {} for table number{}",
                item_id, table_id
            );
            let rows_affected = client
                .execute(
                    "DELETE FROM order_items where table_number=$1 and item_number=$2",
                    &[&table_id, &item_id],
                )
                .unwrap();

            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "Order Item not found".to_string());
            }

            (OK_RESPONSE.to_string(), "Order Item deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//deserialize OrderItems from request body
fn get_request_body(request: &str) -> Result<OrderItems, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

//get_table_id function
fn get_table_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}
//get_item_id function
fn get_item_id(request: &str) -> &str {
    request
        .split("/")
        .nth(3)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}
