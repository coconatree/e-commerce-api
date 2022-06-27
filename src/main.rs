#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;

#[path = "application.rs"]
pub mod app;

#[path = "product_handler.rs"]
pub mod handler;

#[path = "product_model.rs"]
pub mod model;

#[path = "configuration.rs"]
pub mod configuration;

#[path = "connection.rs"]
pub mod connection;

#[path = "application.rs"]
pub mod application;

#[path = "middleware/request_logger.rs"]
pub mod middleware;

use actix_web::{
    App, 
    HttpServer,
    web::Data,
    middleware::Logger,
};

use actix_cors::Cors;

use crate::handler::{
    get_product,
    get_all_products,
    get_product_image
};

use crate::application::{
    initialize_pool,
    load_config,
};

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = load_config(String::from("Application ")).await;
    let pool = initialize_pool(&config).await;

    println!("Pool initialized !!!");

    HttpServer::new(move || {
        
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(get_product)
            .service(get_all_products)
            .service(get_product_image)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}