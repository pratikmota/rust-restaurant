use exitfailure::ExitFailure;
use serde_derive::Deserialize;
use std::io;

//Model: Items struct with item number, name and cooking time
#[derive(Deserialize, Debug)]
pub struct Items {
    pub item_number: i32,
    pub item_name: String,
    pub item_price_usd: f32,
    pub item_cooking_time_min: i32,
}
//Model: Tables struct with table number, name and availability
#[derive(Deserialize, Debug)]
pub struct Tables {
    pub table_number: i32,
    pub name: String,
    pub is_table_available: bool,
}
//Model: OrderItems struct with order id, table number, item number, created name and date
#[derive(Deserialize, Debug)]
pub struct OrderItems {
    pub table_number: i32,
    pub item_number: i32,
    pub created_by_name: String,
}

//get_user_input take input from cmd line
fn get_user_input() -> io::Result<String> {
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let _ = stdin.read_line(&mut user_input);
    Ok(user_input)
}
fn main() {
    println!("===== CLIENT OPTIONS =====");
    println!("1 | Get All Items");
    println!("2 | Get All Tables");
    println!("3 | Get All Items for Particular Table(input: Table id)");
    println!("4 | Add new Item for Particular Table(input: Table id, Item id)");
    println!("5 | Delete Item for Particular Table(input: Table id, Item id)");
    println!("6 | Get particular Item for particular Table(input: Table id, Item id)");
    println!("7 | EXIT");
    println!("=======================");
    println!("Please enter your input 1, 2, 3, 4, 5, 6 or 7");
    let mut options = get_user_input().unwrap();
    options.pop(); // remote /n

    // Convert String to Integer input
    let input;
    match options.trim().parse::<i32>() {
        Ok(value) => input = value,
        Err(_) => {
            println!("Unable to parse. Please enter valid number.");
            return;
        }
    }

    match input {
        1 => {
            _ = get_all_items();
        }
        2 => {
            _ = get_all_tables();
        }
        3 => {
            _ = get_all_tables();
            println!("Please enter Table Number");
            let mut options = get_user_input().unwrap();
            options.pop(); // remote /n

            // Convert String to Integer input and check
            match options.trim().parse::<i32>() {
                Ok(_value) => {}
                Err(_) => {
                    println!("Unable to parse. Please enter valid number.");
                    return;
                }
            }
            _ = get_all_items_for_table(options);
        }
        4 => println!("Four"),
        5 => println!("Five"),
        6 => println!("Six"),
        _ => println!("Invalid Input"),
    }
}

//get_all_items function used to get all items
fn get_all_items() -> Result<(), ExitFailure> {
    println!("===== Total Items =====");
    let client = reqwest::blocking::Client::new();
    let res = client.get("http://localhost:8080/items/").send();
    let items: Vec<Items> = res.unwrap().json()?;
    for it in items {
        println! {"item_number:{} cooking_time:{} item_name:{}", it.item_number,it.item_cooking_time_min, it.item_name}
    }
    println!("==========");
    Ok(())
}

//get_all_tables function used to get all tables list
fn get_all_tables() -> Result<(), ExitFailure> {
    println!("===== Total Tables =====");
    let client = reqwest::blocking::Client::new();
    let res = client.get("http://localhost:8080/tables/").send();
    let items: Vec<Tables> = res.unwrap().json()?;
    for it in items {
        println! {"table_number:{} name:{}", it.table_number,it.name}
    }
    println!("==========");
    Ok(())
}

//get_all_items_for_table function used to get all items for table
fn get_all_items_for_table(table_number: String) -> Result<(), ExitFailure> {
    println!("===== Total Items in Table =====");
    let client = reqwest::blocking::Client::new();
    let mut url: String = "http://localhost:8080/orders/".to_string();
    url.push_str(&table_number);
    let res = client.get(url).send();
    let items: Vec<OrderItems> = res.unwrap().json()?;
    for it in items {
        println! {"table_number:{} item_number:{}", it.table_number,it.item_number}
    }
    println!("==========");
    Ok(())
}
