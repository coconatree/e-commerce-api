-- Your SQL goes here

CREATE TABLE IF NOT EXISTS product_stock (
    
    product_stock_id SERIAL PRIMARY KEY,
    product_id       INTEGER NOT NULL,
    color            VARCHAR(25) NOT NULL,
    amount           INTEGER,
    
    FOREIGN KEY (product_id) REFERENCES product(product_id)
);