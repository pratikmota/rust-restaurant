use crate::restaurant::models::{Items, Tables};
use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use rand::Rng;

//"postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
const DB_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
//env!("DATABASE_URL");

//insert_database function
pub fn insert_item_master_data() -> Result<(), PostgresError> {
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
pub fn insert_tables_master_data() -> Result<(), PostgresError> {
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
pub fn create_table_initialization() -> Result<(), PostgresError> {
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
