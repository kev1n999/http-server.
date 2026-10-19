use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn get() -> Self {
        Self {
            host: env::var("SERVER_HOST").expect("SERVER_HOST is not defined in env!"),
            port: env::var("SERVER_PORT")
                .expect("SERVER_PORT is not defined in env!")
                .parse::<u16>()
                .unwrap(),
        }
    }
}
