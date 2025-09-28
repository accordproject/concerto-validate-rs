use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub(crate) struct TypeDefinition {
    pub inner: ConceptDeclaration,
}

impl TypeDefinition {
    pub fn new(concept_declaration: ConceptDeclaration) -> Self {
        TypeDefinition {
            inner: concept_declaration,
        }
    }

    // pub fn properties(&self) -> Vec<&Property> {
    //     self.inner.properties.iter().map(|x| x).collect()
    // }

    pub fn expected_properties(&self) -> HashMap<String, &Property> {
        self.inner.properties.iter().map(|x| (x.name.clone(), x)).collect()
    }

    pub fn required_properties(&self) -> HashMap<String, &Property> {
        self.inner.properties.iter().filter(|x| !x.is_optional).map(|x| (x.name.clone(), x)).collect()
    }
}

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
    pub super_type: SuperType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ConceptDeclaration {
    #[serde(rename = "$class")]
    pub class: String,
    #[serde(rename = "isAbstract")]
    is_abstract: bool,
    pub properties: Vec<Property>,
    pub name: String,
    #[serde(rename = "superType")]
    pub super_type: SuperType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SuperType {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
}
