use lazy_static::lazy_static;
use redis::{Client, Commands, RedisResult};
use log::{info};

lazy_static! {
    static ref REDIS: Client = {
        let redis_url = "redis://redis:6379".to_string();
        Client::open(redis_url.clone()).unwrap_or_else(|e| {
            info!("Failed to connect to Redis at {}: {}", redis_url, e);
            std::process::exit(1);
        })
    };
}

pub fn store_token(token: &str) -> RedisResult<()> {
    info!("Attempting to store token: {}", token);
    match REDIS.get_connection() {
        Ok(mut con) => {
            match con.set::<_, _, ()>(token, "valid") {
                Ok(_) => {
                    info!("Token stored successfully: {}", token);
                    Ok(())
                },
                Err(e) => {
                    info!("Failed to store token: {}. Error: {}", token, e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            info!("Failed to get Redis connection for storing token. Error: {}", e);
            Err(e)
        }
    }
}

pub fn check_token(token: &str) -> RedisResult<bool> {
    info!("Checking token: {}", token);
    match REDIS.get_connection() {
        Ok(mut con) => {
            match con.exists(token) {
                Ok(exists) => {
                    info!("Token check result for {}: exists = {}", token, exists);
                    Ok(exists)
                },
                Err(e) => {
                    info!("Error checking token: {}. Error: {}", token, e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            info!("Failed to get Redis connection for checking token. Error: {}", e);
            Err(e)
        }
    }
}

pub fn clear_all_tokens() -> RedisResult<()> {
    info!("Attempting to clear all tokens");
    match REDIS.get_connection() {
        Ok(mut con) => {
            match redis::cmd("FLUSHDB").query::<()>(&mut con) {
                Ok(_) => {
                    info!("All tokens cleared successfully");
                    Ok(())
                },
                Err(e) => {
                    info!("Failed to clear all tokens. Error: {}", e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            info!("Failed to get Redis connection for clearing tokens. Error: {}", e);
            Err(e)
        }
    }
}