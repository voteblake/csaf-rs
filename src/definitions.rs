use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};
use url::Url;

pub(crate) type AcknowledgmentsT = Vec<Acknowledgment>;

// TODO: with at least 1 and at most 4 properties
/// [Acknowledgment](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#311-acknowledgments-type)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Acknowledgment {
    pub names: Option<Vec<String>>,
    pub organization: Option<String>,
    pub summary: Option<String>,
    pub urls: Option<Vec<Url>>,
}

/// [Branches](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#312-branches-type)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BranchesT(pub Vec<Branch>);

impl BranchesT {
    pub(crate) fn product_ids(&self) -> Option<Vec<ProductIdT>> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.iter().map(|x| x.try_into().unwrap()).collect())
        }
    }
}

impl TryFrom<&Branch> for ProductIdT {
    type Error = &'static str;

    fn try_from(b: &Branch) -> Result<Self, Self::Error> {
        match &b.product {
            Some(p) => Ok(p.product_id.clone()),
            None => Err("Cannot convert Branch that does not contain a product to a ProductIdT"),
        }
    }
}

/// [Branch](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3121-branches-type---branches)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub category: BranchCategory,
    // TODO - Must have only one of product or branches
    pub product: Option<FullProductName>,
    pub branches: Option<BranchesT>,
}

/// [Branch Category](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3122-branches-type---category)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BranchCategory {
    Architecture,
    HostName,
    Language,
    Legacy,
    PatchLevel,
    ProductFamily,
    ProductName,
    ProductVersion,
    ServicePack,
    Specification,
    Vendor,
}

/// [Full Product Name](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#313-full-product-name-type)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullProductName {
    pub name: String,
    pub product_id: ProductIdT,
    pub product_identification_helper: Option<ProductIdentificationHelper>,
}

/// [Product Identification Helper](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3133-full-product-name-type---product-identification-helper)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdentificationHelper {
    pub cpe: Option<String>, // TODO: Integrate actual CPE aware data type
    pub hashes: Option<Vec<HashCollection>>,
    pub purl: Option<String>, // TODO: Validation https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#31333-full-product-name-type---product-identification-helper---purl
    pub sbom_urls: Option<Vec<Url>>,
    pub serial_numbers: Option<Vec<String>>,
    pub skus: Option<Vec<String>>,
    pub x_generic_uris: Option<Vec<Url>>,
}

/// [Hashes](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#31332-full-product-name-type---product-identification-helper---hashes)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashCollection {
    pub file_hashes: Vec<HashValue>,
    pub file_name: String,
}

/// [Hashes](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#31332-full-product-name-type---product-identification-helper---hashes)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashValue {
    // TODO: Validation - These values are derived from the currently supported digests OpenSSL [OPENSSL]. Leading dashs were removed.
    pub algorithm: String,
    // TODO: Validation  ^[0-9a-fA-F]{32,}$
    pub value: String,
}

/// [LangT](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#314-language-type)
pub(crate) type LangT = String; // TODO: Constrain/validate

pub(crate) type NotesT = Vec<Note>;

/// [Notes](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#315-notes-type)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub category: NoteCategory,
    pub text: String,
    pub audience: Option<String>,
    pub title: Option<String>,
}

/// [Notes](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#315-notes-type)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NoteCategory {
    Description,
    Details,
    Faq,
    General,
    LegalDisclaimer,
    Other,
    Summary,
}

/// [Product Group ID](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#316-product-group-id-type)
pub(crate) type ProductGroupIdT = String;

/// [Product Groups](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#317-product-groups-type)
pub(crate) type ProductGroupsT = Vec<ProductGroupIdT>;

/// [Product IDs](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#318-product-id-type)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdT(pub(crate) String);

/// [Products](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#319-products-type)
pub(crate) type ProductsT = Vec<ProductIdT>;

pub(crate) type ReferencesT = Vec<Reference>;

/// [References](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3110-references-type)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    pub url: Url,
    pub summary: String,
    pub category: Option<ReferenceCategory>,
}

/// [References](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3110-references-type)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceCategory {
    External,
    #[serde(rename = "self")]
    RefSelf,
}

// TODO: Contraint/validation
/// [Version](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3111-version-type)
pub(crate) type VersionT = String;
