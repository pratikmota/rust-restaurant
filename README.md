HOW TO RUN
--
1. sasd
2. asdasd

DATABASE
--
Table details:
we can connect PostgreSQL database using PGAdmin UI
1. Items of menu<br>
![image](https://github.com/pratikmota/rust-restaurant/assets/5825319/898ad4e1-591b-4a26-a46c-912fbee5e602)
2. Total Tables<br>
![image](https://github.com/pratikmota/rust-restaurant/assets/5825319/81ba2ef2-b7fe-484c-a6d1-f780f687ef88)
3. Ordered Items Add/Remove<br>
![image](https://github.com/pratikmota/rust-restaurant/assets/5825319/393677e2-fb93-4b77-8128-1987cc2ffe30)

HOW TO TEST
--
**TEST USING RESTAURANT CLIENT:**
1. Go to client directory - rust-restaurant/restaurant-client
2. cargo build  - ( it will build debug binary)
3. cargo run  - (it will run client)
4. client cmd will run and provide options to perform
![image](https://github.com/pratikmota/rust-restaurant/assets/5825319/95cd4551-19f3-43e4-830c-de9051d8e765)


**DIRECT SERVER TEST USING VSCODE PLUGIN: <br>
Thunder Client Plugin of VSCode for REST API Testing:**
<br>
1. Get all items <br>
GET: localhost:8080/items/ <br>
2. Get Order details from table number and item number <br>
GET: localhost:8080/order/<table_number>/<item_number> <br>
Example: GET: localhost:8080/order/1/1 <br>
3. Get all orders of particular table <br>
GET: localhost:8080/orders/<table_number> <br>
Example: GET: localhost:8080/orders/1 <br>
4. Add new order item <br>
POST: localhost:8080/order <br>
{
    "table_number": 1,
    "item_number": 2,
    "created_by_name": "pratik"
} 
5. delete order item from table <br>
DELETE: localhost:8080/order/<table_number>/<item_number> <br>
Example: DELETE: localhost:8080/order/1/1 <br>

Example:
![image](https://github.com/pratikmota/rust-restaurant/assets/5825319/00cfc6c8-c9ed-442f-aefd-975846f291fb)
