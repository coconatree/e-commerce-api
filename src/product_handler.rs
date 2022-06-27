use actix_web::HttpResponse;
use actix_web::{
    Responder,
    get,
    web::Path,
    web::Data,
};

use image::{DynamicImage, ImageFormat};

use std::fs::File;
use std::io::{BufReader, Cursor, Read};

use crate::connection::DatabasePool;
use crate::model::ProductModel;

#[get("/api/v1/product/all/")]
pub async fn get_all_products(pool: Data<DatabasePool>) -> impl Responder {
    
    let conn = pool.get()
        .expect("Failed to retrieve connetion");

    let result = ProductModel::select_all(&conn);

    HttpResponse::Ok().json(result)
}

#[get("api/v1/product/{product_id}")]
pub async fn get_product(pool: Data<DatabasePool>, path: Path<i32>) -> impl Responder {

    let id = path.into_inner();
    let conn = pool.get()
        .expect("Error retrieving the connection");

    let result = ProductModel::select(id, &conn);

    HttpResponse::Ok().json(result)
}

#[get("api/v1/product/image/")]
pub async fn get_product_image() -> impl Responder {
    let image = convert_png_to_base_64();
    HttpResponse::Ok().json(image)
}

fn load_image_from_path(path: &str) -> DynamicImage {
    let file = File::open("static/image/test.png")
        .expect("Failed to open the image !!!");       

    image::load(BufReader::new(file), ImageFormat::Png)
        .expect("Failed to load the image !!!")
}

fn convert_png_to_base_64() -> String {
    
    let mut buf = Cursor::new(vec![]);
    
    let base_img = image::open("static/image/test.png")
        .expect("Error opening the image");     

    base_img.write_to(&mut buf, image::ImageOutputFormat::Png)
        .expect("Failed to write image to the buffer !!!");

    base64::encode(buf.into_inner())
}
