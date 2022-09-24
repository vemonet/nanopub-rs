
use std::fmt;

/// A nanopublication object
#[derive(Debug, Default)]
pub struct Nanopub {
    pub rdf: String,
    // pubkey
    // privkey
    // orcid
    // server_url
    // publish: bool, // false
}

impl Nanopub {
    /// Creates a new nanopub
    ///
    /// # Arguments
    ///
    /// * `rdf` - A string slice that holds the RDF of the nanopub
    ///
    /// # Examples
    ///
    /// ```
    /// use nanopub_rs::nanopub::Nanopub;
    /// let np = Nanopub::new("<http://s> <http://p> <http://o> .");
    /// ```
    pub fn new(rdf: &str) -> Self {
        // Self::default()

        Self {
            rdf: rdf.to_string()
            // rdf: if let Some(rdf) = rdf {
            //     rdf.to_string()
            // } else {
            //     "Default toast".to_string()
            // }
        }
    }
    // pub fn new(rdf: Option<&str>) -> Self {
    //     // Self::default()
    //     Self {
    //         rdf: if let Some(rdf) = rdf {
    //             rdf.to_string()
    //         } else {
    //             "Default toast".to_string()
    //         }
    //     }
    // }

    /// Returns the RDF of the nanopub
    pub fn get_rdf(&self) -> String {
        self.rdf.clone()
    }



    // /// Returns all the quads contained by the nanopub.
    // pub fn iter(&self) -> Iter<'_> {
    //     let iter = self.spog.iter();
    //     Iter {
    //         dataset: self,
    //         inner: iter,
    //     }
    // }

}


impl fmt::Display for Nanopub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for t in self {
        //     writeln!(f, "{}", t)?;
        // }
        writeln!(f, "RDF: {}", self.rdf)?;
        Ok(())
    }
}