use reqwest::blocking;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

/// URL to the official Concerto metamodel JSON
const METAMODEL_URL: &str = "https://raw.githubusercontent.com/accordproject/concerto-metamodel/main/lib/metamodel.json";

/// Local path to the metamodel file
const LOCAL_METAMODEL_PATH: &str = "metamodel.json";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=metamodel.json");
    
    match update_metamodel_if_needed() {
        Ok(updated) => {
            if updated {
                println!("cargo:warning=Updated metamodel.json from official repository");
            } else {
                println!("cargo:warning=metamodel.json is up to date");
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to check/update metamodel: {}. Using existing local version.", e);
            // Don't fail the build if we can't update - just use the existing file
            if !Path::new(LOCAL_METAMODEL_PATH).exists() {
                panic!("No local metamodel.json found and unable to download from remote");
            }
        }
    }
}

/// Updates the local metamodel if it differs from the remote version
/// Returns Ok(true) if updated, Ok(false) if no update needed, Err(_) if failed
fn update_metamodel_if_needed() -> Result<bool, Box<dyn std::error::Error>> {
    // Download the remote metamodel
    println!("Checking for metamodel updates from {}", METAMODEL_URL);
    
    let client = blocking::Client::builder()
        .user_agent("concerto-validator-rs-build-script")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    let response = client.get(METAMODEL_URL).send()?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }
    
    let remote_content = response.text()?;
    
    // Validate that the remote content is valid JSON
    let _: serde_json::Value = serde_json::from_str(&remote_content)
        .map_err(|e| format!("Remote metamodel is not valid JSON: {}", e))?;
    
    // Calculate hash of remote content
    let remote_hash = calculate_hash(&remote_content);
    
    // Check if local file exists and compare
    if Path::new(LOCAL_METAMODEL_PATH).exists() {
        let local_content = fs::read_to_string(LOCAL_METAMODEL_PATH)?;
        let local_hash = calculate_hash(&local_content);
        
        if remote_hash == local_hash {
            println!("Local metamodel.json is already up to date");
            return Ok(false);
        }
        
        println!("Local metamodel.json differs from remote, updating...");
    } else {
        println!("Local metamodel.json not found, downloading...");
    }
    
    // Write the new content
    fs::write(LOCAL_METAMODEL_PATH, &remote_content)?;
    
    // Verify the written file
    let written_content = fs::read_to_string(LOCAL_METAMODEL_PATH)?;
    let written_hash = calculate_hash(&written_content);
    
    if written_hash != remote_hash {
        return Err("Verification failed: written file hash doesn't match remote".into());
    }
    
    println!("Successfully updated metamodel.json");
    Ok(true)
}

/// Calculate SHA256 hash of the given content (normalized)
fn calculate_hash(content: &str) -> String {
    // Normalize JSON to ensure consistent comparison
    let normalized = match serde_json::from_str::<serde_json::Value>(content) {
        Ok(json) => serde_json::to_string(&json).unwrap_or_else(|_| content.to_string()),
        Err(_) => content.to_string(),
    };
    
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    format!("{:x}", hasher.finalize())
}
