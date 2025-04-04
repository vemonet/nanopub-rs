/// List of available Nanopub servers
/// Checkout the live list at https://monitor.np.trustyuri.net/
pub const LIST_SERVERS: [&str; 3] = [
    "https://registry.np.trustyuri.net/",
    "https://registry.knowledgepixels.com/",
    "https://registry.petapico.org/",
];

/// Nanopub test server URL
pub const TEST_SERVER: &str = "https://test.registry.knowledgepixels.com/";
// pub const TEST_SERVER_GRLC: &str = "https://grlc.test.nps.knowledgepixels.com/api/local/local/";

pub const NP_PREF_NS: &str = "https://w3id.org/np/";
pub const NP_TEMP_URI: &str = "http://purl.org/nanopub/temp/";
// pub const NP_TEMP_URI: &str = "http://purl.org/nanopub/temp/mynanopub#";

pub const DEFAULT_NP_PROFILE: &str = "~/.nanopub/profile.yml";

pub const BOLD: &str = "\x1b[1;96m";
pub const END: &str = "\x1b[0m";
