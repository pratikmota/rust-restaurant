CREATE TABLE IF NOT EXISTS public.items
(
    item_number integer NOT NULL,
    item_name character varying COLLATE pg_catalog."default",
    item_price_usd real,
    item_cooking_time_min integer,
    CONSTRAINT items_pkey PRIMARY KEY (item_number)
)

select * from items

INSERT INTO public.items(
	item_number, item_name, item_price_usd, item_cooking_time_min)
	VALUES (?, ?, ?, ?);

-- 
CREATE TABLE IF NOT EXISTS public.order_items
(
    order_items_id integer NOT NULL,
    table_number integer NOT NULL,
    item_number integer NOT NULL,
    created_by_name character varying COLLATE pg_catalog."default" NOT NULL,
    created_date_time timestamp with time zone NOT NULL,
    CONSTRAINT order_items_pkey PRIMARY KEY (order_items_id)
)

select * from order_items where table_number=1 and item_number=1

INSERT INTO public.order_items(
	order_items_id, table_number, item_number, created_by_name, created_date_time)
	VALUES (?, ?, ?, ?, ?);
--
CREATE TABLE IF NOT EXISTS public.tables
(
    table_number integer NOT NULL,
    name character varying COLLATE pg_catalog."default",
    is_table_available boolean DEFAULT true,
    CONSTRAINT tables_pkey PRIMARY KEY (table_number)
)

select * from tables

INSERT INTO public.tables(
	table_number, name, is_table_available)
	VALUES (?, ?, ?);
