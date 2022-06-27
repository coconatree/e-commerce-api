-- Product Table Migration Up

CREATE TABLE IF NOT EXISTS product (
    product_id          SERIAL PRIMARY KEY,
    product_name        VARCHAR(50)  NOT NULL,
    product_description VARCHAR(150) NOT NULL
);