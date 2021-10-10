use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::definitions::{AcknowledgmentsT, LangT, NotesT, ReferencesT, VersionT};

/// [Document level meta-data](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#321-document-property)
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub category: String,
    pub publisher: Publisher,
    pub title: String,
    pub tracking: Tracking,
    pub csaf_version: CsafVersion,
    pub acknowledgments: Option<AcknowledgmentsT>,
    pub aggregate_severity: Option<AggregateSeverity>,
    pub distribution: Option<Distribution>,
    pub lang: Option<LangT>,
    pub notes: Option<NotesT>,
    pub references: Option<ReferencesT>,
    pub source_lang: Option<LangT>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CsafVersion {
    #[serde(rename = "2.0")]
    TwoDotZero,
}

/// [Publisher property](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3218-document-property---publisher)
#[derive(Serialize, Deserialize, Debug)]
pub struct Publisher {
    pub category: PublisherCategory,
    pub name: String,
    pub namespace: Url,
    pub contact_details: Option<String>,
    pub issuing_authority: Option<String>,
}

/// [Publisher category](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#32181-document-property---publisher---category)
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

/// [Tracking metadata](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#32112-document-property---tracking)
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

/// [Document Generator](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#321123-document-property---tracking---generator)
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

/// [Revision history](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#321126-document-property---tracking---revision-history)
#[derive(Serialize, Deserialize, Debug)]
pub struct Revision {
    pub date: DateTime<Utc>,
    pub number: VersionT,
    pub summary: String,
}

/// [Document status](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#321127-document-property---tracking---status)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Final,
    Interim,
}

/// [Aggregate severity](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3212-document-property---aggregate-severity)
#[derive(Serialize, Deserialize, Debug)]
pub struct AggregateSeverity {
    pub text: String,
    pub namespace: Option<Url>,
}

/// [Distribution](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3215-document-property---distribution)
#[derive(Serialize, Deserialize, Debug)]
pub struct Distribution {
    // TODO: enforce 'with at least 1 of the 2 properties'
    pub text: Option<String>,
    pub tlp: Option<Tlp>,
}

/// [TLP](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#32152-document-property---distribution---tlp)
#[derive(Serialize, Deserialize, Debug)]
pub struct Tlp {
    pub label: TlpLabel,
    pub url: Option<Url>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TlpLabel {
    AMBER,
    GREEN,
    RED,
    WHITE,
}
