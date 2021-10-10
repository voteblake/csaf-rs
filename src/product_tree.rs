use serde::{Deserialize, Serialize};

use crate::definitions::{BranchesT, FullProductName, ProductGroupIdT, ProductIdT};

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#322-product-tree-property
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductTree {
    branches: Option<BranchesT>,
    full_product_names: Option<Vec<FullProductName>>,
    product_groups: Option<Vec<ProductGroup>>,
    relationships: Option<Vec<Relationship>>,
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
