use actix_web::{web};
use serde_json::json;
use chrono::NaiveDate;
use log::{info, debug};
use crate::redis_control;


pub async fn validate_token(token: web::Path<String>) -> Result<String, Box<dyn std::error::Error>> {
    info!("Received token: {}", token);

    let format_valid = validate_token_format(&token);
    let redis_valid = redis_control::check_token(&token).unwrap_or(false);

    info!("Format validation result: {}", format_valid);
    info!("Redis validation result: {}", redis_valid);

    let is_valid = format_valid && redis_valid;

    info!("Token validation result: {}", is_valid);

    let response = json!({
        "rsp": if is_valid { "true" } else { "false" }
    }).to_string();

    Ok(response)
}

fn validate_token_format(token: &str) -> bool {
    debug!("Validating token: {}", token);

    if token.len() != 40 {
        info!("Token length is not 40");
        return false;
    }

    let date_str = &token[..8];
    debug!("Date string: {}", date_str);

    if let Err(e) = NaiveDate::parse_from_str(date_str, "%Y%m%d") {
        info!("Date parsing failed: {:?}", e);
        return false;
    }

    let hex_part_valid = token[8..].chars().all(|c| c.is_ascii_hexdigit());
    debug!("Hex part valid: {}", hex_part_valid);

    hex_part_valid
}