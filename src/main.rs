use dotenvy::dotenv;
use std::env;

use tonic::transport::Server;

mod services;
use services::new_stories::{NewStoriesServer, NewStoriesService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let port = env::var("PORT").expect("Missing PORT value; check env...");
  let address = format!("0.0.0.0:{}", port).parse()?;

  let new_stories_service = NewStoriesServer::new(NewStoriesService::default());

  let server = Server::builder()
    .accept_http1(true)
    .add_service(tonic_web::enable(new_stories_service))
    .serve(address);

  println!("Services server listening at {}...", address);

  server.await?;

  Ok(())
}
