pub struct PubspecEnvironment<'a> {
    pub sdk: Option<&'a str>,
}
pub struct Pubspec<'a> {
    pub origin: &'a serde_json::Value,
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub publish_to: Option<&'a str>,
    pub version: Option<&'a str>,
    pub environment: Option<PubspecEnvironment<'a>>,
}

impl<'a> Pubspec<'a> {
    pub fn from_pubspec(pubspec: &'a serde_json::Value) -> Self {
        Self {
            origin: pubspec,
            name: pubspec["name"].as_str(),
            description: pubspec["description"].as_str(),
            publish_to: pubspec["publish_to"].as_str(),
            version: pubspec["version"].as_str(),
            environment: match pubspec["environment"].as_object() {
                Some(env) => Some(PubspecEnvironment {
                    sdk: env.get("sdk").and_then(|sdk| sdk.as_str()),
                }),
                None => None,
            },
        }
    }
}
