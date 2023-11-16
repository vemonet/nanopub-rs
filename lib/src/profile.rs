use std::{error::Error, fmt};

use crate::{
    constants::{BOLD, END},
    nanopub::NpMetadata,
};

struct Profile {
    orcid_id: String,
    name: String,
    private_key: String,
    // public_key: String,
    introduction_nanopub_uri: String,
}

impl Profile {
    pub fn new(
        orcid_id: &str,
        name: &str,
        private_key: &str,
        introduction_nanopub_uri: &str,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            orcid_id: orcid_id.to_string(),
            name: name.to_string(),
            private_key: private_key.to_string(),
            introduction_nanopub_uri: introduction_nanopub_uri.to_string(),
        })
    }

    // pub fn from_file(filepath: &str) -> Result<Self, Box<dyn Error>> {
    //     // TODO: extract from YAML file
    //     Ok(Self {
    //         orcid_id: orcid_id.to_string(),
    //         name: name.to_string(),
    //         private_key = private_key.to_string(),
    //         introduction_nanopub_uri = introduction_nanopub_uri.to_string(),
    //     })
    // }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}Nanopub Profile:{}", BOLD, END)?;
        writeln!(f, "{}ORCID:{} {}", BOLD, END, self.orcid_id)?;
        writeln!(f, "{}Name:{} {}", BOLD, END, self.name)?;
        writeln!(f, "{}Private key:{} {}", BOLD, END, self.private_key)?;
        writeln!(
            f,
            "{}Introduction URI:{} {}",
            BOLD, END, self.introduction_nanopub_uri
        )?;
        Ok(())
    }
}
