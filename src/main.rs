use email_subscriptions_backend::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    println!("Running server on 127.0.0.1 PORT {}", &port);
    run(listener)?.await
}
