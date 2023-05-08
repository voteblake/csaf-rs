## Common Security Advisory Framework (CSAF)

![Crates.io](https://img.shields.io/crates/v/csaf)
![Crates.io](https://img.shields.io/crates/l/csaf)
![docs.rs](https://img.shields.io/docsrs/csaf)
![GitHub branch checks state](https://img.shields.io/github/checks-status/voteblake/csaf-rs/main)

A lovingly hand-crafted<sup>[1](#footnote1)</sup> implementation of [CSAF](https://www.oasis-open.org/committees/tc_home.php?wg_abbrev=csaf) for Rust. Currently, based on the [v2.0 editor draft](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md). Should be considered strictly less-strict than the spec right now - valid CSAF should deserialize successfully, but invalid CSAF may also succeed and the library may generate invalid CSAF.

My current use case is for experimenting with the [VEX profile](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#45-profile-5-vex).

<a name="footnote1">1</a> - CSAF defines a [JSON Schema](https://json-schema.org/understanding-json-schema/index.html) [schema](https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/json_schema/csaf_json_schema.json) using Draft 2020-12. [`schemafy`](https://crates.io/crates/schemafy) exists for generating Rust code from JSON Schema, but supports an older draft which differs significantly from the 2020-12 draft.
