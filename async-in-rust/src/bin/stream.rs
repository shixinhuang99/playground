use mini_redis::client;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;

    println!("done");

    Ok(())
}

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("localhost:6379").await?;

    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "5".into()).await?;
    client.publish("numbers", "six".into()).await?;
    client.publish("numbers", "7".into()).await?;

    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let messages = client::connect("localhost:6379")
        .await?
        .subscribe(vec!["numbers".to_string()])
        .await?
        .into_stream()
        .filter_map(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => Some(msg.content),
            _ => None,
        })
        .take(3);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("{:?}", msg);
    }

    Ok(())
}
