use serde::Deserialize;
// https://www.elastic.co/guide/en/elasticsearch/reference/7.7/docs-index_.html#docs-index-api-response-body
#[derive(Deserialize, Debug)]
struct Shard {
    failed: i64,
    successful: i64,
    total: i64,
}

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    #[serde(alias = "_index")]
    index: String,
    #[serde(alias = "_type")]
    r#type: String,
    #[serde(alias = "_id")]
    id: String,
    #[serde(alias = "_version")]
    version: i64,
    result: String,
    #[serde(alias = "_shards")]
    shards: Shard,
    #[serde(alias = "_seq_no")]
    seq_no: i64,
    #[serde(alias = "_primary_term")]
    primary_term: i64,
}
