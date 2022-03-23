use email_subscriptions_backend::run;

async fn spawn_app() {
  let server = run().expect("Failed to bind address.");
  let _ = tokio::spawn(server);
}

#[actix_rt::test]
async fn health_check_test() {
  spawn_app();
  let client = reqwest::Client::new();
  let response = client
    .get("http://127.0.0.1:3000/health_check")
    .send()
    .await
    .expect("Failed tp execute request");
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}
