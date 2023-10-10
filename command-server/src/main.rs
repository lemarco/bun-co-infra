use tokio::{io::BufStream, net::TcpListener};
use tracing::info;
mod errors;
mod req;
mod response;
pub use errors::Errors;
static DEFAULT_PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<(), Errors> {
    // Initialize the default tracing subscriber.
    tracing_subscriber::fmt::init();

    let port: u16 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_PORT.to_string())
        .parse()
        .map_err(|_| Errors::PortValueIsNotValid)?;

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    info!(
        "listening on: {}",
        listener
            .local_addr()
            .map_err(|_| Errors::CannotGetLocalAddr)?
    );

    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .map_err(|_| Errors::CannotAcceptConnection)?;
        let stream = BufStream::new(stream);

        tokio::spawn(async move {
            info!(?addr, "new connection");
            req::parse_request(stream)
                .await
                .map_err(|e| {
                    info!("Error parsing  {e}");
                })
                .map(|req| {
                    let headers = req.headers;
                    let method = req.method;
                    let path = req.path;
                    info!("Success parsing                   ");
                    info!("--------------------------------\n");
                    info!("Request.headers  = {:?}\n", headers);
                    info!("--------------------------------\n");
                    info!("Request.method    = {:?}\n", method);
                    info!("--------------------------------\n");
                    info!("Request.path        = {:?}\n", path);
                })
                .ok()
        });
    }
}
