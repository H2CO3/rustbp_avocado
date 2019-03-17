use std::collections::BTreeSet;
use semver::Version;
use chrono::prelude::*;
use avocado::prelude::*;

// TODO(H2CO3): `impl Doc` manually
// TODO(H2CO3): then, `#[derive(Doc)]` instead
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crate {
    #[serde(rename = "_id")]
    pub id: Uid<Crate>,
    pub name: String,
    pub version: Version,
    pub description: Option<String>,
    pub authors: BTreeSet<Uid<User>>,
    pub categories: BTreeSet<Category>,
    pub updated_at: DateTime<FixedOffset>,
    pub downloads: usize,
    pub dependencies: BTreeSet<Uid<Crate>>,
}

impl Doc for Crate {
    const NAME: &'static str = "Crate";

    type Id = ObjectId;

    fn id(&self) -> Option<&Uid<Self>> {
        Some(&self.id)
    }

    fn set_id(&mut self, id: Uid<Self>) {
        self.id = id;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Doc)]
#[index(
    name = "example_compound_embedded_index",
    unique = true,
    keys(
        login = "ascending",
        github_profile::email = "descending",
    )
)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uid<User>,
    pub login: String,
    pub name: String,
    pub github_profile: Option<GithubProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubProfile {
    pub avatar_url: String, // should have been a `Url` but it's not Serialize
    pub email: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Algorithms,
    ApiBindings,
    Asynchronous,
    Authentication,
    Caching,
    CommandLineInterface,
    CommandLineUtilities,
    Compression,
    Concurrency,
    Config,
    Cryptography,
    Database,
    DatabaseImplementations,
    DataStructures,
    DateAndTime,
    DevelopmentTools,
    Email,
    Embedded,
    Emulators,
    Encoding,
    ExternalFfiBindings,
    Filesystem,
    GameEngines,
    Games,
    Graphics,
    Gui,
    HardwareSupport,
    Internationalization,
    Localization,
    MemoryManagement,
    Multimedia,
    NetworkProgramming,
    NoStd,
    Os,
    ParserImplementations,
    Parsing,
    Rendering,
    RustPatterns,
    Science,
    Simulation,
    TemplateEngine,
    TextEditors,
    TextProcessing,
    ValueFormatting,
    Visualization,
    Wasm,
    WebProgramming,
}
