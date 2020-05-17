use elasticsearch::{http::transport::Transport, Elasticsearch};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = Transport::single_node("http://127.0.0.1:9200")?;
    let client = Elasticsearch::new(transport);
    Ok(())
}
