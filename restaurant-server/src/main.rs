use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

//DATABASE_URL
//"postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
const DB_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
//env!("DATABASE_URL");

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

fn main() {
    //Set database
    if let Err(e) = set_database() {
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
            created_by_name character varying NOT NULL,
            created_date_time timestamp without time zone NOT NULL,
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
                r if r.starts_with("GET /users/") => handle_get_request(r),
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

//handle_get_request function
fn handle_get_request(request: &str) -> (String, String) {
    (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
}
