#[derive(thiserror::Error, Debug)]
pub enum AppErr {
    #[error("./cert.pem is missing or corrupt: {0}")]
    Cert(openssl::error::ErrorStack),

    #[error("{0}")]
    G(String),

    #[error("Error generating certificate: {0}")]
    RcGen(#[from] rcgen::RcgenError),

    #[error("Failed to save: {0}")]
    Save(#[from] std::io::Error),

    #[error("Certificate generation stopped.")]
    RcGenStop,

    #[error("./key.pem is missing or corrupt: {0}")]
    Key(openssl::error::ErrorStack),

    #[error("Fluffer failed to bind address: {0}")]
    Bind(std::io::Error),

    #[error(transparent)]
    SslStack(#[from] openssl::error::ErrorStack),

    #[error(transparent)]
    SslError(#[from] openssl::ssl::Error),

    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),
}

#[derive(thiserror::Error, Debug)]
pub enum StreamErr {
    #[error("📚 Reading stream: {0}")]
    Read(std::io::Error),

    #[error("🖌️ Writing stream: {0}")]
    Write(std::io::Error),

    #[error("🔗 Decoding url percents: {0}")]
    UrlDecode(std::string::FromUtf8Error),

    #[error("🔗 Parsing url: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    SslError(#[from] openssl::ssl::Error),
}
