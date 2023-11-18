use std::error::Error;
use std::fmt;

use sophia::api::source::StreamError;
use sophia::inmem::index::TermIndexFullError;
use sophia::iri::InvalidIri;

#[derive(Debug)]
pub struct NpError(pub String);
// pub struct NpError {
//     message: String,
// }

impl Error for NpError {}

impl fmt::Display for NpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Add handling for sophia InvalidIri and base64 decode
impl From<InvalidIri> for NpError {
    fn from(err: InvalidIri) -> Self {
        NpError(format!("Invalid IRI error: {err}"))
    }
}
impl From<TermIndexFullError> for NpError {
    fn from(err: TermIndexFullError) -> Self {
        NpError(format!("RDF term index error: {err}"))
    }
}
impl From<StreamError<TermIndexFullError, std::io::Error>> for NpError {
    fn from(err: StreamError<TermIndexFullError, std::io::Error>) -> Self {
        NpError(format!("RDF Trig serialization error: {err}"))
    }
}
impl From<regex::Error> for NpError {
    fn from(err: regex::Error) -> Self {
        NpError(format!("Regex error: {err}"))
    }
}
impl From<std::io::Error> for NpError {
    fn from(err: std::io::Error) -> Self {
        NpError(format!("File IO error: {err}"))
    }
}
impl From<base64::DecodeError> for NpError {
    fn from(err: base64::DecodeError) -> Self {
        NpError(format!("Base64 decode error: {err}"))
    }
}
impl From<base64::alphabet::ParseAlphabetError> for NpError {
    fn from(err: base64::alphabet::ParseAlphabetError) -> Self {
        NpError(format!("Parse base64 alphabet error: {}", err))
    }
}
impl From<rsa::Error> for NpError {
    fn from(err: rsa::Error) -> Self {
        NpError(format!("RSA signing error: {err}"))
    }
}
impl From<rsa::pkcs8::spki::Error> for NpError {
    fn from(err: rsa::pkcs8::spki::Error) -> Self {
        NpError(format!("Invalid RSA public key error: {err}"))
    }
}
impl From<serde_yaml::Error> for NpError {
    fn from(err: serde_yaml::Error) -> Self {
        NpError(format!("Parse profile YAML error: {err}"))
    }
}
impl From<reqwest::Error> for NpError {
    fn from(err: reqwest::Error) -> Self {
        NpError(format!("Error sendind the HTTP request: {err}"))
    }
}
// impl From<rio_turtle::error::TurtleError> for NpError {
//     fn from(err: rio_turtle::error::TurtleError) -> Self {
//         NpError(format!("Parse RDF Turtle error: {}", err))
//     }
// }
// impl From<rsa::pkcs8::spki::Error> for NpError {
//     fn from(err: rsa::pkcs8::spki::Error) -> Self {
//         NpError(format!("Parse RSA encryption key error: {}", err))
//     }
// }

// impl From<rsa::pkcs8::Error> for NpError {
//     fn from(err: rsa::pkcs8::Error) -> Self {
//         NpError(format!("Invalid RSA encryption key error: {}", err))
//     }
// }

// impl From<GenericLightDataset<SimpleTermIndex<u32>>> for NpError {
//     fn from(err: GenericLightDataset<SimpleTermIndex<u32>>) -> Self {
//         NpError(format!("RDF term error:"))
//     }
// }
// sophia::sophia_inmem::index::TermIndexFullError

// impl From<InvalidIri> for NpError {
//     fn from(err: InvalidIri) -> Self {
//         NpError::new(&format!("Invalid IRI error: {}", err))
//     }
// }
