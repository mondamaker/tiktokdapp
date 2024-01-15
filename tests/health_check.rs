use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
// Arrange
    //spawn_app().await.expect("Failed to spawn our app.");
    /*
    // No .await, no .expect
    //spawn_app_run();

    // No .await, no .expect
    //spawn_app_run_random();

    let client = reqwest::Client::new();
// Act
    let response = client
        .get("http://127.0.0.1:8888/health_check")
        .send()
        .await
        .expect("Failed to execute request.");*/

    // Arrange
    let address = spawn_app_listener();
    let client = reqwest::Client::new();
// Act
    let response = client
// Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

// Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
async fn spawn_app_test() -> std::io::Result<()> {
    Ok(())
}

async fn spawn_app()  { //-> std::io::Result<()> {
    let _ = zero2prod::run();
}
fn spawn_app_run() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}


fn spawn_app_run_random() {
    let server = zero2prod::run_random("127.0.0.1:8888").expect("Failed to bind address");
    let _ = tokio::spawn(server);
}


fn spawn_app_listener() -> String {
    let listener = TcpListener::bind("127.0.0.1:8888")
        .expect("Failed to bind random port");
// We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run_listener(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
// We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}


fn spawn_app_host_subscribe() -> String {
    format!("http://127.0.0.1:{}", "8888")
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
// Arrange
    let app_address = spawn_app_host_subscribe();
    let client = reqwest::Client::new();
// Act
    let body = "name=day-dev&email=day@gmail.com";
    let response = client
        .post(&format!("{}/subscriptions_form", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
    let app_address = spawn_app_host_subscribe();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=day-dev", "missing the name"),
        ("email=day@gmail.com", "missing the email"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {
// Act
        let response = client
            .post(&format!("{}/subscriptions_form" ,&app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

// Assert
        assert_eq!(
            400,
            response.status().as_u16(),
// Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}