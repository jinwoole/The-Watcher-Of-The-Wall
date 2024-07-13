use actix_rt;
use awc::Client;

#[actix_rt::test]
async fn api_check_test() {
    let client = Client::default();

    let response = client.get("http://localhost:8080/api/check/abcde/12345")
        .send()
        .await;

    match response {
        Ok(mut resp) => {
            let body = resp.body().await;

            match body {
                Ok(bytes) => {
                    let response_text = std::str::from_utf8(&bytes).unwrap();
                    println!("[Server Response]: {}", response_text);
                    assert_eq!(response_text, "Database connection is healthy. Query: abcde, 12345");
                    assert!(resp.status().is_success());
                }
                Err(e) => {
                    println!("Failed to read response body: {:?}", e);
                    panic!("Test failed due to body read error");
                }
            }
        }
        Err(e) => {
            println!("Request failed: {:?}", e);
            panic!("Test failed due to request error");
        }
    }
}
