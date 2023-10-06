use std::fmt::Display;

use crate::async_trait;

/// 💎 A trait implemented on types that can be returned as a Gemini response.
#[async_trait]
pub trait GemBytes {
    /// Return this type as a Gemini byte response.
    async fn gem_bytes(self) -> Vec<u8>;
}

/// 💎 Support boxed values
#[async_trait]
impl GemBytes for Box<dyn GemBytes + Sync + Send> {
    async fn gem_bytes(self) -> Vec<u8> {
        self.gem_bytes().await
    }
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

// /// 💎 An implementation of GemBytes for vectors which already contain a Gemini byte response.
// impl GemBytes for Vec<u8> {
//     fn gem_bytes(self) -> Vec<u8> {
//         self.clone()
//     }
// }

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
