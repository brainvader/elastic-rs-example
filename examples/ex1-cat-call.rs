extern crate elasticsearch_rs_examples as MY;

use elasticsearch::{http::transport::Transport, Elasticsearch};
use MY::ESNode;

/*
// Reference
// https://www.elastic.co/guide/en/elasticsearch/reference/current/cat.html#headers
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    MY::setup_logger()?;

    let url = MY::es_url()?;
    let transport = Transport::single_node(url.as_str())?;
    let client = Elasticsearch::new(transport);
    let cat_client = client.cat();
    let nodes = cat_client
        .nodes()
        .h(&["ip", "port", "heapPercent", "name"])
        .format("json");
    let response = nodes.send().await?;

    // let response_body = response.text().await?;
    // dbg!(response_body);

    let response_body: Vec<ESNode> = response.json::<Vec<ESNode>>().await?;
    if let Some(es_node) = response_body.iter().take(1).next() {
        log::info!("{:#?}", es_node);
    }
    Ok(())
}
