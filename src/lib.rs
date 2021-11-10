//! Common Security Advisory Framework (CSAF)
//!
//! A lovingly hand-crafted implementation of [CSAF](https://www.oasis-open.org/committees/tc_home.php?wg_abbrev=csaf) for Rust. Currently,
//! based on the [v2.0 editor draft](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md). Should be
//! considered strictly less-strict than the spec right now - valid CSAF should deserialize successfully, but invalid CSAF may also
//! succeed and the library may generate invalid CSAF.

use serde::{Deserialize, Serialize};

pub mod document;
use document::Document;

pub mod product_tree;
use product_tree::ProductTree;

pub mod vulnerability;
use vulnerability::Vulnerability;

pub mod definitions;

pub mod interop;

/// [Top level CSAF structure definition](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#32-properties)
#[serde_with::skip_serializing_none]
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

    #[test]
    fn first_example_deserializes() {
        let example = include_str!("../tests/CVE-2018-0171-modified.json");
        let document: Csaf = serde_json::from_str(example).unwrap();
        println!("{:#?}", document);
    }
    #[test]
    fn second_example_deserializes() {
        let example = include_str!("../tests/cvrf-rhba-2018-0489-modified.json");
        let document: Csaf = serde_json::from_str(example).unwrap();
        println!("{:#?}", document);
    }
}
