use borsh::{schema::BorshSchemaContainer, BorshDeserialize, BorshSchema, BorshSerialize};
use serde::{Deserialize, Serialize};
/// Version of the metadata format.
const METADATA_SEMVER: [u32; 3] = [0, 1, 0];

/// Metadata of the contract.
#[derive(
    BorshSerialize, BorshDeserialize, BorshSchema, Serialize, Deserialize, Debug, PartialEq,
)]
pub struct Metadata {
    /// Semver of the metadata.
    pub version: [u32; 3],
    /// Metadata of all methods.
    pub methods: Vec<MethodMetadata>,
}

impl Metadata {
    pub fn new(methods: Vec<MethodMetadata>) -> Self {
        Self { version: METADATA_SEMVER, methods }
    }
}

/// Metadata of a single method.
#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, BorshSchema, Debug, PartialEq,
)]
pub struct MethodMetadata {
    pub name: String,
    /// Whether method does not modify the state.
    pub is_view: bool,
    /// Whether method can be used to initialize the state.
    pub is_init: bool,
    /// Schema of the arguments of the method.
    #[serde(skip)]
    pub args: Option<BorshSchemaContainer>,
    /// Schemas for each callback of the method.
    #[serde(skip)]
    pub callbacks: Vec<BorshSchemaContainer>,
    /// If all callbacks have the same type then this field can be used instead.
    #[serde(skip)]
    pub callbacks_vec: Option<BorshSchemaContainer>,
    /// Schema of the return type.
    #[serde(skip)]
    pub result: Option<BorshSchemaContainer>,
}
