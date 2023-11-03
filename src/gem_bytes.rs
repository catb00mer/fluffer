use crate::async_trait;
use std::fmt::Display;

/// Returns the gemtext (if any) from bytes in a type implementing [`GemBytes`].
#[async_trait]
pub trait ToGemtext {
    /// Returns `Some` gemtext.
    async fn to_gemtext(self) -> Option<String>;

    /// Returns type's gemtext, otherwise writes its status in a code block.
    async fn to_gemtext_err(self) -> String;
}

#[async_trait]
impl<T> ToGemtext for T
where
    T: GemBytes + Send,
{
    async fn to_gemtext(self) -> Option<String> {
        let bytes = self.gem_bytes().await;
        if let Ok(b) = std::str::from_utf8(&bytes) {
            if let Some((header, content)) = b.split_once("\r\n") {
                if let Some(pos) = header.find("20 text/gemini") {
                    if pos == 0 {
                        return Some(content.to_string());
                    }
                }
            }
        }
        None
    }

    async fn to_gemtext_err(self) -> String {
        let bytes = self.gem_bytes().await;
        if let Ok(b) = std::str::from_utf8(&bytes) {
            if let Some((header, content)) = b.split_once("\r\n") {
                if let Some(pos) = header.find("20 text/gemini") {
                    if pos == 0 {
                        return content.to_string();
                    }
                }
                return format!("```\nResponse :: {header}\n```");
            }
            return String::from("```\ninvalid gemini response\n```");
        }
        String::from("```\ninvalid utf8\n```")
    }
}

/// 💎 A trait implemented on types that can be returned as a Gemini response.
#[async_trait]
pub trait GemBytes {
    /// Return this type as a Gemini byte response.
    async fn gem_bytes(self) -> Vec<u8>;
}

/// 💎 An implementation for Result, where both Ok and Err implement GemBytes
#[async_trait]
impl<T, E> GemBytes for Result<T, E>
where
    T: GemBytes + Send,
    E: GemBytes + Send,
{
    async fn gem_bytes(self) -> Vec<u8> {
        match self {
            Ok(o) => o.gem_bytes().await,
            Err(e) => e.gem_bytes().await,
        }
    }
}

/// 💎 Returns some response, or a temporary failure if None.
#[async_trait]
impl<T> GemBytes for Option<T>
where
    T: GemBytes + Send,
{
    async fn gem_bytes(self) -> Vec<u8> {
        match self {
            Some(s) => s.gem_bytes().await,
            None => format!("40 None.\r\n").into_bytes(),
        }
    }
}

/// 💎 Tuple for composing responses.
///
/// For example: `(20, "text/gemini", "# Tuple test :o")`
#[async_trait]
impl<META, BODY> GemBytes for (u8, META, BODY)
where
    META: Display + Send,
    BODY: Display + Send,
{
    async fn gem_bytes(self) -> Vec<u8> {
        format!("{} {}\r\n{}", self.0, self.1, self.2).into_bytes()
    }
}

/// 💎 Tuple for responses only containing `status` and `meta`.
///
/// For example: `(51, "Page couldn't be found")`
#[async_trait]
impl<META> GemBytes for (u8, META)
where
    META: Display + Send,
{
    async fn gem_bytes(self) -> Vec<u8> {
        format!("{} {}\r\n", self.0, self.1).into_bytes()
    }
}

/// 💎 An implementation of GemBytes which returns `&str` as gemtext.
#[async_trait]
impl GemBytes for &str {
    async fn gem_bytes(self) -> Vec<u8> {
        format!("20 text/gemini\r\n{self}").into_bytes()
    }
}

/// 💎 An implementationg of GemBytes for an existing byte response.
#[async_trait]
impl GemBytes for Vec<u8> {
    async fn gem_bytes(self) -> Vec<u8> {
        self
    }
}

/// 💎 An implementation of GemBytes which returns `String` as gemtext.
#[async_trait]
impl GemBytes for String {
    async fn gem_bytes(self) -> Vec<u8> {
        format!("20 text/gemini\r\n{}", &self).into_bytes()
    }
}

/// 💎 Returns number as gemtext.
#[async_trait]
impl GemBytes for u32 {
    async fn gem_bytes(self) -> Vec<u8> {
        format!("20 text/gemini\r\n{self}").into_bytes()
    }
}

/// 💎 An implementation of GemBytes for a route function that returns nothing yet.
#[async_trait]
impl GemBytes for () {
    async fn gem_bytes(self) -> Vec<u8> {
        "40 This route returns nothing yet.\r\n"
            .to_string()
            .into_bytes()
    }
}

/// 💎 Returns either a temporary failure or success.
///
/// This is only truly useful for APIs, but it renders with text in a Gemini client.
#[async_trait]
impl GemBytes for bool {
    async fn gem_bytes(self) -> Vec<u8> {
        if self {
            "20 text/gemini\r\nTrue."
        } else {
            "40 False.\r\n"
        }
        .to_string()
        .into_bytes()
    }
}

/// 💎 GemBytes implementation for proxying http with [`reqwest`] without unwrapping the result.
///
/// Returns a descriptive(43 Proxy Error) if anything fails.
#[async_trait]
impl GemBytes for reqwest::Result<reqwest::Response> {
    async fn gem_bytes(self) -> Vec<u8> {
        // Catch Result errors
        let response = match self {
            Ok(o) => o,
            Err(e) => return format!("43 http error :: {}\r\n", e.without_url()).into_bytes(),
        };

        response.gem_bytes().await
    }
}

/// 💎 GemBytes implementation for proxying http with [`reqwest`].
///
/// Returns a descriptive(43 Proxy Error) if anything fails.
#[async_trait]
impl GemBytes for reqwest::Response {
    async fn gem_bytes(self) -> Vec<u8> {
        let status = self.status();
        if status != reqwest::StatusCode::OK {
            return format!("43 http: {status}\r\n").into_bytes();
        }

        let content_type = match self.headers().get("Content-Type") {
            Some(o) => o,
            None => return format!("43 http: invalid content type.\r\n").into_bytes(),
        };
        let content_type = match content_type.to_str() {
            Ok(o) => o,
            Err(_) => return format!("43 http: content type corrupted.\r\n").into_bytes(),
        };

        let mut output = format!("20 {content_type}\r\n").into_bytes();

        let mut bytes = match self.bytes().await {
            Ok(o) => o,
            Err(e) => return format!("43 http: {}\r\n", e.without_url()).into_bytes(),
        }
        .to_vec();

        output.append(&mut bytes);
        output
    }
}
