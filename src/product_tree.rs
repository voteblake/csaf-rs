use serde::{Deserialize, Serialize};

use crate::definitions::{BranchesT, FullProductName, ProductGroupIdT, ProductIdT};

// https://github.com/oasis-tcs/csaf/blob/master/csaf_2.0/prose/csaf-v2-editor-draft.md#322-product-tree-property
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductTree {
    pub branches: Option<BranchesT>,
    pub full_product_names: Option<Vec<FullProductName>>,
    pub product_groups: Option<Vec<ProductGroup>>,
    pub relationships: Option<Vec<Relationship>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductGroup {
    pub group_id: ProductGroupIdT,
    pub product_ids: Vec<ProductIdT>,
    pub summary: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    pub category: RelationshipCategory,
    pub full_product_name: FullProductName,
    pub product_reference: ProductIdT,
    pub relates_to_product_reference: ProductIdT,
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
