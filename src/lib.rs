use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md
// # 4 Profiles

// CSAF documents do not have many required fields as they can be used for different purposes. To ensure a common understanding which fields are required in a use case the standard defines profiles. Each subsection describes such a profile by describing necessary content for that specific use case and providing insights into its purpose. The value of `/document/category` is used to identify a CSAF document's profile. Each profile extends the generic profile **Generic CSAF** making additional fields from the standard mandatory. Any other optional field from the standard can also be added to a CSAF document which conforms with a profile without breaking conformance with the profile. One and only exempt is when the profile requires not to have a certain set of fields.

// ## 4.1 Profile 1: Generic CSAF
// This profile defines the default required fields for any CSAF document. Therefore, it is a "catch all" for CSAF documents that do not satisfy any other profile. Furthermore, it is the foundation all other profiles are build on.

// A CSAF document SHALL fulfill the following requirements to satisfy the profile "Generic CSAF":

// * The following elements must exist and be valid:
//   * `/document/category`
//   * `/document/publisher/category`
//   * `/document/publisher/name`
//   * `/document/publisher/namespace`
//   * `/document/title`
//   * `/document/tracking/current_release_date`
//   * `/document/tracking/id`
//   * `/document/tracking/initial_release_date`
//   * `/document/tracking/revision_history[]/date`
//   * `/document/tracking/revision_history[]/number`
//   * `/document/tracking/revision_history[]/summary`
//   * `/document/tracking/status`
//   * `/document/tracking/version`
// * The value of `/document/category` SHALL NOT be equal to any value that is intended to only be used by another profile nor the (case insensitive) name of any other profile. This does not differentiate between underscore, dash or whitespace. To explicitly select the use of this profile the value `generic_csaf` SHOULD be used.

// > Neither `Security Advisory` nor `security advisory` are valid values for `/document/category`.

// An issuing party might choose to set `/document/publisher/name` in front of a value that is intended to only be used by another profile to state that the CSAF document does not use the profile associated with this value. This should be done if the issuing party is able or unwilling to use the value `generic_csaf`, e.g. due to legal or cooperate identity reasons.

// > Both values `Example Company Security Advisory` and `Example Company security_advisory` in `/document/category` use the profile "Generic CSAF". This is important to prepare forward compatibility as later versions of CSAF might add new profiles. Therefore, the values which can be used for the profile "Generic CSAF" might change.

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#32-properties
#[derive(Serialize, Deserialize, Debug)]
pub struct Csaf {
    pub document: Document,
    pub product_tree: Option<ProductTree>,
    pub vulnerabilities: Option<Vulnerabilities>,
}

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

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#3111-version-type
// TODO: Contraint/validation
type VersionT = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Final,
    Interim,
}

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#322-product-tree-property
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductTree {
    branches: Option<BranchesT>,
    full_product_names: Option<Vec<FullProductName>>,
    product_groups: Option<Vec<ProductGroup>>,
    relationships: Option<Vec<Relationship>>,
}

type BranchesT = Vec<Branch>;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductGroup {
    group_id: ProductGroupIdT,
    product_ids: Vec<ProductIdT>,
    summary: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    category: RelationshipCategory,
    full_product_name: FullProductName,
    product_reference: ProductIdT,
    relates_to_product_reference: ProductIdT,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipCategory {
    DefaultComponentOf,
    ExternalComponentOf,
    InstalledOn,
    InstalledWith,
    OptionalComponentOf,
}

type ProductGroupIdT = String;
type ProductIdT = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Vulnerabilities {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn generic_template_deserializes() {
        let generic = r#"{
            "document": {
              "category": "generic_csaf",
              "csaf_version": "2.0",
              "publisher": {
                "category": "other",
                "name": "OASIS CSAF TC",
                "namespace": "https://csaf.io"
              },
              "title": "Template for generating CSAF files for Validator examples",
              "tracking": {
                "current_release_date": "2021-07-21T10:00:00.000Z",
                "id": "OASIS_CSAF_TC-CSAF_2.0-2021-TEMPLATE",
                "initial_release_date": "2021-07-21T10:00:00.000Z",
                "revision_history": [
                  {
                    "date": "2021-07-21T10:00:00.000Z",
                    "number": "1",
                    "summary": "Initial version."
                  }
                ],
                "status": "final",
                "version": "1"
              }
            }
          }"#;

        let document: Csaf = serde_json::from_str(generic).unwrap();
        println!("{:#?}", document);
    }
}
