use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::definitions::VersionT;

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#321-document-property
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub category: String,
    pub publisher: Publisher,
    pub title: String,
    pub tracking: Tracking,
    pub csaf_version: CsafVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CsafVersion {
    #[serde(rename = "2.0")]
    TwoDotZero,
}

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3218-document-property---publisher
#[derive(Serialize, Deserialize, Debug)]
pub struct Publisher {
    pub category: PublisherCategory,
    pub name: String,
    pub namespace: Url,
    pub contact_details: Option<String>,
    pub issuing_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PublisherCategory {
    Coordinator,
    Discoverer,
    Other,
    Translator,
    User,
    Vendor,
}

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#32112-document-property---tracking
#[derive(Serialize, Deserialize, Debug)]
pub struct Tracking {
    pub current_release_date: DateTime<Utc>,
    pub id: String,
    pub initial_release_date: DateTime<Utc>,
    pub revision_history: Vec<Revision>,
    pub status: Status,
    pub version: VersionT,
    pub aliases: Option<Vec<String>>,
    pub generator: Option<Generator>,
}

//https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#321123-document-property---tracking---generator
#[derive(Serialize, Deserialize, Debug)]
pub struct Generator {
    pub engine: Engine,
    pub date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Revision {
    pub date: DateTime<Utc>,
    pub number: VersionT,
    pub summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Final,
    Interim,
}
