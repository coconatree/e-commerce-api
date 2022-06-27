use crate::schema::product::{
    product_id,
    dsl::*,
};

use diesel::prelude::*;

use serde_derive::{Serialize};

#[derive(Serialize, Queryable)]
pub struct ProductModel  {
    product_id:          i32,
    product_name:        String,
    product_description: String,
}

impl ProductModel {

    pub fn select(id: i32, conn: &PgConnection) -> ProductModel {
        product.filter(product_id.eq(id)).limit(1).get_result(conn).expect(&format!("Error selecting a user with id {}", id))
    }
    
    pub fn select_all(conn: &PgConnection) -> Vec<ProductModel> {
       product.limit(50).load(conn).expect("Error selecting all of the products") 
    }
}