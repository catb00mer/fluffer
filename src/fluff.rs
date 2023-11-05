use crate::{async_trait, GemBytes};
use std::{fs::File, io::Read};

/// 🐰 The flagship implementation of GemBytes.
pub enum Fluff {
    /// (20,51): Return a file at `./static/{path}`.
    ///
    /// The file's mimetype is guessed using the [`mime_guess`] crate.
    File(String),

    /// (10): Prompt user for input
    Input(String),

    /// (51): Not found
    NotFound(String),

    // (30): Temporary redirect to path
    RedirectTemporary(String),

    // (31): Permanent redirect to path
    RedirectPermanent(String),

    // (40): Temporary failure.
    FailureTemporary(String),

    // (50): Permanent failure.
    FailurePermanent(String),

    /// (30): Redirect to ..
    GoUp,

    /// (20): A Gemtext document with a language parameter.
    Lang {
        lang: String,
        body: String,
    },

    /// (20): Return document of an explicit mimetype
    Document {
        mime: String,
        body: String,
    },

    /// (20): Return non-descript gemtext
    Text(String),

    /// (20): Wait 10 seconds, and send a test response.
    ///
    /// Only useful for debugging threads and trolling people.
    DebugWait,
}

#[async_trait]
impl GemBytes for Fluff {
    async fn gem_bytes(self) -> Vec<u8> {
        match self {
            Fluff::File(path) => {
                // Sanitize path
                let path = format!("static/{}", sanitize_filename::sanitize(path));

                // Open file
                let Ok(mut file) = File::open(&path) else {
                    return "51 File not found.\r\n".to_string().into_bytes();
                };

                // Guess mimetype
                let mimetype = match mime_guess::from_path(path).first() {
                    Some(m) => m,
                    None => {
                        return "51 File mimetype could not be guessed.\r\n"
                            .to_string()
                            .into_bytes()
                    }
                };

                // Write file bytes
                let mut v: Vec<u8> = Vec::new();
                match file.read_to_end(&mut v) {
                    Ok(_) => {
                        let mut v2 = format!("20 {mimetype}\r\n").into_bytes();
                        v2.append(&mut v);
                        v2
                    }
                    Err(e) => {
                        debug!("File read error: {e}");
                        "51 File read error.\r\n".to_string().into_bytes()
                    }
                }
            }
            Fluff::NotFound(s) => format!("51 {s}\r\n").into_bytes(),
            Fluff::Input(s) => format!("10 {s}\r\n").into_bytes(),
            Fluff::RedirectTemporary(path) => format!("30 {path}\r\n").into_bytes(),
            Fluff::RedirectPermanent(path) => format!("31 {path}\r\n").into_bytes(),
            Fluff::FailureTemporary(s) => format!("40 {s}\r\n").into_bytes(),
            Fluff::FailurePermanent(s) => format!("50 {s}\r\n").into_bytes(),
            Fluff::GoUp => "30 ..\r\n".to_string().into_bytes(),
            Fluff::Lang { lang, body } => {
                format!("20 text/gemini; lang={lang}\r\n{body}").into_bytes()
            }
            Fluff::Document { mime, body } => format!("20 {mime}\r\n{body}").into_bytes(),
            Fluff::Text(s) => format!("20 text/gemini\r\n{s}").into_bytes(),
            Fluff::DebugWait => {
                std::thread::sleep(std::time::Duration::from_secs(10));
                "20 text/gemini\r\n🧵 Waited 10 seconds!\n"
                    .to_string()
                    .into_bytes()
            }
        }
    }
}
