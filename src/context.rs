use matchit::Params;
use openssl::x509::X509;
use std::collections::HashMap;
use url::Url;

pub struct Context {
    pub url: Url,
    cert:    Option<X509>,
    params:  HashMap<String, String>,
}

impl Context {
    pub fn new(url: Url, cert: Option<X509>, params: Params<'_, '_>) -> Self {
        Self {
            url,
            cert,
            params: params
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }

    /// Returns the value of a route parameter.
    pub fn parameter(&self, key: &str) -> &str {
        if let Some(p) = self.params.get(key) {
            p.as_str()
        } else {
            panic!("Attempted to access an undefined route parameter: {key}");
        }
    }

    /// Returns optional user input.
    pub fn input(&self) -> Option<String> {
        if let Some(query) = self.url.query() {
            if let Ok(query) = urlencoding::decode(query) {
                return Some(query.into_owned());
            }
        }
        None
    }

    /// Returns the client's pem-encoded certificate.
    ///
    /// Can be used to identify clients again later.
    ///
    /// *More info: [Privacy-Enhanced Mail - Wikipedia](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail)*
    pub fn ident_get(&self) -> Option<String> {
        if let Some(cert) = &self.cert {
            match cert.to_pem() {
                Ok(bytes) => match std::str::from_utf8(&bytes) {
                    Ok(s) => return Some(s.to_string()),
                    Err(e) => debug!("📜 pem certificate isn't valid utf-8 :: {e}"),
                },
                Err(e) => debug!("📜 Failed to serialize cert into pem :: {e}"),
            }
        }
        None
    }

    /// Returns true if the certificate was signed by this client's public key.
    pub fn ident_verify(&self, other_cert: &str) -> bool {
        match X509::from_pem(other_cert.as_bytes()) {
            Ok(other_cert) => {
                if let Some(cert) = &self.cert {
                    if let Ok(other_cert) = other_cert.public_key() {
                        if let Ok(is_verified) = cert.verify(&other_cert) {
                            return is_verified;
                        }
                    }
                }
            }
            Err(e) => error!("📜 Couldn't deserialize certificate string in identity_match(): {e}"),
        }
        false
    }

    /// Returns the first `subject name` entry in the client's certificate.
    ///
    /// Good for placeholder usernames if you don't care about the possibility of bad characters.
    pub fn ident_name(&self) -> Option<String> {
        if let Some(cert) = &self.cert {
            if let Some(entry) = cert.subject_name().entries().next() {
                match entry.data().as_utf8() {
                    Ok(name) => return Some(name.to_string()),
                    Err(e) => debug!("Couldn't parse name into utf8: {e}"),
                }
            }
        }
        None
    }
}
