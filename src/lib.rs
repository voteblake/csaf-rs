use serde::{Deserialize, Serialize};

pub mod document;
use document::Document;

pub mod product_tree;
use product_tree::ProductTree;

pub mod vulnerability;
use vulnerability::Vulnerability;

pub mod definitions;

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
    pub vulnerabilities: Option<Vec<Vulnerability>>,
}

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
