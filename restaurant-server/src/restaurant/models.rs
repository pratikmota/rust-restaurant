use serde_derive::Deserialize;
use serde_derive::Serialize;

//Model: Items struct with item number, name and cooking time
#[derive(Serialize, Deserialize)]
pub struct Items {
    pub item_number: i32,
    pub item_name: String,
    pub item_price_usd: f32,
    pub item_cooking_time_min: i32,
}

//Model: Tables struct with table number, name and availability
#[derive(Serialize, Deserialize)]
pub struct Tables {
    pub table_number: i32,
    pub name: String,
    pub is_table_available: bool,
}

//Model: OrderItems struct with order id, table number, item number, created name and date
#[derive(Serialize, Deserialize)]
pub struct OrderItems {
    pub table_number: i32,
    pub item_number: i32,
    pub created_by_name: String,
}
