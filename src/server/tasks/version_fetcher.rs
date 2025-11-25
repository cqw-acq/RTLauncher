use reqwest::Error as ReqwestError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct VersionManifest {
    versions: Vec<VersionEntry>,
}

#[derive(Debug, Deserialize)]
struct VersionEntry {
    id: String,
    #[serde(rename = "type")]
    version_type: String,
    #[serde(rename = "releaseTime")]
    time: String,
}

pub async fn classify_minecraft_versions() -> Result<[Vec<String>; 4], ReqwestError> {
    // Get version manifest data (automatically handles HTTP errors)
    let response = reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest.json")
        .await?
        .error_for_status()?;

    // Parse JSON data
    let manifest: VersionManifest = response.json().await?;

    // Initialize classification containers
    let mut releases = Vec::new();     // Release versions
    let mut snapshots = Vec::new();    // Snapshot versions
    let mut april_fools = Vec::new();  // April Fools versions
    let mut old_versions = Vec::new(); // Old versions

    // Classification processing
    for entry in manifest.versions {
        // Handle old versions (highest priority)
        if matches!(entry.version_type.as_str(), "old_alpha" | "old_beta") {
            old_versions.push(entry.id);
            continue;
        }
        
        // Handle April Fools versions (second highest priority)
        if entry.time.contains("-04-01") {
            april_fools.push(entry.id);
            continue;
        }

        // Handle release and snapshot versions
        match entry.version_type.as_str() {
            "release" => releases.push(entry.id),
            "snapshot" => snapshots.push(entry.id),
            _ => {} // Ignore other types
        }
    }

    Ok([releases, snapshots, april_fools, old_versions])
}
