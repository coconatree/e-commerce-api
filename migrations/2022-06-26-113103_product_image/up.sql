-- Product Image Table Up

CREATE TABLE IF NOT EXISTS product_image(
    product_image_id SERIAL PRIMARY KEY,
    product_id       INTEGER NOT NULL,
    image_path       VARCHAR(50) NOT NULL, 
    FOREIGN KEY (product_id) REFERENCES product(product_id)
);
