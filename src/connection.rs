use diesel::{
    pg::PgConnection, 
    r2d2::ConnectionManager
};

use crate::configuration::Configuration;

pub type DatabasePool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn initialize_pool(config: &Configuration)  -> DatabasePool {
    let manager = ConnectionManager::<PgConnection>::new(
        create_database_url_from_config(config)
    );
    let pool_result  = r2d2::Pool::builder().build(manager);

    match pool_result {
        Ok(pool) => pool,
        Err(err) => panic!("Error: {}", err), 
    }
}

fn create_database_url_from_config(config: &Configuration) -> String {
    // postgres://username:password@localhost/diesel_demo
    String::from(format!(
        "postgres://{username}:{password}@localhost/{name}",
        // port = config.get_database_port(),  
        username = config.get_database_username(),
        password = config.get_database_password(),
        name = config.get_database_name(),
    ))
}
