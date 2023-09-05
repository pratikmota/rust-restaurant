use postgres::types::Date;
use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, SystemTime};

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
    order_items_id: i32,
    table_number: i32,
    item_number: i32,
    created_by_name: String,
}

fn main() {
    //Set database
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }

    /*
    if let Err(e) = insert_database() {
        println!("Error: {}", e);
        return;
    }
    */

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

//insert_database function
fn insert_database() -> Result<(), PostgresError> {
    //Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    let item = Items {
        item_number: 2,
        item_name: "Biriyani".to_string(),
        item_cooking_time_min: 10,
    };
    //Insert into table
    client.execute(
        "INSERT INTO Items (item_number, item_name, item_cooking_time_min) VALUES ($1, $2, $3)",
        &[
            &item.item_number,
            &item.item_name,
            &item.item_cooking_time_min,
        ],
    )?;

    Ok(())
}

//set_database function
fn set_database() -> Result<(), PostgresError> {
    //Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    //Create table
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS Items (
            item_number integer NOT NULL,
            item_name character varying,
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
                r if r.starts_with("POST /order") => handle_post_order_request(r),
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
                    item_cooking_time_min: row.get(2),
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
            client
                .execute(
                    "INSERT INTO order_items (order_items_id, table_number, item_number, created_by_name, created_date_time ) VALUES ($1, $2, $3, $4, $5)",
                    &[&orders.order_items_id, &orders.table_number,  &orders.item_number, &orders.created_by_name, &now],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Order Item created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//deserialize OrderItems from request body
fn get_request_body(request: &str) -> Result<OrderItems, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
