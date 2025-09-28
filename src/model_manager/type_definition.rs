//! `TypeDefinition` is a wrapper around `ConceptDeclaration`.
//! This part of the crate fills in for the missing `introspect`
//! classes that are part of the JS implementation of Concerto.
//! Methods are specific to this crate for now, i.e. not corresponding
//! to the JS implementation 1-1.

use std::collections::HashMap;
use crate::model_manager::ast_structures::{ConceptDeclaration, Property, SuperType};

pub(crate) struct TypeDefinition {
    pub inner: ConceptDeclaration,
}

impl TypeDefinition {
    pub fn new(concept_declaration: ConceptDeclaration) -> Self {
        TypeDefinition {
            inner: concept_declaration,
        }
    }

    /// All the properties that are part of the `ConceptDeclaration`.
    pub fn expected_properties(&self) -> HashMap<String, &Property> {
        self.inner.properties.iter().map(|x| (x.name.clone(), x)).collect()
    }

    /// All the non-optional properties that are part of the `ConceptDeclaration`.
    pub fn required_properties(&self) -> HashMap<String, &Property> {
        self.inner.properties.iter().filter(|x| !x.is_optional).map(|x| (x.name.clone(), x)).collect()
    }

    /// Returns `true` if `ConceptDeclaration` has a parent type.
    pub fn has_supertype(&self) -> bool {
        self.inner.super_type.is_some()
    }

    /// Returns the AST of the parent type, if there is a parent type.
    pub fn get_supertype(&self) -> Option<&SuperType> {
        match &self.inner.super_type {
            Some(inner) => Some(inner),
            None => None,
        }
    }

    /// Returns all the patterns in `StringProperty` objects.
    /// This is used for pre-compiling [`Regex`](regex::Regex) objects.
    pub(crate) fn get_string_validator_patterns(&self) -> Vec<String> {
        self.expected_properties().values().filter(|x| {
            x.validator.is_some()
        })
            .map(|x| {
                x.validator.clone().unwrap().pattern
            })
            .collect::<Vec<_>>()
    }
}


