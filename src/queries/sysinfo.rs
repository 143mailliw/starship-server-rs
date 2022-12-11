#![allow(non_snake_case)]
use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
struct SysInfoPaths {
    #[graphql(name = "emojiURL")]
    emoji_url: String,
    #[graphql(name = "pfpURL")]
    pfp_url: String,
    #[graphql(name = "bannerURL")]
    banner_url: String,
    graphql_endpoint: String,
}

#[derive(SimpleObject)]
struct SysInfo {
    server_name: String,
    version: String,
    feature_level: u16,
    supported_features: Vec<String>,
    supported_components: Vec<String>,
    client_flags: Vec<String>,
    paths: SysInfoPaths,
}

impl Default for SysInfo {
    fn default() -> Self {
        // TODO: Fill this data out properly
        Self {
            server_name: "starship-server-rs".to_string(),
            version: "2023.0-dev-milestone1".to_string(),
            feature_level: 0,
            supported_features: vec!["users".to_string()],
            supported_components: vec![],
            client_flags: vec![],
            paths: SysInfoPaths {
                emoji_url: "".to_string(),
                pfp_url: "".to_string(),
                banner_url: "".to_string(),
                graphql_endpoint: "".to_string(),
            },
        }
    }
}

#[derive(Default)]
pub struct SysInfoQuery;

#[Object]
impl SysInfoQuery {
    async fn sysInfo(&self) -> SysInfo {
        SysInfo::default()
    }
}
