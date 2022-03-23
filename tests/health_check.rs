// use email_subscriptions_backend::run;
// use std::net::TcpListener;

// async fn spawn_app(address: TcpListener) {
//   let server = run(address).expect("Failed to bind address.");
//   let _ = tokio::spawn(server);
// }

// #[actix_rt::test]
// async fn health_check_test() {
//   let listener = TcpListener::bind("!27.0.0.1:0").unwrap("Failed to bind to port");
//   let port = listener.local_addr().unwrap().port();
//   let address = spawn_app(listener);
//   let client = reqwest::Client::new();
//   let response = client
//     .get(&format!("{}/health_check", address))
//     .send()
//     .await
//     .expect("Failed tp execute request");
//   assert!(response.status().is_success());
//   assert_eq!(Some(0), response.content_length());
// }
