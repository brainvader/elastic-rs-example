extern crate elasticsearch_rs_examples as MY;

use elasticsearch::{http::transport::Transport, Elasticsearch};
use elasticsearch::{GetParts, IndexParts};

use serde::{Deserialize, Serialize};
// use serde_json::json;

use MY::api;

#[derive(Serialize, Deserialize, Debug)]
struct Customer {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    MY::setup_logger()?;
    let url = MY::es_url()?;
    let transport = Transport::single_node(url.as_str())?;
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

    let index_response = customer_index.body(&john_doe).send().await?;
    if index_response.status_code().is_success() {
        let response_body = index_response.json::<api::index::ResponseBody>().await?;
        log::info!("{:#?}", response_body);
    }

    let response = client
        .get(GetParts::IndexId("customer", "1"))
        .send()
        .await?;
    if response.status_code().is_success() {
        let response_body: api::get::ResponseBody<Customer> =
            response.json::<api::get::ResponseBody<Customer>>().await?;
        // let response_body = response.json::<Value>().await?;
        log::info!("{:#?}", response_body);
    }
    Ok(())
}
