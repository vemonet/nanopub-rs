/// List of available Nanopub servers
/// Checkout the live list at https://monitor.petapico.org
pub const LIST_SERVERS: [&str; 6] = [
    "https://server.nanopubs.lod.labs.vu.nl/",
    "https://server.np.dumontierlab.com/",
    "https://app.tkuhn.eculture.labs.vu.nl/nanopub-server-1",
    "https://app.tkuhn.eculture.labs.vu.nl/nanopub-server-2",
    "https://app.tkuhn.eculture.labs.vu.nl/nanopub-server-3",
    "https://app.tkuhn.eculture.labs.vu.nl/nanopub-server-4",
];

/// Nanopub test server URL
pub const TEST_SERVER: &str = "https://np.test.knowledgepixels.com/";
// pub const TEST_SERVER: &str = "http://test-server.nanopubs.lod.labs.vu.nl/";

pub const NP_PREF_NS: &str = "https://w3id.org/np/";
pub const NP_TEMP_URI: &str = "http://purl.org/nanopub/temp/";

pub const DEFAULT_NP_PROFILE: &str = "~/.nanopub/profile.yml";

pub const BOLD: &str = "\x1b[1;96m";
pub const END: &str = "\x1b[0m";
