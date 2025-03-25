/// List of available Nanopub servers
/// Checkout the live list at https://monitor.np.trustyuri.net/
pub const LIST_SERVERS: [&str; 4] = [
    "https://server.np.trustyuri.net/",
    "https://np.knowledgepixels.com/",
    "http://server.nanopubs.lod.labs.vu.nl/",
    "https://server.np.dumontierlab.com/",
];

/// Nanopub test server URL
pub const TEST_SERVER: &str = "https://np.test.knowledgepixels.com/";
// pub const TEST_SERVER_GRLC: &str = "https://grlc.test.nps.knowledgepixels.com/api/local/local/";

pub const NP_PREF_NS: &str = "https://w3id.org/np/";
pub const NP_TEMP_URI: &str = "http://purl.org/nanopub/temp/";
// pub const NP_TEMP_URI: &str = "http://purl.org/nanopub/temp/mynanopub#";

pub const DEFAULT_NP_PROFILE: &str = "~/.nanopub/profile.yml";

pub const BOLD: &str = "\x1b[1;96m";
pub const END: &str = "\x1b[0m";
