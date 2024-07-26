use actix_web::{web, HttpResponse};
use serde_json::json;
use chrono::NaiveDate;
use log::{info, error, debug};

pub async fn validate_token(token: web::Path<String>) -> Result<String, Box<dyn std::error::Error>> {
    info!("Received token: {}", token);

    let is_valid = validate_token_format(&token);

    info!("Token validation result: {}", is_valid);

    let response = json!({
        "rsp": if is_valid { "true" } else { "false" }
    }).to_string();

    Ok(response)
}

fn validate_token_format(token: &str) -> bool {
    debug!("Validating token: {}", token);

    if token.len() != 40 {
        error!("Token length is not 40");
        return false;
    }

    let date_str = &token[..8];
    debug!("Date string: {}", date_str);

    if let Err(e) = NaiveDate::parse_from_str(date_str, "%Y%m%d") {
        error!("Date parsing failed: {:?}", e);
        return false;
    }

    let hex_part_valid = token[8..].chars().all(|c| c.is_ascii_hexdigit());
    debug!("Hex part valid: {}", hex_part_valid);

    hex_part_valid
}