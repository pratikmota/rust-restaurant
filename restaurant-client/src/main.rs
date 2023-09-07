use exitfailure::ExitFailure;
use serde_derive::Deserialize;
use serde_derive::Serialize;
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
#[derive(Serialize, Deserialize, Debug)]
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
    loop {
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
            4 => {
                _ = get_all_tables();
                println!("Please enter Table Number:");
                // Convert String to Integer input
                let table_num;
                let mut options = get_user_input().unwrap();
                options.pop(); // remote /n
                match options.trim().parse::<i32>() {
                    Ok(value) => table_num = value,
                    Err(_) => {
                        println!("Unable to parse. Please enter valid number.");
                        return;
                    }
                }
                // check if any new items available or empty
                if let Ok(is_empty) = get_all_remaining_items(options) {
                    if is_empty {
                        println!("No new item available");
                        break;
                    }
                }

                println!("Please enter Item Number:");
                let item_num;
                let mut options = get_user_input().unwrap();
                options.pop(); // remote /n
                match options.trim().parse::<i32>() {
                    Ok(value) => item_num = value,
                    Err(_) => {
                        println!("Unable to parse. Please enter valid number.");
                        return;
                    }
                }

                _ = add_new_item_for_table(table_num, item_num);
            }
            5 => {
                _ = get_all_tables();
                println!("Please enter Table Number:");
                // Convert String to Integer input
                let mut table_num = get_user_input().unwrap();
                table_num.pop(); // remote /n
                match table_num.trim().parse::<i32>() {
                    Ok(_value) => {}
                    Err(_) => {
                        println!("Unable to parse. Please enter valid number.");
                        return;
                    }
                }

                let tbl = table_num.clone();
                _ = get_all_items_for_table(tbl);

                println!("Please enter Item Number:");
                let mut item_num = get_user_input().unwrap();
                item_num.pop(); // remote /n
                                // check if input invalid
                match item_num.trim().parse::<i32>() {
                    Ok(_value) => {}
                    Err(_) => {
                        println!("Unable to parse. Please enter valid number.");
                        return;
                    }
                }
                //let table_number = table_num.clone();
                _ = delete_item_for_table(&table_num, &item_num);
            }
            6 => println!("Six"),
            7 => {
                return;
            }
            _ => println!("Invalid Input"),
        }
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

//get_all_remaining_items function used to get all items which not placed
fn get_all_remaining_items(table_num: String) -> Result<bool, ExitFailure> {
    println!("===== Total Available Items =====");
    let mut is_empty: bool = true;
    let client = reqwest::blocking::Client::new();
    let res = client.get("http://localhost:8080/items/").send();
    let items: Vec<Items> = res.unwrap().json()?;
    // Get items from order
    let mut url: String = "http://localhost:8080/orders/".to_string();
    url.push_str(&table_num);
    let res = client.get(url).send();
    let orders: Vec<OrderItems> = res.unwrap().json()?;
    let items_of_table = &orders;
    // final filter
    let mut is_already_variable: bool;
    for it in items {
        is_already_variable = false;
        for order_item in items_of_table {
            if order_item.item_number == it.item_number {
                is_already_variable = true;
                break;
            }
        }
        // If new then only print
        if is_already_variable == false {
            println! {"item_number:{} cooking_time:{} item_name:{}", it.item_number,it.item_cooking_time_min, it.item_name}
            is_empty = false;
        }
    }
    println!("==========");
    Ok(is_empty)
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
    let orders: Vec<OrderItems> = res.unwrap().json()?;
    // Get Items
    let client_item = reqwest::blocking::Client::new();
    let res_itm = client_item.get("http://localhost:8080/items/").send();
    let items: Vec<Items> = res_itm.unwrap().json()?;
    let total_items = &items;
    let mut item_name: String = "".to_string();

    // Joint Order amd Item table and print data
    for ord in orders {
        for it in total_items {
            if it.item_number == ord.item_number {
                item_name = it.item_name.clone();
                break;
            }
        }
        println! {"item_name:{} table_number:{} item_number:{}", item_name, ord.table_number,ord.item_number}
    }
    println!("==========");
    Ok(())
}

//add_new_item_for_table function used to add item for particular table
fn add_new_item_for_table(table_num: i32, item_num: i32) -> Result<(), ExitFailure> {
    println!("===== Inserting Item =====");
    let data = OrderItems {
        item_number: item_num,
        table_number: table_num,
        created_by_name: "admin".to_string(),
    };

    let client = reqwest::blocking::Client::new();
    let _res = client
        .post("http://localhost:8080/order")
        .json(&data)
        .header("Content-Type", "application/json; charset=utf-8")
        .send()?
        .json()?;
    //println!("{:?}", res);
    println!("item added !!");
    Ok(())
}

//delete_item_for_table function used to delete item for particular table
fn delete_item_for_table(table_num: &String, item_num: &String) -> Result<(), ExitFailure> {
    println!("===== Deleting Item =====");
    let client = reqwest::blocking::Client::new();
    let mut url: String = "http://localhost:8080/order/".to_string();
    url.push_str(table_num);
    url.push_str("/");
    url.push_str(item_num);
    if let Ok(_res) = client.delete(url).send() {
        println!("Item deleted !!");
    } else {
        println!("Error in deleting item !!");
    }
    println!("==========");
    Ok(())
}
