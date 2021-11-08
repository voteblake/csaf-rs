use std::convert::TryInto;

use crate::{
    definitions::{FullProductName, Reference},
    document::{
        CsafVersion, Document, Generator, Publisher, PublisherCategory, Revision, Status, Tracking,
    },
    product_tree::ProductTree,
    vulnerability::{Vulnerability, VulnerabilityId},
    Csaf,
};
use chrono::{TimeZone, Utc};
use rustsec::Advisory;
use url::Url;

const PRODUCT_ID: &str = "PID-1";

// ASSUMPTIONS:
// There is no 'history' in RUSTSEC advisories, so current advisory IS initial advisory
// TODO: Should insert two revisions in the case where withdrawn in set
//
// Each RUSTSEC advisory applies to only one 'product' - in this case crate, thus can be referred to
// by one product_id, which must only be unique with the document.

impl From<Advisory> for Csaf {
    fn from(input: Advisory) -> Self {
        let advisory_date = input.metadata.date;
        let advisory_date = Utc
            .ymd(
                advisory_date.year().try_into().unwrap(),
                advisory_date.month(),
                advisory_date.day(),
            )
            .and_hms(0, 0, 0);

        Csaf {
            document: Document {
                category: "vex".to_string(),
                publisher: Publisher {
                    category: PublisherCategory::Coordinator,
                    name: "RUSTSEC".to_string(),
                    namespace: Url::parse("https://rustsec.org/").unwrap(),
                    contact_details: None,
                    issuing_authority: None,
                },
                title: input.metadata.title,
                tracking: Tracking {
                    current_release_date: advisory_date,
                    id: input.metadata.id.to_string(),
                    initial_release_date: advisory_date,
                    revision_history: vec![Revision {
                        date: advisory_date,
                        number: "1".to_string(),
                        summary: "RUSTSEC advisory".to_string(),
                    }],
                    status: Status::Final,
                    version: "1".to_string(),
                    aliases: if input.metadata.aliases.is_empty() {
                        None
                    } else {
                        Some(
                            input
                                .metadata
                                .aliases
                                .iter()
                                .map(|id| id.to_string())
                                .collect(),
                        )
                    },
                    generator: Some(Generator::default()),
                },
                csaf_version: CsafVersion::TwoDotZero,
                acknowledgments: None,
                aggregate_severity: None,
                distribution: None,
                lang: None, // TODO: Understand if RUSTSEC is canonically english
                notes: None,
                references: if input.metadata.references.is_empty() {
                    None
                } else {
                    Some(
                        input
                            .metadata
                            .references
                            .iter()
                            .map(|url| Reference {
                                url: url.clone(),
                                summary: url.to_string(),
                                category: None,
                            })
                            .collect(),
                    )
                },
                source_lang: None,
            },
            product_tree: Some(ProductTree {
                branches: None,
                full_product_names: Some(vec![FullProductName {
                    name: input.metadata.package.to_string(),
                    product_id: PRODUCT_ID.to_string(),
                    product_identification_helper: None,
                }]),
                product_groups: None,
                relationships: None,
            }),
            vulnerabilities: Some(vec![Vulnerability {
                acknowledgments: None,
                cve: if input.metadata.id.is_cve() {
                    Some(input.metadata.id.to_string())
                } else {
                    None
                },
                cwe: None,
                discovery_date: None,
                id: Some(VulnerabilityId {
                    text: input.metadata.id.to_string(),
                    system_name: match input.metadata.id.kind() {
                        rustsec::advisory::id::Kind::RUSTSEC => "RUSTSEC",
                        rustsec::advisory::id::Kind::CVE => "CVE",
                        rustsec::advisory::id::Kind::GHSA => "GHSA",
                        rustsec::advisory::id::Kind::TALOS => "Talos",
                        _ => "Other",
                    }
                    .to_string(),
                }),
                involvements: todo!(),
                notes: todo!(),
                product_status: todo!(),
                references: todo!(),
                release_date: todo!(),
                remediations: todo!(),
                scores: todo!(),
                threats: todo!(),
                title: todo!(),
            }]),
        }
    }
}
