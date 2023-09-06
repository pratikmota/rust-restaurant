use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use rand::Rng;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::SystemTime;

//DATABASE_URL
//"postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
const DB_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
//env!("DATABASE_URL");

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//Model: Items struct with item number, name and cooking time
#[derive(Serialize, Deserialize)]
struct Items {
    item_number: i32,
    item_name: String,
    item_price_usd: f32,
    item_cooking_time_min: i32,
}

//Model: Tables struct with table number, name and availability
#[derive(Serialize, Deserialize)]
struct Tables {
    table_number: i32,
    name: String,
    is_table_available: bool,
}

//Model: OrderItems struct with order id, table number, item number, created name and date
#[derive(Serialize, Deserialize)]
struct OrderItems {
    table_number: i32,
    item_number: i32,
    created_by_name: String,
}

fn main() {
    //Create Tables if not exist.
    if let Err(e) = create_table_initialization() {
        println!("Error: {}", e);
        return;
    }
    // Insert item table data if not exist ( Master table entry)
    if let Err(e) = insert_item_master_data() {
        println!("Error: {}", e);
        return;
    }
    // Insert tables data if not exist ( Master table entry)
    if let Err(e) = insert_tables_master_data() {
        println!("Error: {}", e);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // handling client with multi thread
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error in handling");
            }
        }
    }
}

//insert_database function
fn insert_item_master_data() -> Result<(), PostgresError> {
    //Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    // Insert for first time only.
    for row in client.query("SELECT COUNT(*) as total_items FROM items", &[])? {
        let row_count: Option<i64> = row.get(0);
        // If there are already Rows in Database do not insert
        if row_count.unwrap() == 0 {
            // Random number
            let mut rng = rand::thread_rng();
            // Fill Master data for items
            let total_items = vec![
                Items {
                    item_number: 1,
                    item_name: "Pizza".to_string(),
                    item_price_usd: 15.0,
                    item_cooking_time_min: rng.gen_range(5..15),
                },
                Items {
                    item_number: 2,
                    item_name: "Biriyani".to_string(),
                    item_price_usd: 20.50,
                    item_cooking_time_min: rng.gen_range(5..15),
                },
                Items {
                    item_number: 3,
                    item_name: "Mango Juice".to_string(),
                    item_price_usd: 5.0,
                    item_cooking_time_min: rng.gen_range(5..15),
                },
            ];

            // Insert Item Master table data
            for item in total_items.iter() {
                //Insert into table
                client.execute(
                "INSERT INTO Items (item_number, item_name, item_price_usd, item_cooking_time_min) VALUES ($1, $2, $3, $4)",
                &[
                    &item.item_number,
                    &item.item_name,
                    &item.item_price_usd,
                    &item.item_cooking_time_min,
                    ],
                )?;
            }
        }
    }
    Ok(())
}

//insert_tables_master_data function
fn insert_tables_master_data() -> Result<(), PostgresError> {
    //Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    // Insert for first time only.
    for row in client.query("SELECT COUNT(*) as total_tables FROM tables", &[])? {
        let row_count: Option<i64> = row.get(0);
        // If there are already Rows in Database do not insert
        if row_count.unwrap() == 0 {
            // Fill Master data for items
            let total_items = vec![
                Tables {
                    table_number: 1,
                    name: "family table".to_string(),
                    is_table_available: true,
                },
                Tables {
                    table_number: 2,
                    name: "couple table".to_string(),
                    is_table_available: true,
                },
                Tables {
                    table_number: 3,
                    name: "party table".to_string(),
                    is_table_available: true,
                },
            ];

            // Insert Item Master table data
            for item in total_items.iter() {
                //Insert into table
                client.execute(
                "INSERT INTO tables (table_number, name, is_table_available) VALUES ($1, $2, $3)",
                &[
                    &item.table_number,
                    &item.name,
                    &item.is_table_available,
                    ],
                )?;
            }
        }
    }
    Ok(())
}

//set_database function
fn create_table_initialization() -> Result<(), PostgresError> {
    //Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    //Create table
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS Items (
            item_number integer NOT NULL,
            item_name character varying,
            item_price_usd real,
            item_cooking_time_min integer,
            PRIMARY KEY (item_number)
        )",
    )?;
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS tables (
            table_number integer NOT NULL,
            name character varying,
            is_table_available boolean DEFAULT true,
            CONSTRAINT tables_pkey PRIMARY KEY (table_number)
        )",
    )?;
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS order_items (
            order_items_id integer NOT NULL,
            table_number integer NOT NULL,
            item_number integer NOT NULL,
            created_by_name VARCHAR NOT NULL,
            created_date_time timestamp with time zone NOT NULL,
            CONSTRAINT order_items_pkey PRIMARY KEY (order_items_id)
        )",
    )?;
    Ok(())
}

//handle_client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("GET /items") => handle_get_all_items(r),
                r if r.starts_with("GET /tables") => handle_get_all_tables(r),
                r if r.starts_with("GET /orders/") => handle_get_orders_for_table(r),
                r if r.starts_with("GET /order/") => handle_get_single_order_of_table(r),
                r if r.starts_with("POST /order") => handle_post_order_request(r),
                r if r.starts_with("DELETE /order/") => handle_delete_order_request(r),
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

//handle_get_all_items function
fn handle_get_all_items(_request: &str) -> (String, String) {
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
fn handle_get_all_tables(_request: &str) -> (String, String) {
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
fn handle_get_orders_for_table(request: &str) -> (String, String) {
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
fn handle_get_single_order_of_table(request: &str) -> (String, String) {
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
fn handle_post_order_request(request: &str) -> (String, String) {
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
fn handle_delete_order_request(request: &str) -> (String, String) {
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
