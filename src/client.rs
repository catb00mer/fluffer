use matchit::Params;
use openssl::{asn1::Asn1Time, x509::X509};
use std::{collections::HashMap, net::SocketAddr};
use url::Url;

#[derive(Clone)]
pub struct Client<S = ()> {
    pub state: S,
    pub url:   Url,
    cert:      Option<X509>,
    params:    HashMap<String, String>,
    ip:        SocketAddr,
}

impl<S> Client<S> {
    pub fn new(
        state: S,
        url: Url,
        cert: Option<X509>,
        params: &Params<'_, '_>,
        ip: SocketAddr,
    ) -> Self {
        Self {
            state,
            url,
            cert,
            params: params
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            ip,
        }
    }

    /// Returns the value of a route parameter.
    ///
    /// **Panics:** if the parameter isn't defined.
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
                    Err(e) => debug!("Certificate pem isn't valid utf-8 :: {e}"),
                },
                Err(e) => debug!("Failed to serialize certificate :: {e}"),
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
            Err(e) => debug!("Deserializing certificate string :: {e}"),
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
                    Err(e) => debug!("Couldn't parse name into utf8 :: {e}"),
                }
            }
        }
        None
    }

    /// Returns true if the client's certificate is expired.
    ///
    /// Will also return true if there's no certificate, or if an error occurs while checking the
    /// dates.
    pub fn ident_expired(&self) -> bool {
        if let Some(cert) = &self.cert {
            match Asn1Time::days_from_now(0) {
                Ok(today) => match cert.not_after().compare(&today) {
                    Ok(cmp) => return cmp.is_le(),
                    Err(e) => debug!("📅 Comparing expiration with today :: {e}"),
                },
                Err(e) => {
                    error!("📅 Creating Asn1Time object from today's date :: {e}")
                }
            }
        }
        true
    }

    /// Return the client's ip address.
    pub fn ip(&self) -> SocketAddr {
        self.ip
    }
}
