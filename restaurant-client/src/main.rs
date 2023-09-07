use std::io;

//get_user_input take input from cmd line
fn get_user_input() -> io::Result<String> {
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let _ = stdin.read_line(&mut user_input);
    Ok(user_input)
}
fn main() {
    println!("1 | Get All Items");
    println!("2 | Get All Tables");
    println!("3 | Get All Items for Particular Table(input: Table id)");
    println!("4 | Add new Item for Particular Table(input: Table id, Item id)");
    println!("5 | Delete Item for Particular Table(input: Table id, Item id)");
    println!("6 | Get particular Item for particular Table(input: Table id, Item id)");
    println!("=======================");
    println!("Please enter your input 1, 2, 3, 4, 5 or 6");
    let mut options = get_user_input().unwrap();
    options.pop(); // remote /n

    // Convert String to Integer iput
    let mut input = -1;
    match options.trim().parse::<i32>() {
        Ok(value) => input = value,
        Err(_) => {
            println!("Unable to parse. Please enter valid number.");
        }
    }

    match input {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        6 => println!("Six"),
        _ => println!("Rest of the number"),
    }
}
