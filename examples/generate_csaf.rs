use std::str::FromStr;

use chrono::Utc;
use csaf::{
    definitions::{Branch, BranchCategory, BranchesT, FullProductName, ProductIdT},
    document::{
        CsafVersion, Distribution, Document, Generator, Publisher, PublisherCategory, Revision,
        Status, Tlp, TlpLabel, Tracking,
    },
    product_tree::ProductTree,
    vulnerability::{ProductStatus, Threat, ThreatCategory, Vulnerability},
    Csaf,
};
use url::Url;

// Tracking list of improvements as I try to use this to generate a single advisory
//
// All the to_string()s are annoying, would be nice to have impl Into<String> or something available
//    Would work with stuff that needs FromStr<> impls anyway From<String> would just call parse()
//
// Probably want Default impls to fill non-required Options to None if possible
//
// Do we really need DateTime's or are Dates Ok? DateTimes are passing validation with the upstream tool
// but it feels semantically incorrect. current_release_date: Utc::today() feels more correct than
// current_release_date: Utc::now()
//
// Generator shouldn't really be up to the user to provide, so tracking defo needs to be covered by a constructor
//
// Default Revision?
//
// Should be able to provide just a Tlp Label and get a full Tlp with the default first Tlp url
//
// I bet there's a library for the lang tags https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
//
// Product Tree is not clear
//   Provide with_branches(impl Into<BranchesT>) && (impl From<T> for BranchesT where T is collection of Branch) constructor? (one with_x constructor for each member enforces that at least one is set)
//
//   Helper function that takes a package and an iterable of versions and creates branches? The one from interop could be made generic
//   and public.
//
//   Three constructors for Branch - new() name and category , with_branches, and with_product
//   Or do the unit struct thing with type param
//
// Provide implementation of vulnerability from rustsec advisory that just doesn't set product status?
//
// ProductStatus with_x

fn main() {
    let now = Utc::now();

    let rustsec_db =
        rustsec::database::Database::fetch().expect("Need access to RustSec git repository");

    let rustsec_2020_0159: Csaf = rustsec_db
        .get(&rustsec::advisory::id::Id::from_str("RUSTSEC-2020-0159").unwrap())
        .unwrap()
        .to_owned()
        .into();
    let rustsec_2020_0159 = rustsec_2020_0159.vulnerabilities.unwrap();
    let rustsec_2020_0071: Csaf = rustsec_db
        .get(&rustsec::advisory::id::Id::from_str("RUSTSEC-2020-0071").unwrap())
        .unwrap()
        .to_owned()
        .into();
    let rustsec_2020_0071 = rustsec_2020_0071.vulnerabilities.unwrap();

    let mut vulns: Vec<Vulnerability> = [&rustsec_2020_0159[..], &rustsec_2020_0071[..]].concat();

    for vuln in &mut vulns {
        // Set our product as the product in the product status instead of the upstream product from the advisory
        // Clear all upstream specific product identifiers and metadat
        vuln.remediations.take();
        vuln.product_status.take();
        vuln.scores.take();
        vuln.product_status = Some(ProductStatus {
            first_affected: None,
            first_fixed: None,
            fixed: None,
            known_affected: None,
            known_not_affected: Some(vec![ProductIdT("CSAF-1".to_string())]),
            last_affected: None,
            recommended: None,
            under_investigation: None,
        });

        // Generate the VEX required threat statemtent for a known_not_affected package
        vuln.threats = Some(vec![Threat {
            category: ThreatCategory::Impact,
            details: "The vulnerability impacts calls to the `localtime_r` function. `csaf` does not use that function directly or call any function that uses that function transitively.".to_string(),
            date: Some(now),
            group_ids: None,
            product_ids: Some(vec![ProductIdT("CSAF-1".to_string())]),
        }])
    }

    let c = Csaf {
        document: Document {
            category: "vex".to_string(),
            publisher: Publisher {
                category: PublisherCategory::Vendor,
                name: "Blake Johnson".to_string(),
                namespace: Url::parse("https://github.com/voteblake/").unwrap(),
                contact_details: Some("https://twitter.com/voteblake".to_string()),
                issuing_authority: None,
            },
            title: "Csaf Crate Unaffected by time and chrono Vulnerabilities".to_string(),
            tracking: Tracking {
                current_release_date: now,
                id: "CSAF-001".to_string(),
                initial_release_date: now,
                revision_history: vec![Revision {
                    date: now,
                    // Existence is pain
                    number: "1".to_string(),
                    summary: "Initial release".to_string(),
                }],
                status: Status::Draft,
                version: "1".to_string(),
                aliases: None,
                generator: Some(Generator::default()),
            },
            csaf_version: CsafVersion::TwoDotZero,
            acknowledgments: None,
            aggregate_severity: None,
            distribution: Some(Distribution {
                text: None,
                tlp: Some(Tlp {
                    label: TlpLabel::WHITE,
                    url: Url::parse("https://www.first.org/tlp/").ok(),
                }),
            }),
            lang: Some("en".to_string()),
            notes: None,
            references: None,
            source_lang: None,
        },
        product_tree: Some(ProductTree {
            branches: Some(BranchesT(vec![Branch {
                name: "csaf".to_string(),
                category: BranchCategory::ProductName,
                product: None,
                branches: Some(BranchesT(vec![Branch {
                    name: "0.3.0".to_string(),
                    category: BranchCategory::ProductVersion,
                    product: Some(FullProductName {
                        name: "csaf 0.3.0".to_string(),
                        product_id: ProductIdT("CSAF-1".to_string()),
                        product_identification_helper: None,
                    }),
                    branches: None,
                }])),
            }])),
            full_product_names: None,
            product_groups: None,
            relationships: None,
        }),
        vulnerabilities: Some(vulns),
    };

    println!("{}", serde_json::to_string_pretty(&c).unwrap());
}
