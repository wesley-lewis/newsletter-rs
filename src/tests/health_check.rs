use std::net::TcpListener;

use reqwest;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    
    let client = reqwest::Client::new();
    dbg!(&address);
    let response = client 
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(19), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind.");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
