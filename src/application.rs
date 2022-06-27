use crate::configuration::Configuration;
use crate::connection::{
    DatabasePool,
};

pub async fn load_config(name: String) ->  Configuration {
    Configuration::initialize(name)
}

pub async fn initialize_pool(config: &Configuration) -> DatabasePool {
    crate::connection::initialize_pool(config)
}