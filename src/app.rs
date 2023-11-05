use crate::{
    err::{AppErr, StreamErr},
    gem_call::GemCall,
    Client, GemBytes,
};
use matchit::Router;
use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod, SslVerifyMode};
use std::{net::SocketAddr, pin::Pin, sync::Arc, time};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tokio_openssl::SslStream;
use url::Url;

/// 🖥️ A Fluffer App
///
/// ## Default behavior
///
/// * App looks for your keypair at `./key.pem` `./cert.pem`. If
///
/// * Address defaults to `127.0.0.1:1965`. You can change the address at runtime with the
///   `FLUFFER_ADDRESS` environment variable.
///
/// ```
/// App::default()
///     .route("/", |_| async { "Hello :>" })
///     .run()
/// ```
///
/// ## Custom
///
/// If you have something else in mind for any of the app's options:
///
/// ```
/// App {
///     address: "192.168.1.69:1965".to_string(),
///     ..Default::default()
/// }
/// .route("/", |_| async { "Hello :>" })
/// .run()
/// .await
/// ```
pub struct App<S = ()> {
    pub state:     S,
    pub address:   String,
    pub not_found: String,
    pub routes:    Router<Box<dyn GemCall<S> + Send + Sync>>,
    /// Path to public key
    pub key:       String,
    /// Path to certificate/private key
    pub cert:      String,
}

impl Default for App<()> {
    fn default() -> Self {
        let address = std::env::var("FLUFFER_ADDRESS").unwrap_or_else(|e| {
            warn!("❕ {e}: FLUFFER_ADDRESS. Defaulting to `127.0.0.1:1965`...");
            String::from("127.0.0.1:1965")
        });

        Self {
            state: (),
            address,
            not_found: String::from("Page not found."),
            routes: Router::default(),
            key: String::from("key.pem"),
            cert: String::from("cert.pem"),
        }
    }
}

impl<S> App<S>
where
    S: Send + Sync + Clone + 'static,
{
    /// Takes a generic function that implements [`GemCall`], and boxes it into our `routes` hashmap.
    ///
    /// May panic if an insert error occurs.
    pub fn route(mut self, path: &str, func: impl GemCall<S> + 'static + Sync + Send) -> Self {
        self.routes.insert(path, Box::new(func)).unwrap();
        self
    }

    /// Replace [`App`]'s unit state with State.
    pub fn state<T: Send + Sync + Clone>(self, state: T) -> App<T> {
        App {
            state,
            routes: Router::default(),
            address: self.address,
            not_found: self.not_found,
            cert: self.cert,
            key: self.key,
        }
    }

    /// Enter the app loop.
    ///
    /// This function returns [`AppErr`] if the inital setup
    /// fails. After that, all errors are logged as `debug`, and ignored.
    pub async fn run(self) -> Result<(), AppErr> {
        crate::interactive::gen_cert(&self.cert, &self.key)?;

        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        builder
            .set_private_key_file(&self.key, SslFiletype::PEM)
            .map_err(|e| AppErr::Key(e))?;
        builder
            .set_certificate_file(&self.cert, SslFiletype::PEM)
            .map_err(|e| AppErr::Cert(e))?;
        builder.check_private_key()?;
        builder.set_verify_callback(SslVerifyMode::PEER, |_, _| true);
        builder.set_session_id_context(
            time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)?
                .as_secs()
                .to_string()
                .as_bytes(),
        )?;

        let acceptor = builder.build();
        let listener = TcpListener::bind(&self.address)
            .await
            .map_err(|e| AppErr::Bind(e))?;

        println!("🦊 App running [{}]", self.address);

        let self_arc = Arc::new(self);

        loop {
            let Ok((stream, addr)) = listener.accept().await else {
                continue;
            };
            let Ok(ssl) = Ssl::new(acceptor.context()) else {
                continue;
            };
            let Ok(stream) = SslStream::new(ssl, stream) else {
                continue;
            };
            let self_clone = Arc::clone(&self_arc);

            tokio::spawn(async move {
                match self_clone.handle_stream(stream, addr).await {
                    Ok(_) => (),
                    Err(e) => debug!("🦊 Stream error: {e}"),
                }
            });
        }
    }

    async fn handle_stream(
        &self,
        mut stream: SslStream<TcpStream>,
        addr: SocketAddr,
    ) -> Result<(), StreamErr> {
        Pin::new(&mut stream).accept().await?;

        // 📖 Stream to bytes
        let mut read_bytes: [u8; 1026] = [0; 1026];
        let n = stream
            .read(&mut read_bytes)
            .await
            .map_err(|e| StreamErr::Read(e))?;

        // 🔗 Bytes to url
        let url = Url::parse(std::str::from_utf8(&read_bytes[..n - 2])?)
            .map_err(|e| StreamErr::UrlParse(e))?;

        // % Decode url to path
        let path = urlencoding::decode(url.path()).map_err(|e| StreamErr::UrlDecode(e))?;

        // 🔁 Get response bytes
        let response = match &self.routes.at(path.into_owned().as_str()) {
            Ok(route) => {
                info!("{addr} :: ✅🔗 Found [{url}] ({n} bytes)");

                let client = Client::new(
                    self.state.clone(),
                    url,
                    stream.ssl().peer_certificate(),
                    &route.params,
                    addr,
                );
                route.value.gem_call(client).await
            }
            Err(e) => {
                info!("{addr} :: ❌🔗 Not found [{url}] :: {e}");
                (51, &self.not_found).gem_bytes().await
            }
        };

        // 📜 Write response
        stream
            .write_all(response.as_ref())
            .await
            .map_err(|e| StreamErr::Write(e))?;

        info!("{addr} :: ✅📜 Wrote response ({} bytes)", response.len());

        Ok(())
    }
}
