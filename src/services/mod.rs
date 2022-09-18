use tonic::{Request, Response, Status};

mod ingest;
use ingest::{new_stories, story_activity};

mod generate;
use generate::collections;

pub use test_server::{Test, TestServer};

tonic::include_proto!("services");

#[derive(Debug, Default)]
pub struct TestServices {}

#[tonic::async_trait]
impl Test for TestServices {
  async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
    println!("PING");

    let reply = PingReply {
      message: format!("PONG"),
    };

    Ok(Response::new(reply))
  }
}

pub use story_server::{Story, StoryServer};

#[derive(Debug, Default)]
pub struct StoryServices {}

#[tonic::async_trait]
impl Story for StoryServices {
  async fn ingest_new_stories(
    &self,
    request: Request<IngestNewStoriesRequest>,
  ) -> Result<Response<IngestNewStoriesReply>, Status> {
    println!("Got a request: {:?}", request);

    new_stories::process();

    let reply = IngestNewStoriesReply {
      message: format!("Done!"),
    };

    Ok(Response::new(reply))
  }

  async fn ingest_story_activity(
    &self,
    request: Request<IngestStoryActivityRequest>,
  ) -> Result<Response<IngestStoryActivityReply>, Status> {
    println!("Got a request: {:?}", request);

    story_activity::process();

    let reply = IngestStoryActivityReply {
      message: format!("Done!"),
    };

    Ok(Response::new(reply))
  }

  async fn generate_collections(
    &self,
    request: Request<GenerateCollectionsRequest>,
  ) -> Result<Response<GenerateCollectionsReply>, Status> {
    println!("Got a request: {:?}", request);

    collections::process();

    let reply = GenerateCollectionsReply {
      message: format!("Done!"),
    };

    Ok(Response::new(reply))
  }
}
