use error_chain::error_chain;
use reqwest;
use tokio;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;

    println!("Status: {}", res.status());
    println!("Headers\n: {:#?}", res.headers());
    println!("Body: {}", res.text().await?);

    Ok(())
}