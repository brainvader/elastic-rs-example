use serde::Serialize;

#[derive(Serialize)]
pub struct ActionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_index")]
    pub index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<String>,
}

#[derive(Serialize)]
pub enum Action {
    #[serde(rename = "create")]
    Create {
        #[serde(flatten)]
        params: ActionParams,
    },
    #[serde(rename = "index")]
    Index {
        #[serde(flatten)]
        params: ActionParams,
    },
    #[serde(rename = "update")]
    Update {
        #[serde(flatten)]
        params: ActionParams,
    },
    #[serde(rename = "delete")]
    Delete {
        #[serde(flatten)]
        params: ActionParams,
    },
}
