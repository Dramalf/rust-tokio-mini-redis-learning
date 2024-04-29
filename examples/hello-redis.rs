use mini_redis::{client, Result};
use tokio;
#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;
    let mut client2 = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    // let result = client.get("hello").await?;
    // let result= tokio::join!( client.get("hello"),  client2.get("hello"));
    let result1=client.get("hello").await;

    let result2=client2.get("hello").await;
    println!("1 got value from the server; result={:?}", result1);
    println!("2 got value from the server; result={:?}", result2);
    Ok(())
}