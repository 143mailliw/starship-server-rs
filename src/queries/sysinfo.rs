use async_graphql::{Description, Object, SimpleObject};

#[derive(SimpleObject)]
struct SysInfoPaths {
    #[graphql(name = "emojiUrl")]
    emoji_url: String,
    #[graphql(name = "pfpUrl")]
    pfp_url: String,
    #[graphql(name = "bannerUrl")]
    banner_url: String,
    graphql_endpoint: String,
}

#[derive(SimpleObject)]
struct SysInfo {
    #[graphql(name = "serverName")]
    server_name: String,
    version: String,
    #[graphql(name = "featureLevel")]
    feature_level: u16,
    #[graphql(name = "supportedFeatures")]
    supported_features: Vec<String>,
    #[graphql(name = "supportedComponents")]
    supported_components: Vec<String>,
    #[graphql(name = "clientFlags")]
    client_flags: Vec<String>,
    paths: SysInfoPaths,
}

impl Default for SysInfo {
    fn default() -> Self {
        // TODO: Fill this data out properly
        Self {
            server_name: "starship-server-rs".to_string(),
            version: "2023.0-dev-milestone1".to_string(),
            feature_level: 1,
            supported_features: vec!["users".to_string()],
            supported_components: vec![],
            client_flags: vec![],
            paths: SysInfoPaths {
                emoji_url: String::new(),
                pfp_url: String::new(),
                banner_url: String::new(),
                graphql_endpoint: String::new(),
            },
        }
    }
}

#[derive(Default, Description)]
pub struct SysInfoQuery;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl SysInfoQuery {
    /// Retrieves information about the server.
    async fn sys_info(&self) -> SysInfo {
        SysInfo::default()
    }
}
