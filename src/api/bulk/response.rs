use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    took: i64,
    errors: bool,
    items: Vec<Action>,
}

#[derive(Deserialize, Debug)]
pub struct Shard {
    failed: i64,
    successful: i64,
    total: i64,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    r#type: String,
    reason: String,
    index_uuid: String,
    shard: String,
    index: String,
}

#[derive(Deserialize, Debug)]
pub struct ActionParams {
    #[serde(rename(deserialize = "_index"))]
    pub index: String,
    #[serde(rename(deserialize = "_type"))]
    pub r#type: String,
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    #[serde(alias = "_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "_shards")]
    pub shards: Option<Shard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "_seq_no")]
    pub seq_no: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "_primary_term")]
    pub primary_term: Option<i64>,
    pub status: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

#[derive(Deserialize, Debug)]
pub enum Action {
    #[serde(alias = "index")]
    Index {
        #[serde(flatten)]
        params: ActionParams,
    },
    #[serde(alias = "delete")]
    Delete {
        #[serde(flatten)]
        params: ActionParams,
    },
    #[serde(alias = "create")]
    Create {
        #[serde(flatten)]
        params: ActionParams,
    },
    #[serde(alias = "update")]
    Update {
        #[serde(flatten)]
        params: ActionParams,
    },
}
