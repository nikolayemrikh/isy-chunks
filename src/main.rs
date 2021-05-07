use std::path::Path;
use dotenv::dotenv;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use tokio::fs::File;
mod stream_spliiter;

async fn upload_chunk(chunk_path: &str) {
  let file_name = Path::new(&chunk_path).file_name().unwrap();
  let file_name = file_name.to_str().unwrap();
  let creds = Credentials::new(None, None, None, None, None).unwrap();
  let bucket = Bucket::new("isy-chunks", "eu-north-1".parse().unwrap(), creds).unwrap();

  let mut file = File::open(&chunk_path).await.unwrap();
  
  let status_code = bucket.put_object_stream(&mut file, &file_name).await.unwrap();
  println!("{}", status_code);
}

#[tokio::main]
async fn main() {
  #[derive(Debug)]
  struct Square(i32);

  dotenv().ok();

  stream_spliiter::split_into_chunks().await;

  upload_chunk("~/Documents/learn/rust/chunks/001.mp4").await
}

