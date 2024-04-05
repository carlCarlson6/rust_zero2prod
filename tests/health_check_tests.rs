use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app();

    let response = reqwest::Client::new()
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("fail to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = rust_zero2prod::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
