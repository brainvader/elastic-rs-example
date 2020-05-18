extern crate elasticsearch_rs_examples as MY;

use elasticsearch::IndexParts;
use elasticsearch::{http::transport::Transport, Elasticsearch};

use serde::Serialize;
// use serde_json::json;

use MY::api;

#[derive(Serialize)]
struct Customer {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = Transport::single_node("http://127.0.0.1:9200")?;
    let client = Elasticsearch::new(transport);

    // Rest API version
    /*
        PUT /customer/_doc/1
    {
        "name": "John Doe"
    }
        */
    let customer_index = client.index(IndexParts::IndexId("customer", "1"));
    let john_doe = Customer {
        name: "John Doe".to_owned(),
    };

    // or use json macro in serde create instead
    // let john_doe = json!({
    //     "name": "John Doe"
    // });

    let index_response = customer_index.body(john_doe).send().await?;
    let success = index_response.status_code().is_success();
    if success {
        let response_body = index_response.json::<api::index::ResponseBody>().await?;
        println!("{:#?}", response_body);
    }
    Ok(())
}
