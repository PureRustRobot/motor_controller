use zenoh::{
    config::Config,
    prelude::r#async::*,
};

use futures::select;

use async_std;

#[async_std::main]
async fn main()
{
    let session = zenoh::open(Config::default()).res().await.unwrap();
}