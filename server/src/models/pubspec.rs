pub struct PubspecEnvironment {
    pub sdk: Option<String>,
}
pub struct Pubspec {
    pub origin: serde_json::Value,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub issue_tracker: Option<String>,
    pub documentation: Option<String>,
    pub environment: Option<PubspecEnvironment>,
    pub publish_to: Option<String>,
    pub topics: Option<Vec<String>>,
}

impl Pubspec {
    pub fn from_pubspec(pubspec: &serde_json::Value) -> Self {
        Self {
            origin: pubspec.clone(),
            name: Some(pubspec["name"].to_string()),
            version: Some(pubspec["version"].to_string()),
            description: Some(pubspec["description"].to_string()),
            homepage: Some(pubspec["homepage"].to_string()),
            repository: Some(pubspec["repository"].to_string()),
            issue_tracker: Some(pubspec["issue_tracker"].to_string()),
            documentation: Some(pubspec["documentation"].to_string()),
            environment: match pubspec["environment"].as_object() {
                Some(env) => Some(PubspecEnvironment {
                    sdk: env.get("sdk").map(|value| value.to_string()),
                }),
                None => None,
            },
            publish_to: Some(pubspec["publish_to"].to_string()),
            topics: match pubspec["topics"].as_array() {
                Some(topics) => Some(topics.iter().map(|value| value.to_string()).collect()),
                None => None,
            },
        }
    }
}
