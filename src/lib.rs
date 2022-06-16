pub mod maven;
pub mod util;

pub use maven::*;

pub async fn get_forge_versions() -> MavenVersionResult {
    let resp = util::get(
        "https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml".to_string(),
    );
    let raw = xmltojson::to_json(&resp).expect("Something went wrong.");
    let serialized = serde_json::to_string(&raw).unwrap();
    let des: RawMavenVersionResult = serde_json::from_str(&serialized).unwrap();
    return des.metadata;
}
