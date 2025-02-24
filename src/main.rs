use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("http://httpbin.org/get").await?;

    println!("Status: {}", res.status());
    println!("Body: {}", res.text().await?);

    Ok(())
}

