use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root metamodel structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    #[serde(rename = "$class")]
    pub class: String,
    pub decorators: Option<Vec<Decorator>>,
    pub namespace: String,
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decorator {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    pub arguments: Option<Vec<DecoratorArgument>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$class")]
pub enum DecoratorArgument {
    #[serde(rename = "concerto.metamodel@1.0.0.DecoratorString")]
    DecoratorString { value: String },
    #[serde(rename = "concerto.metamodel@1.0.0.DecoratorNumber")]
    DecoratorNumber { value: f64 },
    #[serde(rename = "concerto.metamodel@1.0.0.DecoratorBoolean")]
    DecoratorBoolean { value: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    #[serde(rename = "$class")]
    pub class: String,
    pub namespace: String,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$class")]
pub enum Declaration {
    #[serde(rename = "concerto.metamodel@1.0.0.ConceptDeclaration")]
    ConceptDeclaration {
        name: String,
        #[serde(rename = "isAbstract")]
        is_abstract: bool,
        properties: Vec<Property>,
        decorators: Option<Vec<Decorator>>,
        #[serde(rename = "superType")]
        super_type: Option<TypeIdentifier>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.AssetDeclaration")]
    AssetDeclaration {
        name: String,
        #[serde(rename = "isAbstract")]
        is_abstract: bool,
        properties: Vec<Property>,
        decorators: Option<Vec<Decorator>>,
        #[serde(rename = "superType")]
        super_type: Option<TypeIdentifier>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.ParticipantDeclaration")]
    ParticipantDeclaration {
        name: String,
        #[serde(rename = "isAbstract")]
        is_abstract: bool,
        properties: Vec<Property>,
        decorators: Option<Vec<Decorator>>,
        #[serde(rename = "superType")]
        super_type: Option<TypeIdentifier>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.TransactionDeclaration")]
    TransactionDeclaration {
        name: String,
        #[serde(rename = "isAbstract")]
        is_abstract: bool,
        properties: Vec<Property>,
        decorators: Option<Vec<Decorator>>,
        #[serde(rename = "superType")]
        super_type: Option<TypeIdentifier>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.EventDeclaration")]
    EventDeclaration {
        name: String,
        #[serde(rename = "isAbstract")]
        is_abstract: bool,
        properties: Vec<Property>,
        decorators: Option<Vec<Decorator>>,
        #[serde(rename = "superType")]
        super_type: Option<TypeIdentifier>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.EnumDeclaration")]
    EnumDeclaration {
        name: String,
        properties: Vec<EnumProperty>,
        decorators: Option<Vec<Decorator>>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.MapDeclaration")]
    MapDeclaration {
        name: String,
        key: TypeIdentifier,
        value: TypeIdentifier,
        decorators: Option<Vec<Decorator>>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.ScalarDeclaration")]
    ScalarDeclaration {
        name: String,
        #[serde(rename = "type")]
        scalar_type: String,
        decorators: Option<Vec<Decorator>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$class")]
pub enum Property {
    #[serde(rename = "concerto.metamodel@1.0.0.StringProperty")]
    StringProperty {
        name: String,
        #[serde(rename = "isArray")]
        is_array: bool,
        #[serde(rename = "isOptional")]
        is_optional: bool,
        decorators: Option<Vec<Decorator>>,
        validator: Option<StringValidator>,
        #[serde(rename = "defaultValue")]
        default_value: Option<String>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.IntegerProperty")]
    IntegerProperty {
        name: String,
        #[serde(rename = "isArray")]
        is_array: bool,
        #[serde(rename = "isOptional")]
        is_optional: bool,
        decorators: Option<Vec<Decorator>>,
        validator: Option<IntegerValidator>,
        #[serde(rename = "defaultValue")]
        default_value: Option<i64>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.LongProperty")]
    LongProperty {
        name: String,
        #[serde(rename = "isArray")]
        is_array: bool,
        #[serde(rename = "isOptional")]
        is_optional: bool,
        decorators: Option<Vec<Decorator>>,
        validator: Option<IntegerValidator>,
        #[serde(rename = "defaultValue")]
        default_value: Option<i64>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.DoubleProperty")]
    DoubleProperty {
        name: String,
        #[serde(rename = "isArray")]
        is_array: bool,
        #[serde(rename = "isOptional")]
        is_optional: bool,
        decorators: Option<Vec<Decorator>>,
        validator: Option<DoubleValidator>,
        #[serde(rename = "defaultValue")]
        default_value: Option<f64>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.BooleanProperty")]
    BooleanProperty {
        name: String,
        #[serde(rename = "isArray")]
        is_array: bool,
        #[serde(rename = "isOptional")]
        is_optional: bool,
        decorators: Option<Vec<Decorator>>,
        #[serde(rename = "defaultValue")]
        default_value: Option<bool>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.DateTimeProperty")]
    DateTimeProperty {
        name: String,
        #[serde(rename = "isArray")]
        is_array: bool,
        #[serde(rename = "isOptional")]
        is_optional: bool,
        decorators: Option<Vec<Decorator>>,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.ObjectProperty")]
    ObjectProperty {
        name: String,
        #[serde(rename = "isArray")]
        is_array: bool,
        #[serde(rename = "isOptional")]
        is_optional: bool,
        decorators: Option<Vec<Decorator>>,
        #[serde(rename = "type")]
        object_type: TypeIdentifier,
        #[serde(rename = "defaultValue")]
        default_value: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumProperty {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    pub decorators: Option<Vec<Decorator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeIdentifier {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$class")]
pub enum StringValidator {
    #[serde(rename = "concerto.metamodel@1.0.0.StringRegexValidator")]
    StringRegexValidator {
        pattern: String,
        flags: String,
    },
    #[serde(rename = "concerto.metamodel@1.0.0.StringLengthValidator")]
    StringLengthValidator {
        #[serde(rename = "minLength")]
        min_length: Option<i32>,
        #[serde(rename = "maxLength")]
        max_length: Option<i32>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$class")]
pub enum IntegerValidator {
    #[serde(rename = "concerto.metamodel@1.0.0.IntegerDomainValidator")]
    IntegerDomainValidator {
        #[serde(rename = "lower")]
        lower: Option<i64>,
        #[serde(rename = "upper")]
        upper: Option<i64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$class")]
pub enum DoubleValidator {
    #[serde(rename = "concerto.metamodel@1.0.0.DoubleDomainValidator")]
    DoubleDomainValidator {
        #[serde(rename = "lower")]
        lower: Option<f64>,
        #[serde(rename = "upper")]
        upper: Option<f64>,
    },
}

impl Model {
    /// Load the metamodel from the embedded JSON
    pub fn load_concerto_metamodel() -> Result<Model, crate::error::ValidationError> {
        let metamodel_json = include_str!("../metamodel.json");
        let model: Model = serde_json::from_str(metamodel_json)?;
        Ok(model)
    }
    
    /// Create a type registry from the declarations
    pub fn create_type_registry(&self) -> HashMap<String, &Declaration> {
        let mut registry = HashMap::new();
        for declaration in &self.declarations {
            let name = match declaration {
                Declaration::ConceptDeclaration { name, .. } => name,
                Declaration::AssetDeclaration { name, .. } => name,
                Declaration::ParticipantDeclaration { name, .. } => name,
                Declaration::TransactionDeclaration { name, .. } => name,
                Declaration::EventDeclaration { name, .. } => name,
                Declaration::EnumDeclaration { name, .. } => name,
                Declaration::MapDeclaration { name, .. } => name,
                Declaration::ScalarDeclaration { name, .. } => name,
            };
            registry.insert(format!("{}.{}", self.namespace, name), declaration);
        }
        registry
    }
}
