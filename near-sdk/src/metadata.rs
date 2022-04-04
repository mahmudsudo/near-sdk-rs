use schemars::schema::SchemaObject;
use serde::{Deserialize, Serialize};
/// Version of the metadata format.
const METADATA_SEMVER: [u32; 3] = [0, 1, 0];

/// Metadata of the contract.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Metadata {
    /// Semver of the metadata.
    pub version: [u32; 3],
    /// Metadata of all methods.
    pub methods: Vec<MethodMetadata>,
    /// Type registry
    pub types: Vec<TypeDef>,
}

impl Metadata {
    pub fn new(methods: Vec<MethodMetadata>, types: Vec<TypeDef>) -> Self {
        schemars::schema_for!(u32);
        Self { version: METADATA_SEMVER, methods, types }
    }
}

/// Metadata of a single method.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MethodMetadata {
    pub name: String,
    /// Whether method does not modify the state.
    pub is_view: bool,
    /// Whether method can be used to initialize the state.
    pub is_init: bool,
    /// Type identifiers of the arguments of the method.
    pub args: Vec<u32>,
    /// Type identifiers of the callbacks of the method.
    pub callbacks: Vec<u32>,
    /// Type identifiers of the vector callbacks of the method.
    pub callbacks_vec: Vec<u32>,
    /// Return type identifier.
    pub result: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TypeDef {
    pub id: u32,
    pub schema: SchemaObject,
}
