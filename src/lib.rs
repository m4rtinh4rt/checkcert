use std::{
    io::{self, Error, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    sync::Arc,
    time::Duration,
};

use ring::digest::{SHA256, digest};
use rustls::{ClientConfig, pki_types::ServerName};

pub struct Host {
    domain: String,
    port: u16,
    digest: String,
}

impl Host {
    pub fn new(domain: String, port: u16, digest: String) -> Host {
        Host {
            domain,
            port,
            digest,
        }
    }

    pub fn equal(&self, digest: &str) -> bool {
        self.digest == digest
    }

    pub fn host_with_port(&self) -> String {
        format!("{}:{}", self.domain, self.port)
    }

    pub fn resolve(&self) -> Result<SocketAddr, Error> {
        let mut iter = self.host_with_port().to_socket_addrs()?;
        iter.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("No addresses found for {}", self.domain),
            )
        })
    }

    pub fn hello(&self, config: ClientConfig) -> Result<String, io::Error> {
        let server_name = ServerName::try_from(self.domain.clone())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid DNS name"))?;

        let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name)
            .map_err(io::Error::other)?;

        let mut sock = TcpStream::connect_timeout(&self.resolve()?, Duration::from_secs(10))
            .map_err(io::Error::other)?;

        let mut tls = rustls::Stream::new(&mut conn, &mut sock);
        tls.write_all(b"HELLO")?;

        let certs = tls.conn.peer_certificates().unwrap();
        let cert = certs
            .first()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no host certificate found"))?;

        let sha256_digest = digest(&SHA256, cert);
        Ok(hex::encode(sha256_digest))
    }
}
