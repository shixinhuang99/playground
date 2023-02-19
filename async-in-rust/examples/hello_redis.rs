use mini_redis::{client as mini_redis_client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = mini_redis_client::connect("localhost:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("result={:?}", result);

    Ok(())
}
