use matchit::Params;
use openssl::x509::X509;
use std::collections::HashMap;
use url::Url;

pub struct Context {
    pub url:    Url,
    pub cert:   Option<X509>,
    pub params: HashMap<String, String>,
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

    /// Returns optional user input.
    ///
    /// You *should* prompt the user if this returns `None`.
    pub fn input(&self) -> Option<String> {
        if let Some(query) = self.url.query() {
            if let Ok(query) = urlencoding::decode(query) {
                return Some(query.into_owned());
            }
        }
        None
    }

    /// Returns `subject name` field of the client's certificate.
    ///
    /// Good for placeholder usernames if you don't care about the possibility of bad characters.
    pub fn subject_name(&self) -> Option<String> {
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

    /// Returns client's pem-encoded public key.
    ///
    /// You can use this string to verify clients later.
    ///
    /// [Privacy-Enhanced Mail - Wikipedia](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail)
    pub fn pem(&self) -> Option<String> {
        if let Some(cert) = &self.cert {
            match cert.public_key() {
                Ok(pkey) => match pkey.public_key_to_pem() {
                    Ok(bytes) => match std::str::from_utf8(&bytes) {
                        Ok(s) => return Some(s.to_string()),
                        Err(e) => debug!("🔑 PEM-encoded key isn't valid utf-8 :: {e}"),
                    },
                    Err(e) => debug!("🔑 Failed to serialize key into pem :: {e}"),
                },
                Err(e) => debug!("🔑 Failed to get key :: {e}"),
            }
        }
        None
    }

    /// Returns true if the current client's public key pem matches `other_pem`.
    pub fn verify_pem(&self, other_pem: String) -> bool {
        if let Some(pem) = self.pem() {
            pem == other_pem
        } else {
            false
        }
    }
}
