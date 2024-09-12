CREATE TABLE IF NOT EXISTS Delivery (
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT,
    phone TEXT,
    zip TEXT,
    city TEXT,
    address TEXT,
    region TEXT,
    email TEXT
);
CREATE TABLE IF NOT EXISTS Payment (
    id SERIAL PRIMARY KEY NOT NULL,
    transaction TEXT,
    request_id TEXT,
    currency TEXT,
    provider TEXT,
    amount FLOAT,
    payment_dt INT,
    bank TEXT,
    delivery_cost FLOAT,
    goods_total FLOAT,
    custom_fee TEXT
);
CREATE TABLE IF NOT EXISTS Item (
    chrt_id INT PRIMARY KEY NOT NULL,
    track_number TEXT,
    price FLOAT,
    rid TEXT,
    name TEXT,
    sale FLOAT,
    size TEXT,
    total_price FLOAT,
    nm_id INT,
    brand TEXT,
    status INT
);
CREATE TABLE IF NOT EXISTS Order_Info (
    order_uid TEXT PRIMARY KEY NOT NULL,
    track_number TEXT,
    entry TEXT,
    delivery INT,
    payment INT,
    locale TEXT,
    customer_id TEXT,
    internal_signature TEXT,
    delivery_service TEXT,
    shardkey TEXT,
    sm_id INT,
    date_created TEXT,
    oof_shard TEXT,
    FOREIGN KEY (delivery) REFERENCES Delivery(id),
    FOREIGN KEY (payment) REFERENCES Payment(id)
);
CREATE TABLE IF NOT EXISTS Order_Item (
    id SERIAL PRIMARY KEY NOT NULL,
    order_uid TEXT,
    chrt_id INT,
    FOREIGN KEY (order_uid) REFERENCES Order_Info(order_uid),
    FOREIGN KEY (chrt_id) REFERENCES Item(chrt_id)
);