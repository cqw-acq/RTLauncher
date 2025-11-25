use std::path::Path;

mod version_fetcher;
mod original_dwl;
mod decompression;

/// original_dwl download task
pub async fn original_download_task(
    version: &str,
    mc_home: &Path,
) -> anyhow::Result<String> {
    original_dwl::download(version, mc_home).await?;
    Ok(format!("download {version} done"))
}

/// version_fetcher task
pub async fn classify_versions_task() -> anyhow::Result<String> {
    let [releases, snapshots, fools, olds] =
        version_fetcher::classify_minecraft_versions().await?;
    Ok(format!(
        "Releases {:?}\nSnapshots {:?}\nApril Fools {:?}\nOld versions {:?}",
        releases, snapshots, fools, olds
    ))
}
