table! {
    product (product_id) {
        product_id -> Int4,
        product_name -> Varchar,
        product_description -> Varchar,
    }
}

table! {
    product_image (product_image_id) {
        product_image_id -> Int4,
        product_id -> Int4,
        image_path -> Varchar,
    }
}

table! {
    product_stock (product_stock_id) {
        product_stock_id -> Int4,
        product_id -> Int4,
        color -> Varchar,
        amount -> Nullable<Int4>,
    }
}

joinable!(product_image -> product (product_id));
joinable!(product_stock -> product (product_id));

allow_tables_to_appear_in_same_query!(
    product,
    product_image,
    product_stock,
);
