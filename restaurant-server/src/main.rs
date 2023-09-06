use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
mod restaurant;
use restaurant::db;
use restaurant::handlers;

#[macro_use]
extern crate dotenv_codegen;

fn main() {
    //Create Tables if not exist.
    if let Err(e) = db::create_table_initialization() {
        println!("Error: {}", e);
        return;
    }
    // Insert item table data if not exist ( Master table entry)
    if let Err(e) = db::insert_item_master_data() {
        println!("Error: {}", e);
        return;
    }
    // Insert tables data if not exist ( Master table entry)
    if let Err(e) = db::insert_tables_master_data() {
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

//handle_client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("GET /items") => handlers::handle_get_all_items(r),
                r if r.starts_with("GET /tables") => handlers::handle_get_all_tables(r),
                r if r.starts_with("GET /orders/") => handlers::handle_get_orders_for_table(r),
                r if r.starts_with("GET /order/") => handlers::handle_get_single_order_of_table(r),
                r if r.starts_with("POST /order") => handlers::handle_post_order_request(r),
                r if r.starts_with("DELETE /order/") => handlers::handle_delete_order_request(r),
                _ => (
                    restaurant::constants::NOT_FOUND.to_string(),
                    "404 Not Found".to_string(),
                ),
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
