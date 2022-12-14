// https://github.com/hyperium/tonic/blob/master/examples/src/grpc-web/client.rs
use bytes::{Buf, BufMut, Bytes, BytesMut};
use http::header::{ACCEPT, CONTENT_TYPE};
use hyper_tls::HttpsConnector;

mod services;
use services::{PingReply, PingRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let ping_msg = PingRequest {};

  println!("{:?}", ping_msg);

  let https = HttpsConnector::new();

  let ping_request = http::Request::builder()
    .version(http::Version::HTTP_11)
    .method(http::Method::POST)
    .uri("http://0.0.0.0:51234/services.Test/Ping")
    .header(CONTENT_TYPE, "application/grpc-web")
    .header(ACCEPT, "application/grpc-web")
    .body(hyper::Body::from(encode_body(ping_msg)))
    .unwrap();

  let client = hyper::Client::builder().build::<_, hyper::Body>(https);

  let ping_response = client.request(ping_request).await.unwrap();

  assert_eq!(
    ping_response.headers().get(CONTENT_TYPE).unwrap(),
    "application/grpc-web+proto"
  );

  let ping_body = ping_response.into_body();
  let ping_reply = decode_body::<PingReply>(ping_body).await;

  println!("REPLY={:?}", ping_reply);

  Ok(())
}

// one byte for the compression flag plus four bytes for the length
const GRPC_HEADER_SIZE: usize = 5;

fn encode_body<T>(msg: T) -> Bytes
where
  T: prost::Message,
{
  let mut buf = BytesMut::with_capacity(1024);

  // first skip past the header
  // cannot write it yet since we don't know the size of the
  // encoded message
  buf.reserve(GRPC_HEADER_SIZE);
  unsafe {
    buf.advance_mut(GRPC_HEADER_SIZE);
  }

  // write the message
  msg.encode(&mut buf).unwrap();

  // now we know the size of encoded message and can write the
  // header
  let len = buf.len() - GRPC_HEADER_SIZE;
  {
    let mut buf = &mut buf[..GRPC_HEADER_SIZE];

    // compression flag, 0 means "no compression"
    buf.put_u8(0);

    buf.put_u32(len as u32);
  }

  buf.split_to(len + GRPC_HEADER_SIZE).freeze()
}

async fn decode_body<T>(body: hyper::Body) -> T
where
  T: Default + prost::Message,
{
  let mut body = hyper::body::to_bytes(body).await.unwrap();

  // ignore the compression flag
  body.advance(1);

  let len = body.get_u32();
  #[allow(clippy::let_and_return)]
  let msg = T::decode(&mut body.split_to(len as usize)).unwrap();

  msg
}
