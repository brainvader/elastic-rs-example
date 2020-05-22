extern crate elasticsearch_rs_examples as MY;

use std::vec::Vec;

use elasticsearch::http::{request::JsonBody, transport::Transport};
use elasticsearch::{BulkParts, Elasticsearch};

use serde::{Deserialize, Serialize};
use serde_json::json;

use chrono::prelude::*;

use MY::api;

#[derive(Serialize, Deserialize, Debug)]
struct Tweet {
    id: i64,
    user: String,
    post_date: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date_to_str = |date| format!("{:?}", date);

    let bulk_index_1 = api::bulk::request::Action::Index {
        params: api::bulk::request::ActionParams {
            id: Some("1".to_owned()),
            index: None,
        },
    };

    let tweet_1 = Tweet {
        id: 1,
        user: "kimchy".to_owned(),
        post_date: date_to_str(Utc.ymd(2009, 11, 15).and_hms(00, 00, 00)),
        message: "Trying out Elasticsearch, so far so good?".to_owned(),
    };

    let bulk_index_2 = api::bulk::request::Action::Index {
        params: api::bulk::request::ActionParams {
            id: Some("2".to_owned()),
            index: None,
        },
    };

    let tweet_2 = Tweet {
        id: 2,
        user: "forloop".to_owned(),
        post_date: date_to_str(Utc.ymd(2020, 01, 08).and_hms(00, 00, 00)),
        message: "Bulk indexing with the rust client, yeah!".to_owned(),
    };

    let transport = Transport::single_node("http://127.0.0.1:9200")?;
    let client: Elasticsearch = Elasticsearch::new(transport);

    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);
    body.push(serde_json::to_value(bulk_index_1)?.into());
    body.push(serde_json::to_value(tweet_1)?.into());
    body.push(serde_json::to_value(bulk_index_2)?.into());
    body.push(serde_json::to_value(tweet_2)?.into());

    let response = client
        .bulk(BulkParts::Index("tweets"))
        .body(body)
        .send()
        .await?;

    if response.status_code().is_success() {
        let response_body = response.json::<api::bulk::response::ResponseBody>().await?;
        println!("{:#?}", response_body);
    } else {
        let response_body = response.text().await?;
        println!("{}", response_body);
    }
    Ok(())
}
