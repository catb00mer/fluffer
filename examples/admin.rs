use fluffer::{async_trait, App, Client, Fluff, GemBytes};

#[derive(thiserror::Error, Debug)]
enum CustomErr {
    #[error("Page not found.")]
    Hide,
}

#[async_trait]
impl GemBytes for CustomErr {
    async fn gem_bytes(self) -> Vec<u8> {
        let status = match self {
            CustomErr::Hide => 51,
        };

        format!("{} {}\r\n", status, self).into_bytes()
    }
}

// Uses CustomErr to hide pages if the user isn't admin.
fn hide(c: &Client) -> Result<(), CustomErr> {
    // Maybe read the certificate from a file instead.
    let admin = "-----BEGIN CERTIFICATE-----
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
    if c.ident_verify(admin) {
        Ok(())
    } else {
        Err(CustomErr::Hide)
    }
}

async fn hidden_page(c: Client) -> Result<Fluff, CustomErr> {
    hide(&c)?;

    Ok(Fluff::Text("yo wasuup? 😎".to_string()))
}

#[tokio::main]
async fn main() {
    let app = App::default()
        .route("/", |_| async {
            "=> /hidden Absolutely nothing at this url btw"
        })
        .route("/hidden", hidden_page)
        .run()
        .await;

    if let Err(e) = app {
        eprintln!("Error: {e:?}");
    }
}
