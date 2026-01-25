use std::error::Error;
use std::fmt;

use oxrdf::IriParseError;

#[derive(Debug)]
pub struct NpError(pub String);

impl Error for NpError {}

impl fmt::Display for NpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
// InvalidNanopub
// InvalidSignature
// InvalidProfile
// ProfileMissing
// ErrorPublishing

// Add handling for errors from external dependencies
// to be able to use ? more to handle errors
impl From<IriParseError> for NpError {
    fn from(err: IriParseError) -> Self {
        NpError(format!("Invalid IRI error: {err}"))
    }
}
impl From<regex::Error> for NpError {
    fn from(err: regex::Error) -> Self {
        NpError(format!("Regex error: {err}"))
    }
}
impl From<String> for NpError {
    fn from(err: String) -> Self {
        NpError(format!("Error: {err}"))
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
        NpError(format!("Parse base64 alphabet error: {err}"))
    }
}
impl From<rsa::Error> for NpError {
    fn from(err: rsa::Error) -> Self {
        NpError(format!("RSA signing error: {err}"))
    }
}
impl From<rsa::pkcs8::Error> for NpError {
    fn from(err: rsa::pkcs8::Error) -> Self {
        NpError(format!("Invalid RSA public key error: {err}"))
    }
}
impl From<rsa::pkcs8::spki::Error> for NpError {
    fn from(err: rsa::pkcs8::spki::Error) -> Self {
        NpError(format!("Invalid RSA public key error: {err}"))
    }
}
impl From<reqwest::Error> for NpError {
    fn from(err: reqwest::Error) -> Self {
        NpError(format!("Error sendind the HTTP request: {err}"))
    }
}
impl From<std::string::FromUtf8Error> for NpError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        NpError(format!("UTF-8 conversion error: {err}"))
    }
}
impl From<oxrdfio::RdfParseError> for NpError {
    fn from(err: oxrdfio::RdfParseError) -> Self {
        NpError(format!("RDF parse error: {err}"))
    }
}

