//! AST structures for a few Concerto types.
//! These are needed to be serialized from Concerto System AST
//! because Rust implementation of Concerto doesn't have
//! what would correspond to introspection classes in JS
//! implementation.

use serde::{Deserialize, Serialize};

/// A serialization of Concerto `Property` definition from AST.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Property {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    #[serde(rename = "isArray")]
    pub is_array: bool,
    #[serde(rename = "isOptional")]
    pub is_optional: bool,
    #[serde(rename = "type")]
    pub super_type: Option<SuperType>,
    pub validator: Option<Validator>,
}

/// A serialization of Concerto `ConceptDeclaration` definition from AST.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ConceptDeclaration {
    #[serde(rename = "$class")]
    pub class: String,
    #[serde(rename = "isAbstract")]
    is_abstract: bool,
    pub properties: Vec<Property>,
    pub name: String,
    #[serde(rename = "superType")]
    pub super_type: Option<SuperType>,
}

/// A serialization of parent type references from AST.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SuperType {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    pub namespace: Option<String>,
}

/// A serialization of validator definition from AST.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Validator {
    #[serde(rename = "$class")]
    pub class: String,
    pub pattern: String,
    pub flags: String,
}