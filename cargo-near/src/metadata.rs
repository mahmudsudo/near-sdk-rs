use crate::crate_metadata::CrateMetadata;
use crate::util;
use crate::workspace::{ManifestPath, Workspace};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{fs, path::PathBuf};

const METADATA_FILE: &str = "metadata.json";

/// Metadata generation result.
#[derive(serde::Serialize)]
pub struct MetadataResult {
    /// Path to the resulting metadata file.
    pub dest_metadata: PathBuf,
}

/// Smart contract meta information.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContractMetaInfo {
    /// The name of the smart contract.
    pub name: String,
    /// The version of the smart contract.
    pub version: String,
    /// The authors of the smart contract.
    pub authors: Vec<String>,
}

/// Smart contract metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContractMetadata {
    metainfo: ContractMetaInfo,
    #[serde(flatten)]
    pub aci: Map<String, Value>,
}

impl ContractMetadata {
    pub fn new(metainfo: ContractMetaInfo, aci: Map<String, Value>) -> Self {
        Self { metainfo, aci }
    }
}

fn extract_metainfo(crate_metadata: &CrateMetadata) -> ContractMetaInfo {
    let package = &crate_metadata.root_package;
    ContractMetaInfo {
        name: package.name.clone(),
        version: package.version.to_string(),
        authors: package.authors.clone(),
    }
}

pub(crate) fn execute(crate_metadata: &CrateMetadata) -> Result<MetadataResult> {
    let target_directory = crate_metadata.target_directory.clone();
    let out_path_metadata = target_directory.join(METADATA_FILE);

    let generate_metadata = |manifest_path: &ManifestPath| -> Result<()> {
        let target_dir_arg = format!("--target-dir={}", target_directory.to_string_lossy());
        let stdout = util::invoke_cargo(
            "run",
            &[
                "--package",
                "metadata-gen",
                &manifest_path.cargo_arg()?,
                &target_dir_arg,
                "--release",
            ],
            manifest_path.directory(),
            vec![],
        )?;

        let near_meta: serde_json::Map<String, serde_json::Value> =
            serde_json::from_slice(&stdout)?;
        let metainfo = extract_metainfo(&crate_metadata);
        let metadata = ContractMetadata::new(metainfo, near_meta);
        let contents = serde_json::to_string_pretty(&metadata)?;
        fs::write(&out_path_metadata, contents)?;

        Ok(())
    };

    Workspace::new(&crate_metadata.cargo_meta, &crate_metadata.root_package.id)?
        .with_root_package_manifest(|manifest| {
            manifest.with_added_crate_type("rlib")?.with_profile_release_lto(false)?;
            Ok(())
        })?
        .with_metadata_gen_package(crate_metadata.manifest_path.absolute_directory()?)?
        .using_temp(generate_metadata)?;

    Ok(MetadataResult { dest_metadata: out_path_metadata })
}
