/// List of available Nanopub servers
/// Checkout the live list at https://monitor.petapico.org
pub const LIST_SERVERS: [&str; 6] = [
    "http://server.nanopubs.lod.labs.vu.nl/",
    "http://server.np.dumontierlab.com/",
    "http://app.tkuhn.eculture.labs.vu.nl/nanopub-server-1",
    "http://app.tkuhn.eculture.labs.vu.nl/nanopub-server-2",
    "http://app.tkuhn.eculture.labs.vu.nl/nanopub-server-3",
    "http://app.tkuhn.eculture.labs.vu.nl/nanopub-server-4",
];

/// Nanopub test server URL
pub const TEST_SERVER: &str = "http://test-server.nanopubs.lod.labs.vu.nl/";

pub const NP_PREF_NS: &str = "https://w3id.org/np/";
pub const NP_TEMP_URI: &str = "http://purl.org/nanopub/temp/";

pub const DEFAULT_NP_PROFILE: &str = "~/.nanopub/profile.yml";

pub const BOLD: &str = "\x1b[1;96m";
pub const END: &str = "\x1b[0m";
