use serde::{Deserialize, Serialize};
use url::Url;

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#311-acknowledgments-type
pub(crate) type AcknowledgmentsT = Vec<Acknowledgment>;

// TODO: with at least 1 and at most 4 properties
#[derive(Serialize, Deserialize, Debug)]
pub struct Acknowledgment {
    names: Option<Vec<String>>,
    organization: Option<String>,
    summary: Option<String>,
    urls: Option<Vec<Url>>,
}

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#312-branches-type
pub(crate) type BranchesT = Vec<Branch>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Branch {
    name: String,
    category: BranchCategory,
    // TODO - Must have only one of product or branches
    product: Option<FullProductName>,
    branches: BranchesT,
}

#[derive(Serialize, Deserialize, Debug)]
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

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#313-full-product-name-type
#[derive(Serialize, Deserialize, Debug)]
pub struct FullProductName {
    name: String,
    product_id: ProductIdT,
    product_identification_helper: Option<ProductIdentificationHelper>,
}

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3133-full-product-name-type---product-identification-helper
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductIdentificationHelper {
    cpe: Option<String>, // TODO: Integrate actual CPE aware data type
    hashes: Option<Vec<HashCollection>>,
    purl: Option<String>, // TODO: Validation https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#31333-full-product-name-type---product-identification-helper---purl
    sbom_urls: Option<Vec<Url>>,
    serial_numbers: Option<Vec<String>>,
    skus: Option<Vec<String>>,
    x_generic_uris: Option<Vec<Url>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HashCollection {
    file_hashes: Vec<HashValue>,
    file_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HashValue {
    algorithm: String,
    value: String,
}

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#314-language-type
pub(crate) type LangT = String; // TODO: Constrain/validate

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#315-notes-type
pub(crate) type NotesT = Vec<Note>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    category: NoteCategory,
    text: String,
    audience: Option<String>,
    title: Option<String>,
}

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

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#316-product-group-id-type
pub(crate) type ProductGroupIdT = String;

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#317-product-groups-type
pub(crate) type ProductGroupsT = Vec<ProductGroupIdT>;

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#318-product-id-type
pub(crate) type ProductIdT = String;

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#319-products-type
pub(crate) type ProductsT = Vec<ProductIdT>;

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3110-references-type
pub(crate) type ReferencesT = Vec<Reference>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    url: Url,
    summary: String,
    category: Option<ReferenceCategory>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceCategory {
    External,
    #[serde(rename = "self")]
    RefSelf,
}

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3111-version-type
// TODO: Contraint/validation
pub(crate) type VersionT = String;
