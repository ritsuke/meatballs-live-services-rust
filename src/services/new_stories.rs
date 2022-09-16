pub use super::new_stories::new_stories_server::{NewStories, NewStoriesServer};
use tonic::{Request, Response, Status};

tonic::include_proto!("new_stories");

#[derive(Debug, Default)]
pub struct NewStoriesService {}

#[tonic::async_trait]
impl NewStories for NewStoriesService {
  async fn get_new_stories(
    &self,
    request: Request<NewStoriesRequest>,
  ) -> Result<Response<NewStoriesReply>, Status> {
    println!("Got a request: {:?}", request);

    let reply = super::new_stories::NewStoriesReply {
      message: format!("Done!"),
    };

    Ok(Response::new(reply))
  }
}
