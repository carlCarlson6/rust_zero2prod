#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let response = reqwest::Client::new()
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("fail to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = rust_zero2prod::run().expect("failed to bind address");
    let _ = tokio::spawn(server);
}
