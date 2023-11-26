use reqwest;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    
    let client = reqwest::Client::new();
    let response = client 
        .get("localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let server = newsletter::run("localhost").expect("Failed to bind.");

    let _ = tokio::spawn(server);
}
