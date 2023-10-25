use fluffer::{App, Context};

async fn name(ctx: Context) -> String {
    ctx.ident_name().unwrap_or("Who are you? 🥴".to_string())
}

async fn id(ctx: Context) -> String {
    ctx.ident_get().unwrap_or("Who are you? 🥴".to_string())
}

async fn verify(ctx: Context) -> String {
    // NOTE: To test this function, replace the certificate below with your own.
    let cert = "-----BEGIN CERTIFICATE-----
MIICmTCCAYECCDxmaR4g0RXyMA0GCSqGSIb3DQEBCwUAMA4xDDAKBgNVBAMMA093
TzAgFw0yMzEwMjUxNzIzMjdaGA85OTk5MDEwMTA2MDAwMFowDjEMMAoGA1UEAwwD
T3dPMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAnPgrliGdyfyZAWab
+QL338ztDVaLTdjhQTGAncRWrkZzmUbmKKHZBzSneQ3KyQxV745G2t/XrVxh1joB
/HFBoVTKFZ4+mHbcXK8UcA/nLRRtmnr7fMI70i54jHmPqKKI7g++pY7u8b0GiW0q
hyuFsfceqKb7PKeo57Cjq27OrU6GGyaIAQ+WKQche4uvXHY2pPdWnMnqXeA6kuxx
KbKY9BEDURZrlxTJILfS6GuG628zFre0Bzg9R3JXrm6wXFEFsrc63VDt82SJW0XD
5KDEgXzkE5NR2yh9FNyonDMcB6Z3JkJ1oZo5Ur52fdPEYxtljEJtXPG/ZhHtlVPt
fwIVjQIDAQABMA0GCSqGSIb3DQEBCwUAA4IBAQAkOKIS9Z3s5pv2wtPmNONn2gxI
JmI5/s0aCjvxcC58nBkhDoaniOfLPRfmior1PSYJO3CywsoVlLWNBPKidIKJaYcR
cMlwTCvX8Yi71dGkAKXqogAE4R1bB5+mcF9fK5EN0LzCsKxh7CLLWIGDcz2xkBoS
yfFa/hM29HqXhHvIVK0aJkn9J6DbV8UPGlasKk0mQswNNGT5mQMdKjXZGfsWkrkm
I3JmHvLxq9osKGbA3jctThPIHr324AoWWENJf33lqs8/UVxu4DTDhRlmp9g900k0
UDhrx+oupwUUcYnSaTR3gP44+IPU05mYLI6Pf3RiNP02u5ztpTpHS91nBNrx
-----END CERTIFICATE-----";

    if ctx.ident_verify(cert) {
        format!(
            "Hey, don't I recognize you? You're, {}! :D",
            ctx.ident_name().unwrap_or("[no name]".to_string())
        )
    } else {
        "I don't recognize you. Try replacing the certificate in cert.rs with your own.".to_string()
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    App::default()
        .route("/", |_| async {
            "=> /name ident_name()
=> /id ident_get()
=> /verify ident_verify(cert)"
        })
        .route("/name", name)
        .route("/id", id)
        .route("/verify", verify)
        .run()
        .await
        .unwrap();
}
