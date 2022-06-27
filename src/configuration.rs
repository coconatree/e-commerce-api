
use dotenv::dotenv;

#[allow(dead_code)]
pub struct Configuration {    
    application_name: String,
    app_port: String,
    database_port: String,
    database_name: String,
    database_username: String,
    database_password: String,
}

impl Configuration {
    
    pub fn initialize(application_name: String) -> Self {
        dotenv().ok();
        Self {
            application_name,
            app_port: dotenv::var("PORT").expect("Port number must be set as an env variable"),
            database_port: dotenv::var("DATABASE_PORT").expect("Database port must be set as an env variable."),
            database_name: dotenv::var("DATABASE_NAME").expect("Database password must be set as an env variable"),
            database_username: dotenv::var("DATABASE_USERNAME").expect("Data base password must be set as an env variable"),
            database_password: dotenv::var("DATABASE_PASSWORD").expect("Data base password must be set as an env variable"),
        }
    }

    pub fn get_database_port(&self) -> &String{
        &self.database_port
    }

    pub fn get_database_username(&self) -> &String {
        &self.database_username
    }

    pub fn get_database_password(&self) -> &String {
        &self.database_password
    }

    pub fn get_database_name(&self) -> &String {
        &self.database_name
    }
}

