use dotenvy::dotenv;
use std::env;

use tonic::transport::Server;

mod services;
use services::{StoryServer, StoryServices, TestServer, TestServices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let host = env::var("HOST").expect("Missing HOST value; check env...");
  let port = env::var("PORT").expect("Missing PORT value; check env...");
  let address = format!("{}:{}", host, port).parse()?;

  let test_services = TestServer::new(TestServices::default());
  let story_services = StoryServer::new(StoryServices::default());

  let server = Server::builder()
    .accept_http1(true)
    .add_service(tonic_web::enable(test_services))
    .add_service(tonic_web::enable(story_services))
    .serve(address);

  println!("Services available at {}...", address);

  server.await?;

  Ok(())
}
